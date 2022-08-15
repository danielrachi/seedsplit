mod cli;
mod word_list;
pub use crate::cli::*;
use crate::word_list::WORDS;
use levenshtein::levenshtein;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use std::io::{Error, ErrorKind};

pub fn run(cli: Cli) -> Result<(), Error> {
    match cli.command {
        Subcommands::Split { seedphrase } => {
            if !has_unique_elements(seedphrase.clone()) {
                println!("\x1b[93mWarning: \x1b[0m Found repeated words. Verify that all the provided words are correct.");
            }
            check_words_in_wordlist(seedphrase.clone())?;
            let seedphrase: Vec<&str> = seedphrase.iter().map(|s| &**s).collect();
            println!("\n---------------------------------------------------------------------");
            for i in 1..4 {
                let (key_a, key_b) = split(seedphrase.clone());
                print!("\x1b[32mA{}:\x1b[0m ", i);
                for word in key_a {
                    print!("{} ", word);
                }
                println!();
                print!("\x1b[92mB{}:\x1b[0m ", i);
                for word in key_b {
                    print!("{} ", word);
                }
                println!("\n---------------------------------------------------------------------");
            }
            Ok(())
        }
        Subcommands::Rebuild { key_a, key_b } => {
            if !has_unique_elements(key_a.clone()) {
                println!("\x1b[93mWarning: \x1b[0m Found repeated words. Verify that all the provided words are correct.");
            }
            if !has_unique_elements(key_b.clone()) {
                println!("\x1b[93mWarning: \x1b[0m Found repeated words. Verify that all the provided words are correct.");
            }
            check_words_in_wordlist(key_a.clone())?;
            check_words_in_wordlist(key_b.clone())?;
            if key_a.len() != key_b.len() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!(
                        "Keys must have the same number of words. Key A has {} words and key B has {} words.",
                        key_a.len(),
                        key_b.len()
                    ),
                ));
            }
            println!("\n---------------------------------------------------------------------");
            let key_a: Vec<&str> = key_a.iter().map(|s| &**s).collect();
            let key_b: Vec<&str> = key_b.iter().map(|s| &**s).collect();
            print!("\x1b[32mSeedprhase:\x1b[0m");
            for word in rebuild(key_a, key_b) {
                print!("{} ", word);
            }
            println!("\n---------------------------------------------------------------------");
            Ok(())
        }
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn get_most_similar_word(word: &str, dictionary: [&str; 2048]) -> &'static str {
    let levenshtein_distances = dictionary.iter().map(|x| levenshtein(word, x));
    let closest_word_index = levenshtein_distances
        .enumerate()
        .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(Ordering::Equal))
        .map(|(index, _)| index)
        .unwrap();
    WORDS[closest_word_index]
}

fn check_words_in_wordlist(words: Vec<String>) -> Result<(), Error> {
    for word in words {
        if !WORDS.contains(&word.as_str()) {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "Word '{}' is not in the BIP39 wordlist. Did you mean:'\x1b[32m{}\x1b[0m'?",
                    word,
                    get_most_similar_word(&word, WORDS)
                ),
            ));
        }
    }
    Ok(())
}

// this random seed is used as the key A for the algorithm
fn generate_random_seed(length: i32) -> Vec<&'static str> {
    let mut rng = &mut rand::thread_rng();
    WORDS
        .choose_multiple(&mut rng, length as usize)
        .cloned()
        .collect()
}

fn calculate_key_b_indexes(seedphrase_indexes: Vec<i32>, key_a_indexes: Vec<i32>) -> Vec<i32> {
    let mut key_b_indexes = Vec::new();
    for i in 0..seedphrase_indexes.len() {
        key_b_indexes.push((seedphrase_indexes[i] - key_a_indexes[i]).rem_euclid(2048));
    }
    key_b_indexes
}

fn words_to_indexes(words: Vec<&str>) -> Vec<i32> {
    let mut indexes: Vec<i32> = Vec::new();
    for word in words {
        indexes.push(WORDS.iter().position(|x| x == &word).unwrap() as i32);
    }
    indexes
}

fn indexes_to_words(indexes: Vec<i32>) -> Vec<&'static str> {
    let mut words = Vec::new();
    for index in indexes {
        words.push(WORDS[index as usize]);
    }
    words
}

pub fn split(seedphrase: Vec<&str>) -> (Vec<&'static str>, Vec<&'static str>) {
    let key_a = generate_random_seed(seedphrase.clone().len() as i32);
    let key_a_indexes = words_to_indexes(key_a.clone());
    let seed_indexes = words_to_indexes(seedphrase);
    let key_b_indexes = calculate_key_b_indexes(seed_indexes, key_a_indexes);
    let key_b = indexes_to_words(key_b_indexes);
    (key_a, key_b)
}

pub fn rebuild(key_a: Vec<&str>, key_b: Vec<&str>) -> Vec<&'static str> {
    let key_a_indexes = words_to_indexes(key_a.clone());
    let key_b_indexes = words_to_indexes(key_b);
    let mut rebuilt_seed_indexes = Vec::new();
    for i in 0..key_a.len() {
        rebuilt_seed_indexes.push((key_a_indexes[i] + key_b_indexes[i]).rem_euclid(2048));
    }
    indexes_to_words(rebuilt_seed_indexes)
}

#[cfg(test)]
mod tests;

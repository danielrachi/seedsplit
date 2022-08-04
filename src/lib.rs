mod word_list;
use crate::word_list::WORDS;
use clap::{builder::PossibleValuesParser, Parser, Subcommand};
use rand::seq::SliceRandom;
use std::io::{Error, ErrorKind};

pub fn run(cli: Cli) -> Result<(), Error> {
    match cli.command {
        Subcommands::Split { seedphrase } => {
            let seedphrase: Vec<&str> = seedphrase.iter().map(|s| &**s).collect();
            for i in 1..4 {
                let (key_a, key_b) = split(seedphrase.clone());
                println!("A{}: {:?} B{}: {:?}", i, key_a, i, key_b);
            }
            Ok(())
        }
        Subcommands::Rebuild { key_a, key_b } => {
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
            let key_a: Vec<&str> = key_a.iter().map(|s| &**s).collect();
            let key_b: Vec<&str> = key_b.iter().map(|s| &**s).collect();
            println!("Seedphrase: {:?}", rebuild(key_a, key_b));
            Ok(())
        }
    }
}

#[derive(Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Operation to perform
    #[clap(subcommand)]
    pub command: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Splits seedphrase
    Split {
        /// Seedprhase to split. Seedphrase must have min 12 and max 24 words.
        #[clap(value_parser = PossibleValuesParser::new(WORDS),
            required(true),
            min_values(12), max_values(24))]
        seedphrase: Vec<String>,
    },
    /// Rebuilds seedphrase from keys A and B. Keys must have the same number of words.
    Rebuild {
        /// Key A to rebuild seedphrase
        #[clap(value_parser = PossibleValuesParser::new(WORDS),
            required(true),
            min_values(12), max_values(24))]
        key_a: Vec<String>,
        /// Key B to rebuild seedphrase
        #[clap(value_parser = PossibleValuesParser::new(WORDS),
            required(true), last(true),
            min_values(12), max_values(24))]
        key_b: Vec<String>,
    },
}

// this random seed is used as the key A for the algorithm
pub fn generate_random_seed(length: i32) -> Vec<&'static str> {
    let mut rng = &mut rand::thread_rng();
    WORDS
        .choose_multiple(&mut rng, length as usize)
        .cloned()
        .collect()
}

pub fn calculate_key_b_indexes(seedphrase_indexes: Vec<i32>, key_a_indexes: Vec<i32>) -> Vec<i32> {
    let mut key_b_indexes = Vec::new();
    for i in 0..seedphrase_indexes.len() {
        key_b_indexes.push((seedphrase_indexes[i] - key_a_indexes[i]).rem_euclid(2048));
    }
    key_b_indexes
}

pub fn words_to_indexes(words: Vec<&str>) -> Vec<i32> {
    let mut indexes: Vec<i32> = Vec::new();
    for word in words {
        indexes.push(WORDS.iter().position(|x| x == &word).unwrap() as i32);
    }
    indexes
}

pub fn indexes_to_words(indexes: Vec<i32>) -> Vec<&'static str> {
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
mod tests {
    use super::*;

    #[test]
    fn test_random_seed_generation() {
        let random_seed: Vec<&str> = generate_random_seed(12);
        assert!(random_seed.iter().all(|word| WORDS.contains(word)));
        assert_eq!(random_seed.len(), 12);
        assert!(!(1..random_seed.len()).any(|i| random_seed[i..].contains(&random_seed[i - 1])));
    }

    #[test]
    fn test_key_b_indexes_calculation() {
        let seedphrase_indexes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let key_a_indexes = vec![
            2047, 2046, 2045, 2044, 2043, 2042, 2041, 2040, 2039, 2038, 2037, 2036,
        ];

        let key_b_indexes =
            calculate_key_b_indexes(seedphrase_indexes.clone(), key_a_indexes.clone());

        let mut rebuilt_seed_indexes = Vec::new();
        for i in 0..12 {
            rebuilt_seed_indexes.push((key_a_indexes[i] + key_b_indexes[i]).rem_euclid(2048));
        }

        assert_eq!(rebuilt_seed_indexes, seedphrase_indexes);
    }

    #[test]
    fn test_words_to_indexes_conversion() {
        let words = vec![
            "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
            "absurd", "abuse", "access", "accident",
        ];
        assert_eq!(
            words_to_indexes(words),
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]
        )
    }

    #[test]
    fn test_indexes_to_words_conversion() {
        let indexes = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        assert_eq!(
            indexes_to_words(indexes),
            vec![
                "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
                "absurd", "abuse", "access", "accident",
            ]
        );
    }

    #[test]
    fn test_split_and_rebuild() {
        let seedphrase = vec![
            "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract",
            "absurd", "abuse", "access", "accident",
        ];
        let (key_a, key_b): (Vec<&str>, Vec<&str>) = split(seedphrase.clone());
        let rebuilt_seed: Vec<&str> = rebuild(key_a, key_b);
        assert_eq!(seedphrase, rebuilt_seed);
    }
}

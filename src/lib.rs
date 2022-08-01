mod word_list;
use crate::word_list::WORDS;
use rand::seq::SliceRandom;

pub fn generate_random_seed(length: u8) -> Vec<&'static str> {
    let mut rng = &mut rand::thread_rng();
    WORDS.choose_multiple(&mut rng, length.into()).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_random_seed() {
        let random_seed: Vec<&str> = generate_random_seed(12);
        assert!(random_seed.iter().all(|word| WORDS.contains(word)));
        assert_eq!(random_seed.len(), 12);
        assert!(!(1..random_seed.len()).any(|i| random_seed[i..].contains(&random_seed[i-1])));
    }
}

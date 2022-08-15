
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

    let key_b_indexes = calculate_key_b_indexes(seedphrase_indexes.clone(), key_a_indexes.clone());

    let mut rebuilt_seed_indexes = Vec::new();
    for i in 0..12 {
        rebuilt_seed_indexes.push((key_a_indexes[i] + key_b_indexes[i]).rem_euclid(2048));
    }

    assert_eq!(rebuilt_seed_indexes, seedphrase_indexes);
}

#[test]
fn test_words_to_indexes_conversion() {
    let words = vec![
        "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "absurd",
        "abuse", "access", "accident",
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
        "abandon", "ability", "able", "about", "above", "absent", "absorb", "abstract", "absurd",
        "abuse", "access", "accident",
    ];
    let (key_a, key_b): (Vec<&str>, Vec<&str>) = split(seedphrase.clone());
    let rebuilt_seed: Vec<&str> = rebuild(key_a, key_b);
    assert_eq!(seedphrase, rebuilt_seed);
}

#[test]
fn all_words_in_wordlist() {
    let input = vec![
        "abandon".to_string(),
        "ability".to_string(),
        "able".to_string(),
        "about".to_string(),
        "above".to_string(),
        "absent".to_string(),
        "absorb".to_string(),
        "abstract".to_string(),
        "absurd".to_string(),
        "abuse".to_string(),
        "access".to_string(),
        "accident".to_string(),
    ];
    assert_eq!(check_words_in_wordlist(input).unwrap(), ())
}

#[test]
#[should_panic]
fn some_words_not_in_wordlist() {
    let input = vec![
        "a".to_string(),
        "ability".to_string(),
        "able".to_string(),
        "b".to_string(),
        "above".to_string(),
        "absent".to_string(),
        "absorb".to_string(),
        "abstract".to_string(),
        "d".to_string(),
        "abuse".to_string(),
        "access".to_string(),
        "accident".to_string(),
    ];
    check_words_in_wordlist(input).unwrap();
}

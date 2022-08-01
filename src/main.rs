use std::env;
use seedsplit::{split, rebuild};

fn main() {
    let args: Vec<String> = env::args().collect();
    let words:Vec<&str> = args[2..].iter().map(|x| x.as_str()).collect();
    if args[1] == "split".to_string() {
        let seed_phrase = words;
        let (key_a, key_b) = split(seed_phrase);
        println!("key A: {:?}", key_a);
        println!("key B: {:?}", key_b);
    } else {
        let mut keys = words.split(|x| x as &str == "+" as &str);
        let key_a = keys.next().unwrap().to_vec();
        let key_b = keys.next().unwrap().to_vec();
        let seedphrase = rebuild(key_a, key_b);
        println!("seedphrase: {:?}", seedphrase);
    }
}

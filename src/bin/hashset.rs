use std::collections::HashSet;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    // Number of words
    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut dictionary = HashSet::new();

    // Insert words (case-insensitive)
    for _ in 0..n {
        dictionary.insert(iter.next().unwrap().to_lowercase());
    }

    // Number of queries
    let q: usize = iter.next().unwrap().parse().unwrap();

    // Search
    for _ in 0..q {
        let word = iter.next().unwrap().to_lowercase();

        if dictionary.contains(&word) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
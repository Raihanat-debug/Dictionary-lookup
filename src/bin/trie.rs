use std::io::{self, Read};
use dictionary_lookup::Trie;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut trie = Trie::new();

    for _ in 0..n {
        trie.add(iter.next().unwrap());
    }

    let q: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let word = iter.next().unwrap();

        if trie.find(word) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
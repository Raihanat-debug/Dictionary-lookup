use std::collections::{HashMap, HashSet};

#[derive(Default)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl TrieNode {
    fn insert(&mut self, word: &str) {
        let mut current = self;

        for ch in word.chars() {
            current = current.children.entry(ch).or_default();
        }

        current.is_end = true;
    }

    fn search(&self, word: &str) -> bool {
        let mut current = self;

        for ch in word.chars() {
            match current.children.get(&ch) {
                Some(next) => current = next,
                None => return false,
            }
        }

        current.is_end
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::default(),
        }
    }

    pub fn add(&mut self, word: &str) {
        self.root.insert(&word.to_lowercase());
    }

    pub fn find(&self, word: &str) -> bool {
        self.root.search(&word.to_lowercase())
    }

    pub fn remove(&mut self, word: &str) {
        let chars: Vec<char> = word.to_lowercase().chars().collect();
        Self::remove_helper(&mut self.root, &chars, 0);
    }

    fn remove_helper(node: &mut TrieNode, chars: &[char], depth: usize) -> bool {
        if depth == chars.len() {
            if node.is_end {
                node.is_end = false;
            }
            return node.children.is_empty();
        }

        let ch = chars[depth];
        if let Some(child) = node.children.get_mut(&ch) {
            let should_remove = Self::remove_helper(child, chars, depth + 1);
            if should_remove {
                node.children.remove(&ch);
            }
        }

        !node.is_end && node.children.is_empty()
    }
}

pub struct HashSetDict {
    words: HashSet<String>,
}

impl HashSetDict {
    pub fn new() -> Self {
        Self {
            words: HashSet::new(),
        }
    }

    pub fn add(&mut self, word: &str) {
        self.words.insert(word.to_lowercase());
    }

    pub fn find(&self, word: &str) -> bool {
        self.words.contains(&word.to_lowercase())
    }

    pub fn remove(&mut self, word: &str) {
        self.words.remove(&word.to_lowercase());
    }
}

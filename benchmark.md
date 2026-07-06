# Benchmark – Dictionary Lookup

## Objective

The objective is to implement a dictionary lookup system that stores a collection of words and answers membership queries. Two different data structures are compared:

1. Trie
2. HashSet

Both implementations support the same operations:

- Insert words into the dictionary
- Search whether a word exists in the dictionary

The comparison evaluates their theoretical performance, memory usage, and practical characteristics.

---

## Algorithm 1 – Trie

A Trie (Prefix Tree) is a tree specifically designed for storing strings. Each node represents one character, and a path from the root to a terminal node represents a complete word.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Insert | O(L) |
| Search | O(L) |
| Memory | O(total characters) |

where **L** is the length of the word.

### Advantages

- Search time depends only on the word length.
- Efficient for large dictionaries.
- Naturally supports prefix searches and autocomplete.
- Shared prefixes reduce duplicate storage.

### Disadvantages

- Higher memory usage because every character is stored as a node.
- More complex implementation.

---

## Algorithm 2 – HashSet

A HashSet stores words using a hash function that maps each word to a bucket. Lookup is performed by computing the hash value and checking the corresponding bucket.

### Complexity

| Operation | Complexity |
|-----------|------------|
| Insert | O(1) average |
| Search | O(1) average |
| Memory | O(n) |

### Advantages

- Very fast average lookup.
- Simple implementation using Rust's standard library.
- Excellent for exact membership queries.
- Efficient memory usage.

### Disadvantages

- Does not preserve any ordering.
- Does not support efficient prefix searches.
- Worst-case complexity can degrade if many collisions occur.

---

## Comparison

| Feature | Trie | HashSet |
|---------|------|----------|
| Main Data Structure | Prefix Tree | Hash Table |
| Insert | O(L) | O(1) average |
| Search | O(L) | O(1) average |
| Prefix Search | Yes | No |
| Ordered | No | No |
| Memory Usage | Higher | Lower |
| Implementation Difficulty | Higher | Lower |

---

## Conclusion

Both implementations correctly solve the Dictionary Lookup problem.

The Trie is specialized for string processing and performs operations based on the length of the searched word. It is particularly useful when prefix-based operations are required.

The HashSet provides constant average-time insertion and lookup, making it an excellent choice for exact word membership queries. Its implementation is simpler and uses Rust's standard library efficiently.

For dictionary applications requiring autocomplete or prefix matching, the Trie is generally the better choice. For fast exact lookups with minimal implementation complexity, the HashSet is preferred.
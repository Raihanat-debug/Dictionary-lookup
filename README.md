# Dictionary Lookup

A Rust project that compares two dictionary lookup implementations:

- Trie
- HashSet

## Project structure

```text
Dictionary_Lookup/
├── Cargo.toml
├── benchmark.md
├── src/
│   ├── lib.rs
│   └── bin/
│       ├── trie.rs
│       └── hashset.rs
└── tests/
    ├── trie_tests.rs
    └── hashset_tests.rs
```

## Run the programs

Run the Trie version:

```bash
cargo run --bin trie
```

Run the HashSet version:

```bash
cargo run --bin hashset
```

## Run tests

```bash
cargo test
```

## Benchmark

See [benchmark.md](benchmark.md) for the comparison between Trie and HashSet.

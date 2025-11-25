# mdbook-replace

[![Build status](https://github.com/jdh8/mdbook-replace/actions/workflows/rust.yml/badge.svg)](https://github.com/jdh8/mdbook-replace)
[![Crates.io](https://img.shields.io/crates/v/mdbook-replace.svg)](https://crates.io/crates/mdbook-replace)
[![Documentation](https://docs.rs/mdbook-replace/badge.svg)](https://docs.rs/mdbook-replace)

mdBook preprocessor that simply replaces text

This preprocessor is inspired by [mdbook-abbr][abbr] and [mdbook-yapp][yapp].

[abbr]: https://crates.io/crates/mdbook-abbr
[yapp]: https://crates.io/crates/mdbook-yapp

## Installation

```bash
cargo install mdbook-replace
```

## Configuration

```toml
[preprocessor.replace]
# This enables the preprocessor

[preprocessor.replace.list]
# List the patterns and their replacements
"♠" = "<span class='♠'>♠</span>"
"♥" = "<span class='♥'>♥</span>"
"♦" = "<span class='♦'>♦</span>"
"♣" = "<span class='♣'>♣</span>"
"(R)" = "<abbr title='relay'>(R)</abbr>"
```

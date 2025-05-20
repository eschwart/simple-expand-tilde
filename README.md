# simple-expand-tilde
[![Crate](https://img.shields.io/crates/v/simple-expand-tilde.svg)](https://crates.io/crates/simple-expand-tilde)

A minimal, fast, and reliable  [tilde expansion](https://www.gnu.org/software/bash/manual/html_node/Tilde-Expansion.html) solution.

## Usage
```rust
use simple_expand_tilde::*;

fn main() {
    // Unix     =>  /home/jdoe/.rustup
    // Windows  =>  C:\Users\jdoe\.rustup
    let path = expand_tilde("~/.rustup").unwrap();
}
```
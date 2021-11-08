# oni-comb-rs (鬼昆布,おにこんぶ)

A Rust crate for LL(k) parser combinators.

[![Workflow Status](https://github.com/j5ik2o/parsing-rust/workflows/Rust/badge.svg)](https://github.com/j5ik2o/parsing-rust/actions?query=workflow%3A%22Rust%22)
[![crates.io](https://img.shields.io/crates/v/parsing-rust.svg)](https://crates.io/crates/parsing-rust)
[![docs.rs](https://docs.rs/parsing-rust/badge.svg)](https://docs.rs/parsing-rust)
[![dependency status](https://deps.rs/repo/github/j5ik2o/parsing-rust/status.svg)](https://deps.rs/repo/github/j5ik2o/parsing-rust)
[![tokei](https://tokei.rs/b1/github/j5ik2o/parsing-rust)](https://github.com/XAMPPRocky/tokei)

## Install to Cargo.toml

Add this to your `Cargo.toml`:

```toml
[dependencies]
oni-comb-rs = "<<version>>"
```

## Influenced by the following parsers implementations.

- Rust
  - [J-F-Liu/pom](https://github.com/J-F-Liu/pom)
  - [Geal/nom](https://github.com/Geal/nom)
- Scala
  - [fp in scala](https://github.com/fpinscala/fpinscala/blob/first-edition/answers/src/main/scala/fpinscala/parsing)
  - [scala-parser-combinators](https://github.com/scala/scala-parser-combinators)
- Java
  - [jparsec](https://github.com/jparsec/jparsec)

## Usage

```rust
use oni_comb_rs::core::{Parser, ParserRunner};
use oni_comb_rs::extension::parser::{CollectParser, ConversionParser};
use oni_comb_rs::prelude::*;

fn main() {
  let input: &[u8; 14] = b"'hello world';";

  let parser: Parser<u8, &str> = surround(
    elm(b'\''),
    (seq(b"hello") + elm_space() + seq(b"world")).collect(),
    elm(b'\'') + elm(b';'),
  )
  .map_res(std::str::from_utf8);
  let result: &str = parser.parse(input).unwrap();

  println!("{}", result); // hello world
}
```

## Examples

- [Hello World!](//hello_world.rs)
- [CROND](//crond.rs)
- JSON Parsers
  - [Bytes](//json_byte.rs)
  - [Characters](//json_char.rs)
- [Calculator](//calculator.rs)
- [Toys](//toys.rs)
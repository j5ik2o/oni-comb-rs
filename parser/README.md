# oni-comb-parser-rs

## WIP

## Install to Cargo.toml

Add this to your `Cargo.toml`:

```toml
[dependencies]
oni-comb-parser-rs = "<<version>>"
```

## Usage

```rust
use oni_comb_parser_rs::prelude::*;

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

## Influenced by the following parsers implementations.

- Rust
  - [J-F-Liu/pom](https://github.com/J-F-Liu/pom)
  - [Geal/nom](https://github.com/Geal/nom)
- Scala
  - [fp in scala](https://github.com/fpinscala/fpinscala/blob/first-edition/answers/src/main/scala/fpinscala/parsing)
  - [scala-parser-combinators](https://github.com/scala/scala-parser-combinators)
- Java
  - [jparsec](https://github.com/jparsec/jparsec)

## Examples

- [Hello World!](/parser/examples/hello_world.rs)
- JSON Parsers
  - [Bytes](/parser/examples/json_byte.rs)
  - [Characters](/parser/examples/json_char.rs)
- [Calculator](/parser/examples/calculator.rs)

## Alternative parsers

- [Geal/nom](https://github.com/Geal/nom)
- [J-F-Liu/pom](https://github.com/J-F-Liu/pom)
- [Marwes/combine](https://github.com/Marwes/combine)
- [zesterer/chumsky](https://github.com/zesterer/chumsky)
- [zesterer/parze](https://github.com/zesterer/parze)

# A Toys language mplementation by oni-comb-rs

[Toys](https://github.com/kmizu/toys) is a simple scripting language.


```rust
fn main() {
  let source = r#"
    define sub(i) {
      if (i > 3) {
        println(i);
      }
    }
    define main() {
      for (i in 1 to 10) {
        sub(i);
      }
    }
    "#;
  let input = source.chars().collect::<Vec<_>>();
  let result = program().parse(&input).to_result().unwrap();
  println!("{:?}", result);
  Interpreter::new().call_main(result);
}
```


## Other implementations

- https://github.com/kmizu/toys
- https://github.com/yuk1ty/toy-lang

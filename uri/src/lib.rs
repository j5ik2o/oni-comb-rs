#![warn(dead_code)]
mod expr;
pub mod models;
mod parsers;

#[cfg(test)]
mod tests {
  #[test]
  fn it_works() {
    assert_eq!(2 + 2, 4);
  }
}

#[derive(Clone, Debug, PartialEq)]
pub enum ConfigNumberValue {
  SignedLong(i64),
  UnsignedLong(u64),
  Float(f64),
}

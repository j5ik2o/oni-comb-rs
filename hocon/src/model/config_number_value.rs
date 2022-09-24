use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigNumberValue(Decimal);

impl Display for ConfigNumberValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

macro_rules! from_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for ConfigNumberValue {
          fn from(value: $t) -> Self {
            Self(Decimal::from(value))
          }
        }
    )*)
}

from_impl! { i128 u128 i64 u64 i32 u32 i16 u16 i8 u8 usize }

impl From<String> for ConfigNumberValue {
  fn from(value: String) -> Self {
    Self(Decimal::from_str_radix(&value, 10).unwrap())
  }
}

impl From<f32> for ConfigNumberValue {
  fn from(value: f32) -> Self {
    Self(Decimal::from_f32_retain(value).unwrap())
  }
}

impl From<f64> for ConfigNumberValue {
  fn from(value: f64) -> Self {
    Self(Decimal::from_f64_retain(value).unwrap())
  }
}

impl From<Decimal> for ConfigNumberValue {
  fn from(value: Decimal) -> Self {
    Self::new(value)
  }
}

impl ConfigNumberValue {
  pub fn new(value: Decimal) -> Self {
    Self(value)
  }

  pub fn as_decimal(&self) -> &Decimal {
    &self.0
  }

  pub fn to_i128(self) -> Option<i128> {
    self.0.to_i128()
  }

  pub fn to_u128(self) -> Option<u128> {
    self.0.to_u128()
  }

  pub fn to_i64(self) -> Option<i64> {
    self.0.to_i64()
  }

  pub fn to_u64(self) -> Option<u64> {
    self.0.to_u64()
  }

  pub fn to_f64(self) -> Option<f64> {
    self.0.to_f64()
  }

  pub fn to_f32(self) -> Option<f32> {
    self.0.to_f32()
  }
}

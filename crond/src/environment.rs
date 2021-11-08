pub struct Environment {
  pub(crate) now: u8,
  pub(crate) max: u8,
}

impl Environment {
  pub fn new(now: u8, max: u8) -> Self {
    Self { now, max }
  }
}

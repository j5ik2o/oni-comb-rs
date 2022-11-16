pub struct CronEnvironment {
  pub(crate) now: u8,
  pub(crate) max: u8,
}

impl CronEnvironment {
  pub fn new(now: u8, max: u8) -> Self {
    Self { now, max }
  }
}

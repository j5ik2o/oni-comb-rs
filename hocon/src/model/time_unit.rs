use std::fmt::{Display, Formatter};

#[derive(Clone, Debug, PartialEq)]
pub enum TimeUnit {
  Days,
  Hours,
  Microseconds,
  Milliseconds,
  Minutes,
  Nanoseconds,
  Seconds,
}

impl Display for TimeUnit {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let s = match self {
      TimeUnit::Days => "d",
      TimeUnit::Hours => "h",
      TimeUnit::Microseconds => "us",
      TimeUnit::Milliseconds => "ms",
      TimeUnit::Minutes => "m",
      TimeUnit::Nanoseconds => "ns",
      TimeUnit::Seconds => "s",
    };
    write!(f, "{}", s)
  }
}

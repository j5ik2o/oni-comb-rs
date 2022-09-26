use crate::model::config_number_value::ConfigNumberValue;
use crate::model::time_unit::TimeUnit;
use std::fmt::{Display, Formatter};
use std::time::Duration;

use anyhow::{bail, Result};

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigDurationValue {
  value: ConfigNumberValue,
  unit: TimeUnit,
}

impl Display for ConfigDurationValue {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} {}", self.value, self.unit)
  }
}

impl From<Duration> for ConfigDurationValue {
  fn from(value: Duration) -> Self {
    let config_number_value = ConfigNumberValue::from(value.as_nanos());
    Self::new(config_number_value, TimeUnit::Nanoseconds)
  }
}

impl From<chrono::Duration> for ConfigDurationValue {
  fn from(value: chrono::Duration) -> Self {
    let config_number_value = ConfigNumberValue::from(value.num_nanoseconds().unwrap());
    Self::new(config_number_value, TimeUnit::Nanoseconds)
  }
}

impl ConfigDurationValue {
  pub fn new(value: ConfigNumberValue, unit: TimeUnit) -> Self {
    Self { value, unit }
  }

  pub fn to_std_duration(&self) -> Result<Duration> {
    let value = match self.clone().value.to_u64() {
      Some(v) => v,
      None => bail!("Occurred convert error: {:?}", self.value),
    };
    match self.unit {
      TimeUnit::Nanoseconds => Ok(Duration::from_nanos(value)),
      TimeUnit::Microseconds => Ok(Duration::from_micros(value)),
      TimeUnit::Milliseconds => Ok(Duration::from_millis(value)),
      TimeUnit::Seconds => Ok(Duration::from_secs(value)),
      TimeUnit::Minutes => Ok(Duration::from_secs(value * 60)),
      TimeUnit::Hours => Ok(Duration::from_secs(value * 60 * 60)),
      TimeUnit::Days => Ok(Duration::from_secs(value * 60 * 60 * 24)),
    }
  }

  pub fn to_duration(&self) -> Result<chrono::Duration> {
    let value = match self.clone().value.to_i64() {
      Some(v) => v,
      None => bail!("Occurred convert error: {:?}", self.value),
    };
    match self.unit {
      TimeUnit::Nanoseconds => Ok(chrono::Duration::nanoseconds(value)),
      TimeUnit::Microseconds => Ok(chrono::Duration::microseconds(value)),
      TimeUnit::Milliseconds => Ok(chrono::Duration::milliseconds(value)),
      TimeUnit::Seconds => Ok(chrono::Duration::seconds(value)),
      TimeUnit::Minutes => Ok(chrono::Duration::minutes(value)),
      TimeUnit::Hours => Ok(chrono::Duration::hours(value)),
      TimeUnit::Days => Ok(chrono::Duration::days(value)),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::env;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::try_init();
  }

  #[test]
  fn test_to_std_duration() {
    let duration = Duration::from_secs(60);
    let cdv: ConfigDurationValue = duration.into();
    println!("{}", cdv);
    assert_eq!(cdv.to_std_duration().unwrap(), duration);
  }

  #[test]
  fn test_to_duration() {
    let duration = chrono::Duration::seconds(60);
    let cdv: ConfigDurationValue = duration.into();
    assert_eq!(cdv.to_duration().unwrap(), duration);
  }

  #[test]
  fn test_to_duration_cross() {
    let duration = chrono::Duration::days(1);
    let cdv: ConfigDurationValue = duration.into();
    assert_eq!(cdv.to_std_duration().unwrap().as_secs(), duration.num_seconds() as u64);
  }
}

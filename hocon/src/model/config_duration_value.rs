use crate::model::config_number_value::ConfigNumberValue;
use crate::model::time_unit::TimeUnit;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq)]
pub struct ConfigDurationValue {
  value: ConfigNumberValue,
  unit: TimeUnit,
}

impl ConfigDurationValue {
  pub fn new(value: ConfigNumberValue, unit: TimeUnit) -> Self {
    Self { value, unit }
  }

  pub fn to_duration(self) -> Duration {
    match self.unit {
      TimeUnit::Nanoseconds => Duration::from_nanos(self.value.to_u64().unwrap()),
      TimeUnit::Microseconds => Duration::from_micros(self.value.to_u64().unwrap()),
      TimeUnit::Milliseconds => Duration::from_millis(self.value.to_u64().unwrap()),
      TimeUnit::Seconds => Duration::from_secs(self.value.to_u64().unwrap()),
      TimeUnit::Minutes => Duration::from_secs(self.value.to_u64().unwrap() * 60),
      TimeUnit::Hours => Duration::from_secs(self.value.to_u64().unwrap() * 60 * 60),
      TimeUnit::Days => Duration::from_secs(self.value.to_u64().unwrap() * 60 * 60 * 24),
    }
  }
}

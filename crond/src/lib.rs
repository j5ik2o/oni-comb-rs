#![warn(dead_code)]
#![feature(box_patterns)]
use chrono::NaiveDate;

pub use cron_environment::*;
pub use cron_evaluator::*;
pub use cron_expr::*;
pub use cron_interval::*;
pub use cron_interval_iterator::*;
pub use cron_parser::*;
pub use cron_schedule::*;
pub use cron_specification::*;

mod cron_environment;
mod cron_evaluator;
mod cron_expr;
mod cron_interval;
mod cron_interval_iterator;
mod cron_parser;
mod cron_schedule;
mod cron_specification;

fn get_days_from_month(year: i32, month: u32) -> i64 {
  NaiveDate::from_ymd(
    match month {
      12 => year + 1,
      _ => year,
    },
    match month {
      12 => 1,
      _ => month + 1,
    },
    1,
  )
  .signed_duration_since(NaiveDate::from_ymd(year, month, 1))
  .num_days()
}

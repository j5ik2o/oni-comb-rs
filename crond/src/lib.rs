#![warn(dead_code)]
#![feature(box_patterns)]
mod environment;
mod evaluator;
mod expr;
mod parsers;

use chrono::NaiveDate;

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

#![feature(box_patterns)]
mod environment;
mod evaluator;
mod expr;

use chrono::NaiveDate;
use oni_comb_parser_rs::prelude::*;

use crate::expr::Expr;
use crate::expr::Expr::*;

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

fn min_digit<'a>() -> Parser<'a, char, Expr> {
  (elm_in('1', '5') + elm_digit())
    .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
    .attempt()
    | (elm('0') * elm_digit()).map(|e| ValueExpr(e as u8 - 48)).attempt()
    | (elm_digit()).map(|e| ValueExpr(e as u8 - 48))
}

fn hour_digit<'a>() -> Parser<'a, char, Expr> {
  (elm('2') + elm_in('0', '3'))
    .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
    .attempt()
    | (elm('1') + elm_digit())
      .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
      .attempt()
    | (elm('0') * elm_digit()).map(|e| ValueExpr(e as u8 - 48)).attempt()
    | elm_digit().map(|e| ValueExpr(e as u8 - 48)).debug("hour_digit_4")
}

fn day_digit<'a>() -> Parser<'a, char, Expr> {
  (elm('3') + elm_of("01"))
    .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
    .attempt()
    | (elm_of("12") + elm_digit())
      .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
      .attempt()
    | (elm('0') * elm_digit_1_9()).map(|e| ValueExpr(e as u8 - 48)).attempt()
    | elm_digit_1_9().map(|e| ValueExpr(e as u8 - 48))
}

fn month_digit<'a>() -> Parser<'a, char, Expr> {
  (elm('1') + elm_of("012"))
    .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
    .attempt()
    | (elm('0') * elm_digit_1_9()).map(|e| ValueExpr(e as u8 - 48)).attempt()
    | elm_digit_1_9().map(|e| ValueExpr(e as u8 - 48))
}

fn day_of_week_digit<'a>() -> Parser<'a, char, Expr> {
  tag("SUN").map(|_| ValueExpr(1)).attempt()
    | tag("MON").map(|_| ValueExpr(2)).attempt()
    | tag("TUE").map(|_| ValueExpr(3)).attempt()
    | tag("WED").map(|_| ValueExpr(4)).attempt()
    | tag("THU").map(|_| ValueExpr(5)).attempt()
    | tag("FRI").map(|_| ValueExpr(6)).attempt()
    | tag("SAT").map(|_| ValueExpr(7)).attempt()
    | elm('L').map(|_| LastValueExpr)
}

fn day_of_week_text<'a>() -> Parser<'a, char, Expr> {
  elm_in('1', '7').map(|e| ValueExpr(e as u8 - 48))
}

fn asterisk<'a>() -> Parser<'a, char, Expr> {
  elm('*').map(|_| AnyValueExpr)
}

fn per(p: Parser<char, Expr>) -> Parser<char, Expr> {
  elm('/') * p
}

fn asterisk_per(p: Parser<char, Expr>) -> Parser<char, Expr> {
  (asterisk() + per(p))
    .map(|(d, op)| PerExpr {
      digit: Box::from(d.clone()),
      option: Box::from(op.clone()),
    })
    .attempt()
}

fn range_per(p: Parser<char, Expr>) -> Parser<char, Expr> {
  per(p).opt().map(|e| match e {
    None => NoOp,
    Some(s) => s,
  })
}

fn list(p: Parser<char, Expr>) -> Parser<char, Expr> {
  p.of_many1_sep(elm(','))
    .map(|e| match e {
      e if e.len() == 1 => e.get(0).unwrap().clone(),
      e => ListExpr(e),
    })
    .attempt()
}

macro_rules! range {
  ( $x:expr ) => {
    ($x - elm('-') + $x + range_per($x))
      .map(|((e1, e2), e3)| RangeExpr {
        from: Box::from(e1),
        to: Box::from(e2),
        per_option: Box::from(e3),
      })
      .attempt()
  };
}

macro_rules! digit_instruction {
  ( $x:expr ) => {
    list(range!($x) | $x) | asterisk_per($x) | asterisk()
  };
}

fn instruction<'a>() -> Parser<'a, char, Expr> {
  (digit_instruction!(min_digit()) - elm(' ') + digit_instruction!(hour_digit()) - elm(' ')
    + digit_instruction!(day_digit())
    - elm(' ')
    + digit_instruction!(month_digit())
    - elm(' ')
    + digit_instruction!(day_of_week_text() | day_of_week_digit()))
  .map(|((((mins, hours), days), months), day_of_weeks)| CronExpr {
    mins: Box::from(mins),
    hours: Box::from(hours),
    days: Box::from(days),
    months: Box::from(months),
    day_of_weeks: Box::from(day_of_weeks),
  })
}

pub fn parse<'a>(input: &str) -> Result<Expr, String> {
  let input = input.chars().collect::<Vec<_>>();
  let x = (instruction() - end()).parse(&input).to_result();
  x.map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::evaluator::Evaluator;
  use crate::expr::Expr;
  use crate::expr::Expr::{AnyValueExpr, PerExpr, RangeExpr, ValueExpr};
  use chrono::{TimeZone, Utc};
  use std::env;

  fn init() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_instruction_1() {
    init();
    let result = parse("* * * * *").unwrap();
    assert_eq!(
      result,
      CronExpr {
        mins: Box::from(AnyValueExpr),
        hours: Box::from(AnyValueExpr),
        days: Box::from(AnyValueExpr),
        months: Box::from(AnyValueExpr),
        day_of_weeks: Box::from(AnyValueExpr)
      }
    );
  }

  #[test]
  fn test_instruction_2() {
    init();
    let result = parse("1 1 1 1 1").unwrap();
    assert_eq!(
      result,
      CronExpr {
        mins: Box::from(ValueExpr(1)),
        hours: Box::from(ValueExpr(1)),
        days: Box::from(ValueExpr(1)),
        months: Box::from(ValueExpr(1)),
        day_of_weeks: Box::from(ValueExpr(1))
      }
    );
  }

  #[test]
  fn test_digit_instruction_1() {
    init();
    let input = "*".chars().collect::<Vec<_>>();
    let result = (digit_instruction!(min_digit()) - end())
      .parse(&input)
      .to_result()
      .unwrap();
    assert_eq!(result, AnyValueExpr);
  }

  #[test]
  fn test_digit_instruction_2() {
    init();
    let input = "*/2".chars().collect::<Vec<_>>();
    let result = (digit_instruction!(min_digit()) - end())
      .parse(&input)
      .to_result()
      .unwrap();
    assert_eq!(
      result,
      PerExpr {
        digit: Box::from(AnyValueExpr),
        option: Box::from(ValueExpr(2))
      }
    );
  }

  #[test]
  fn test_digit_instruction_3() {
    init();
    let input = "1-10/2".chars().collect::<Vec<_>>();
    let result = (digit_instruction!(min_digit()) - end())
      .parse(&input)
      .to_result()
      .unwrap();
    assert_eq!(
      result,
      RangeExpr {
        from: Box::from(ValueExpr(1)),
        to: Box::from(ValueExpr(10)),
        per_option: Box::from(ValueExpr(2))
      }
    );
  }

  #[test]
  fn test_digit_instruction_4() {
    init();
    let input = "1,2,3".chars().collect::<Vec<_>>();
    let result = (digit_instruction!(min_digit()) - end())
      .parse(&input)
      .to_result()
      .unwrap();
    assert_eq!(result, ListExpr(vec![ValueExpr(1), ValueExpr(2), ValueExpr(3)]));
  }

  #[test]
  fn test_digit_instruction_5() {
    init();
    let input = "1".chars().collect::<Vec<_>>();
    let result = (digit_instruction!(min_digit()) - end())
      .parse(&input)
      .to_result()
      .unwrap();
    assert_eq!(result, ValueExpr(1));
  }

  #[test]
  fn test_list() {
    init();
    let s = (0..=59).map(|v| v.to_string()).collect::<Vec<_>>().join(",");
    let input = s.chars().collect::<Vec<_>>();
    let result = (list(min_digit()) - end()).parse(&input).to_result().unwrap();
    let values = (0..=59).map(|v| ValueExpr(v)).collect::<Vec<_>>();
    assert_eq!(result, ListExpr(values));
  }

  #[test]
  fn test_range() {
    init();
    for n2 in 1..=59 {
      let option = n2 / 2;
      let n1 = n2 - 1;
      let s: &str = &format!("{:<02}-{:<02}/{:<02}", n1, n2, option);
      let input = s.chars().collect::<Vec<_>>();
      println!("{}", s);
      let result = (range!(min_digit()) - end()).parse(&input).to_result().unwrap();
      assert_eq!(
        result,
        RangeExpr {
          from: Box::from(ValueExpr(n1)),
          to: Box::from(ValueExpr(n2)),
          per_option: Box::from(ValueExpr(option)),
        }
      );
    }
  }

  #[test]
  fn test_asterisk_per() {
    init();
    for n in 0..59 {
      let s: &str = &format!("*/{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result = (asterisk_per(min_digit()) - end()).parse(&input).to_result().unwrap();
      assert_eq!(
        result,
        PerExpr {
          digit: Box::from(AnyValueExpr),
          option: Box::from(ValueExpr(n)),
        }
      );
    }
  }

  #[test]
  fn test_per() {
    init();
    let input = "/2".chars().collect::<Vec<_>>();
    let _result = asterisk_per(min_digit()) - end();
    let result = (per(min_digit()) - end()).parse(&input).to_result().unwrap();
    assert_eq!(result, ValueExpr(2));
  }

  #[test]
  fn test_min_digit() {
    init();
    for n in 0..59 {
      let s: &str = &format!("{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result = (min_digit() - end()).parse(&input).to_result().unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let input = "60".chars().collect::<Vec<_>>();
    let result = (min_digit() - end()).parse(&input).to_result();
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_hour_digit() {
    init();
    for n in 0..=23 {
      if n < 10 {
        let s = &n.to_string();
        let input = s.chars().collect::<Vec<_>>();
        let result: Expr = (hour_digit() - end()).debug("test").parse(&input).to_result().unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result: Expr = (hour_digit() - end()).parse(&input).to_result().unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let input = "24".chars().collect::<Vec<_>>();
    let result = (hour_digit() - end()).parse(&input).to_result();
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_day_digit() {
    init();
    for n in 1..=31 {
      if n < 10 {
        let s: &str = &n.to_string();
        let input = s.chars().collect::<Vec<_>>();
        let result: Expr = (day_digit() - end()).parse(&input).to_result().unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result: Expr = (day_digit() - end()).parse(&input).to_result().unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let input = "32".chars().collect::<Vec<_>>();
    let result = (day_digit() - end()).parse(&input).to_result();
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_month_digit() {
    init();
    for n in 1..=12 {
      if n < 10 {
        let s: &str = &n.to_string();
        let input = s.chars().collect::<Vec<_>>();
        let result: Expr = (month_digit() - end()).parse(&input).to_result().unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result: Expr = (month_digit() - end()).parse(&input).to_result().unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let input = "13".chars().collect::<Vec<_>>();
    let result = (month_digit() - end()).parse(&input).to_result();
    assert_eq!(result.is_err(), true);
  }

  #[test]
  fn test_anytime() {
    let date_time = Utc.ymd(2021, 1, 1).and_hms(1, 1, 1);
    let cron_evaluator = Evaluator::new(&date_time);
    let expr = Expr::CronExpr {
      mins: Box::from(Expr::AnyValueExpr),
      hours: Box::from(Expr::AnyValueExpr),
      days: Box::from(Expr::AnyValueExpr),
      months: Box::from(Expr::AnyValueExpr),
      day_of_weeks: Box::from(Expr::AnyValueExpr),
    };
    let result = cron_evaluator.eval(&expr);
    assert!(result)
  }

  #[test]
  fn test_point_time() {
    let date_time = Utc.ymd(2021, 1, 1).and_hms(1, 1, 1);
    let cron_evaluator = Evaluator::new(&date_time);
    let expr = Expr::CronExpr {
      mins: Box::from(Expr::ValueExpr(1)),
      hours: Box::from(Expr::ValueExpr(1)),
      days: Box::from(Expr::ValueExpr(1)),
      months: Box::from(Expr::ValueExpr(1)),
      day_of_weeks: Box::from(Expr::AnyValueExpr),
    };
    let result = cron_evaluator.eval(&expr);
    assert!(result)
  }

  #[test]
  fn test_example() {
    let input = "* * * * *".chars().collect::<Vec<_>>();
    let expr = (instruction() - end()).parse(&input).to_result().unwrap();
    let date_time = Utc.ymd(2021, 1, 1).and_hms(1, 1, 1);
    let cron_evaluator = Evaluator::new(&date_time);
    let result = cron_evaluator.eval(&expr);
    assert!(result)
  }
}

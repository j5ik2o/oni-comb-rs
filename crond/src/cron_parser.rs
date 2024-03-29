use crate::cron_expr::CronExpr;
use crate::cron_expr::CronExpr::*;
use oni_comb_parser_rs::prelude::*;

fn min_digit<'a>() -> Parser<'a, char, CronExpr> {
  ((elm_in('1', '5') + elm_digit())
    .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
    .attempt()
    | (elm('0') * elm_digit()).map(|e| ValueExpr(e as u8 - 48)).attempt()
    | (elm_digit()).map(|e| ValueExpr(e as u8 - 48)))
  .cache()
}

fn hour_digit<'a>() -> Parser<'a, char, CronExpr> {
  ((elm('2') + elm_in('0', '3'))
    .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
    .attempt()
    | (elm('1') + elm_digit())
      .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
      .attempt()
    | (elm('0') * elm_digit()).map(|e| ValueExpr(e as u8 - 48)).attempt()
    | elm_digit().map(|e| ValueExpr(e as u8 - 48)).debug("hour_digit_4"))
  .cache()
}

fn day_digit<'a>() -> Parser<'a, char, CronExpr> {
  ((elm('3') + elm_of("01"))
    .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
    .attempt()
    | (elm_of("12") + elm_digit())
      .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
      .attempt()
    | (elm('0') * elm_digit_1_9()).map(|e| ValueExpr(e as u8 - 48)).attempt()
    | elm_digit_1_9().map(|e| ValueExpr(e as u8 - 48)))
  .cache()
}

fn month_digit<'a>() -> Parser<'a, char, CronExpr> {
  ((elm('1') + elm_of("012"))
    .map(|(e1, e2)| ValueExpr((e1 as u8 - 48) * 10 + e2 as u8 - 48))
    .attempt()
    | (elm('0') * elm_digit_1_9()).map(|e| ValueExpr(e as u8 - 48)).attempt()
    | elm_digit_1_9().map(|e| ValueExpr(e as u8 - 48)))
  .cache()
}

fn day_of_week_digit<'a>() -> Parser<'a, char, CronExpr> {
  (tag("SUN").map(|_| ValueExpr(1)).attempt()
    | tag("MON").map(|_| ValueExpr(2)).attempt()
    | tag("TUE").map(|_| ValueExpr(3)).attempt()
    | tag("WED").map(|_| ValueExpr(4)).attempt()
    | tag("THU").map(|_| ValueExpr(5)).attempt()
    | tag("FRI").map(|_| ValueExpr(6)).attempt()
    | tag("SAT").map(|_| ValueExpr(7)).attempt()
    | elm('L').map(|_| LastValueExpr))
  .cache()
}

fn day_of_week_text<'a>() -> Parser<'a, char, CronExpr> {
  elm_in('1', '7').map(|e| ValueExpr(e as u8 - 48)).cache()
}

fn asterisk<'a>() -> Parser<'a, char, CronExpr> {
  elm('*').map(|_| AnyValueExpr).cache()
}

fn per(p: Parser<char, CronExpr>) -> Parser<char, CronExpr> {
  elm('/') * p
}

fn asterisk_per(p: Parser<char, CronExpr>) -> Parser<char, CronExpr> {
  (asterisk() + per(p))
    .map(|(d, op)| PerExpr {
      digit: Box::from(d.clone()),
      option: Box::from(op.clone()),
    })
    .attempt()
    .cache()
}

fn range_per(p: Parser<char, CronExpr>) -> Parser<char, CronExpr> {
  per(p).opt().map(|e| match e {
    None => NoOp,
    Some(s) => s,
  })
}

fn list(p: Parser<char, CronExpr>) -> Parser<char, CronExpr> {
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

pub(crate) fn instruction<'a>() -> Parser<'a, char, CronExpr> {
  (digit_instruction!(min_digit()) - elm(' ') + digit_instruction!(hour_digit()) - elm(' ')
    + digit_instruction!(day_digit())
    - elm(' ')
    + digit_instruction!(month_digit())
    - elm(' ')
    + digit_instruction!(day_of_week_text() | day_of_week_digit()))
  .map(|((((mins, hours), days), months), day_of_weeks)| CronExpr::CronExpr {
    mins: Box::from(mins),
    hours: Box::from(hours),
    days: Box::from(days),
    months: Box::from(months),
    day_of_weeks: Box::from(day_of_weeks),
  })
}

pub struct CronParser;

impl CronParser {
  pub fn parse<'a>(input: &str) -> Result<CronExpr, String> {
    let input = input.chars().collect::<Vec<_>>();
    let x = (instruction() - end()).parse(&input).to_result();
    x.map_err(|e| e.to_string())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  use crate::cron_expr::CronExpr;
  use crate::cron_expr::CronExpr::{AnyValueExpr, PerExpr, RangeExpr, ValueExpr};

  use oni_comb_parser_rs::prelude::end;
  use std::env;

  #[ctor::ctor]
  fn init_logger() {
    env::set_var("RUST_LOG", "debug");
    let _ = env_logger::builder().is_test(true).try_init();
  }

  #[test]
  fn test_instruction_1() {
    let result = CronParser::parse("* * * * *").unwrap();
    assert_eq!(
      result,
      CronExpr::CronExpr {
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
    let result = CronParser::parse("1 1 1 1 1").unwrap();
    assert_eq!(
      result,
      CronExpr::CronExpr {
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
    let input = "*".chars().collect::<Vec<_>>();
    let result = (digit_instruction!(min_digit()) - end())
      .parse(&input)
      .to_result()
      .unwrap();
    assert_eq!(result, AnyValueExpr);
  }

  #[test]
  fn test_digit_instruction_2() {
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
    let input = "1,2,3".chars().collect::<Vec<_>>();
    let result = (digit_instruction!(min_digit()) - end())
      .parse(&input)
      .to_result()
      .unwrap();
    assert_eq!(result, ListExpr(vec![ValueExpr(1), ValueExpr(2), ValueExpr(3)]));
  }

  #[test]
  fn test_digit_instruction_5() {
    let input = "1".chars().collect::<Vec<_>>();
    let result = (digit_instruction!(min_digit()) - end())
      .parse(&input)
      .to_result()
      .unwrap();
    assert_eq!(result, ValueExpr(1));
  }

  #[test]
  fn test_list() {
    let s = (0..=59).map(|v| v.to_string()).collect::<Vec<_>>().join(",");
    let input = s.chars().collect::<Vec<_>>();
    let result = (list(min_digit()) - end()).parse(&input).to_result().unwrap();
    let values = (0..=59).map(|v| ValueExpr(v)).collect::<Vec<_>>();
    assert_eq!(result, ListExpr(values));
  }

  #[test]
  fn test_range() {
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
    let input = "/2".chars().collect::<Vec<_>>();
    let _result = asterisk_per(min_digit()) - end();
    let result = (per(min_digit()) - end()).parse(&input).to_result().unwrap();
    assert_eq!(result, ValueExpr(2));
  }

  #[test]
  fn test_min_digit() {
    for n in 0..59 {
      let s: &str = &format!("{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result = (min_digit() - end()).parse(&input).to_result().unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let input = "60".chars().collect::<Vec<_>>();
    let result = (min_digit() - end()).parse(&input).to_result();
    assert!(result.is_err());
  }

  #[test]
  fn test_hour_digit() {
    for n in 0..=23 {
      if n < 10 {
        let s = &n.to_string();
        let input = s.chars().collect::<Vec<_>>();
        let result: CronExpr = (hour_digit() - end()).debug("test").parse(&input).to_result().unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result: CronExpr = (hour_digit() - end()).parse(&input).to_result().unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let input = "24".chars().collect::<Vec<_>>();
    let result = (hour_digit() - end()).parse(&input).to_result();
    assert!(result.is_err());
  }

  #[test]
  fn test_day_digit() {
    for n in 1..=31 {
      if n < 10 {
        let s: &str = &n.to_string();
        let input = s.chars().collect::<Vec<_>>();
        let result: CronExpr = (day_digit() - end()).parse(&input).to_result().unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result: CronExpr = (day_digit() - end()).parse(&input).to_result().unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let input = "32".chars().collect::<Vec<_>>();
    let result = (day_digit() - end()).parse(&input).to_result();
    assert!(result.is_err());
  }

  #[test]
  fn test_month_digit() {
    for n in 1..=12 {
      if n < 10 {
        let s: &str = &n.to_string();
        let input = s.chars().collect::<Vec<_>>();
        let result: CronExpr = (month_digit() - end()).parse(&input).to_result().unwrap();
        assert_eq!(result, ValueExpr(n));
      }
      let s: &str = &format!("{:<02}", n);
      let input = s.chars().collect::<Vec<_>>();
      let result: CronExpr = (month_digit() - end()).parse(&input).to_result().unwrap();
      assert_eq!(result, ValueExpr(n));
    }
    let input = "13".chars().collect::<Vec<_>>();
    let result = (month_digit() - end()).parse(&input).to_result();
    assert!(result.is_err());
  }
}

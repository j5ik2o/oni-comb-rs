use oni_comb_parser::error::ParseError;
use oni_comb_parser::fail::{Fail, PResult};
use oni_comb_parser::input::Input;
use oni_comb_parser::parser::Parser;
use oni_comb_parser::parser_ext::ParserExt;
use oni_comb_parser::prelude::*;

use crate::cron_expr::CronExpr;

// --- 数値パーサー ---

fn uint8_parser<'a>() -> impl Parser<StrInput<'a>, Output = u8, Error = ParseError> {
  take_while1(|c: char| c.is_ascii_digit()).map(|s: &str| s.parse::<u8>().unwrap())
}

// --- バリデーション付き数値パーサー ---

fn ranged_uint8<'a>(
  min: u8,
  max: u8,
) -> impl Parser<StrInput<'a>, Output = u8, Error = ParseError> {
  fn_parser(move |input: &mut StrInput<'_>| {
    let pos = input.offset();
    let cp = input.checkpoint();
    let mut p = take_while1(|c: char| c.is_ascii_digit());
    let s = p.parse_next(input)?;
    match s.parse::<u8>() {
      Ok(n) if n >= min && n <= max => Ok(n),
      _ => {
        input.reset(cp);
        Err(Fail::Backtrack(ParseError::expected_description(
          pos,
          "value in range",
        )))
      }
    }
  })
}

// --- 曜日テキストパーサー ---

fn dow_text<'a>() -> impl Parser<StrInput<'a>, Output = u8, Error = ParseError> {
  tag("SUN")
    .map(|_| 1u8)
    .or(tag("MON").map(|_| 2u8))
    .or(tag("TUE").map(|_| 3u8))
    .or(tag("WED").map(|_| 4u8))
    .or(tag("THU").map(|_| 5u8))
    .or(tag("FRI").map(|_| 6u8))
    .or(tag("SAT").map(|_| 7u8))
}

// --- 式ビルダー ---

fn asterisk<'a>() -> impl Parser<StrInput<'a>, Output = CronExpr, Error = ParseError> {
  tag("*").map(|_| CronExpr::AnyValue)
}

fn any_step<'a>() -> impl Parser<StrInput<'a>, Output = CronExpr, Error = ParseError> {
  tag("*/")
    .zip_right(uint8_parser())
    .map(CronExpr::AnyStep)
}

fn step_suffix<'a>() -> impl Parser<StrInput<'a>, Output = u8, Error = ParseError> {
  tag("/").zip_right(uint8_parser())
}

fn range_expr<'a>(
  min: u8,
  max: u8,
) -> impl Parser<StrInput<'a>, Output = CronExpr, Error = ParseError> {
  fn_parser(move |input: &mut StrInput<'_>| {
    let mut val = ranged_uint8(min, max);
    let from = val.parse_next(input)?;
    let mut dash = tag("-");
    dash.parse_next(input)?;
    let to = val.parse_next(input)?;
    let mut step = step_suffix().optional();
    let s = step.parse_next(input)?;
    Ok(CronExpr::Range {
      from,
      to,
      step: s,
    })
  })
}

fn value_expr<'a>(
  min: u8,
  max: u8,
) -> impl Parser<StrInput<'a>, Output = CronExpr, Error = ParseError> {
  ranged_uint8(min, max).map(CronExpr::Value)
}

fn last_value<'a>() -> impl Parser<StrInput<'a>, Output = CronExpr, Error = ParseError> {
  tag("L").map(|_| CronExpr::LastValue)
}

// --- フィールド式 ---

fn field_expr<'a>(
  min: u8,
  max: u8,
) -> impl Parser<StrInput<'a>, Output = CronExpr, Error = ParseError> {
  fn_parser(move |input: &mut StrInput<'_>| {
    // リストは内部でカンマ区切りのアイテムを解析
    // 各アイテムは range | any_step | asterisk | value
    let item = |input: &mut StrInput<'_>| -> PResult<CronExpr, ParseError> {
      // range (N-M or N-M/S)
      if let Ok(r) = range_expr(min, max).attempt().parse_next(input) {
        return Ok(r);
      }
      // */N
      if let Ok(r) = any_step().attempt().parse_next(input) {
        return Ok(r);
      }
      // *
      if let Ok(r) = asterisk().attempt().parse_next(input) {
        return Ok(r);
      }
      // single value
      value_expr(min, max).parse_next(input)
    };

    let first = item(input)?;

    // カンマがあればリスト
    let mut items = vec![first];
    while tag(",").attempt().parse_next(input).is_ok() {
      let next = item(input)?;
      items.push(next);
    }

    if items.len() == 1 {
      Ok(items.into_iter().next().unwrap())
    } else {
      Ok(CronExpr::List(items))
    }
  })
}

fn dow_field_expr<'a>() -> impl Parser<StrInput<'a>, Output = CronExpr, Error = ParseError> {
  fn_parser(move |input: &mut StrInput<'_>| {
    let dow_item = |input: &mut StrInput<'_>| -> PResult<CronExpr, ParseError> {
      // L
      if let Ok(r) = last_value().attempt().parse_next(input) {
        return Ok(r);
      }
      // range (N-M or N-M/S)
      if let Ok(r) = range_expr(1, 7).attempt().parse_next(input) {
        return Ok(r);
      }
      // */N
      if let Ok(r) = any_step().attempt().parse_next(input) {
        return Ok(r);
      }
      // *
      if let Ok(r) = asterisk().attempt().parse_next(input) {
        return Ok(r);
      }
      // text (SUN, MON, ...)
      if let Ok(n) = dow_text().attempt().parse_next(input) {
        return Ok(CronExpr::Value(n));
      }
      // numeric 1-7
      value_expr(1, 7).parse_next(input)
    };

    let first = dow_item(input)?;

    let mut items = vec![first];
    while tag(",").attempt().parse_next(input).is_ok() {
      let next = dow_item(input)?;
      items.push(next);
    }

    if items.len() == 1 {
      Ok(items.into_iter().next().unwrap())
    } else {
      Ok(CronExpr::List(items))
    }
  })
}

// --- フル cron 式パーサー ---

fn cron_expr<'a>() -> impl Parser<StrInput<'a>, Output = CronExpr, Error = ParseError> {
  fn_parser(|input: &mut StrInput<'_>| {
    let sp = |input: &mut StrInput<'_>| -> PResult<(), ParseError> {
      take_while1(|c: char| c == ' ')
        .map(|_| ())
        .parse_next(input)
    };

    let mins = field_expr(0, 59).parse_next(input)?;
    sp(input)?;
    let hours = field_expr(0, 23).parse_next(input)?;
    sp(input)?;
    let days = field_expr(1, 31).parse_next(input)?;
    sp(input)?;
    let months = field_expr(1, 12).parse_next(input)?;
    sp(input)?;
    let dow = dow_field_expr().parse_next(input)?;

    Ok(CronExpr::Cron {
      mins: Box::new(mins),
      hours: Box::new(hours),
      days: Box::new(days),
      months: Box::new(months),
      dow: Box::new(dow),
    })
  })
}

// --- 公開 API ---

pub struct CronParser;

impl CronParser {
  pub fn parse(input: &str) -> Result<CronExpr, String> {
    let mut parser = cron_expr().zip_left(eof());
    let mut str_input = StrInput::new(input);
    parser
      .parse_next(&mut str_input)
      .map_err(|e| format!("{:?}", e))
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::cron_expr::CronExpr::*;

  #[test]
  fn parse_single_value() {
    let r = CronParser::parse("30 9 15 6 3").unwrap();
    assert_eq!(
      r,
      Cron {
        mins: Box::new(Value(30)),
        hours: Box::new(Value(9)),
        days: Box::new(Value(15)),
        months: Box::new(Value(6)),
        dow: Box::new(Value(3)),
      }
    );
  }

  #[test]
  fn parse_asterisk() {
    let r = CronParser::parse("* * * * *").unwrap();
    assert_eq!(
      r,
      Cron {
        mins: Box::new(AnyValue),
        hours: Box::new(AnyValue),
        days: Box::new(AnyValue),
        months: Box::new(AnyValue),
        dow: Box::new(AnyValue),
      }
    );
  }

  #[test]
  fn parse_any_step() {
    let r = CronParser::parse("*/5 */2 * * *").unwrap();
    match r {
      Cron { mins, hours, .. } => {
        assert_eq!(*mins, AnyStep(5));
        assert_eq!(*hours, AnyStep(2));
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn parse_range() {
    let r = CronParser::parse("0-30 9-17 * * *").unwrap();
    match r {
      Cron { mins, hours, .. } => {
        assert_eq!(
          *mins,
          Range {
            from: 0,
            to: 30,
            step: None
          }
        );
        assert_eq!(
          *hours,
          Range {
            from: 9,
            to: 17,
            step: None
          }
        );
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn parse_range_with_step() {
    let r = CronParser::parse("0-59/15 * * * *").unwrap();
    match r {
      Cron { mins, .. } => {
        assert_eq!(
          *mins,
          Range {
            from: 0,
            to: 59,
            step: Some(15)
          }
        );
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn parse_list() {
    let r = CronParser::parse("0,15,30,45 * * * *").unwrap();
    match r {
      Cron { mins, .. } => {
        assert_eq!(
          *mins,
          List(vec![Value(0), Value(15), Value(30), Value(45)])
        );
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn parse_dow_text() {
    let r = CronParser::parse("0 9 * * MON").unwrap();
    match r {
      Cron { dow, .. } => {
        assert_eq!(*dow, Value(2));
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn parse_dow_sun() {
    let r = CronParser::parse("0 0 * * SUN").unwrap();
    match r {
      Cron { dow, .. } => {
        assert_eq!(*dow, Value(1));
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn parse_last_value() {
    let r = CronParser::parse("0 0 * * L").unwrap();
    match r {
      Cron { dow, .. } => {
        assert_eq!(*dow, LastValue);
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn parse_complex_expression() {
    let r = CronParser::parse("0-59/30 0-23/2 * * *").unwrap();
    match r {
      Cron { mins, hours, .. } => {
        assert_eq!(
          *mins,
          Range {
            from: 0,
            to: 59,
            step: Some(30)
          }
        );
        assert_eq!(
          *hours,
          Range {
            from: 0,
            to: 23,
            step: Some(2)
          }
        );
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn parse_list_with_ranges() {
    let r = CronParser::parse("1-5,10-15 * * * *").unwrap();
    match r {
      Cron { mins, .. } => {
        assert_eq!(
          *mins,
          List(vec![
            Range {
              from: 1,
              to: 5,
              step: None
            },
            Range {
              from: 10,
              to: 15,
              step: None
            },
          ])
        );
      }
      _ => panic!("expected Cron"),
    }
  }

  #[test]
  fn reject_too_few_fields() {
    assert!(CronParser::parse("* * *").is_err());
  }

  #[test]
  fn reject_invalid_characters() {
    assert!(CronParser::parse("abc * * * *").is_err());
  }
}

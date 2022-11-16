#[derive(Debug, PartialEq, Clone)]
pub enum CronExpr {
  NoOp,
  ValueExpr(u8),
  LastValueExpr,
  AnyValueExpr,
  PerExpr {
    digit: Box<CronExpr>,
    option: Box<CronExpr>,
  },
  RangeExpr {
    from: Box<CronExpr>,
    to: Box<CronExpr>,
    per_option: Box<CronExpr>,
  },
  ListExpr(Vec<CronExpr>),
  CronExpr {
    mins: Box<CronExpr>,
    hours: Box<CronExpr>,
    days: Box<CronExpr>,
    months: Box<CronExpr>,
    day_of_weeks: Box<CronExpr>,
  },
}

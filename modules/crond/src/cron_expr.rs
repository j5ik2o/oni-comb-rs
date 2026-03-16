#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CronExpr {
  Value(u8),
  AnyValue,
  LastValue,
  Range {
    from: u8,
    to: u8,
    step: Option<u8>,
  },
  AnyStep(u8),
  List(Vec<CronExpr>),
  Cron {
    mins: Box<CronExpr>,
    hours: Box<CronExpr>,
    days: Box<CronExpr>,
    months: Box<CronExpr>,
    dow: Box<CronExpr>,
  },
}

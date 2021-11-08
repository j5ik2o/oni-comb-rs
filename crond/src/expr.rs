#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    NoOp,
    ValueExpr(u8),
    LastValueExpr,
    AnyValueExpr,
    PerExpr {
        digit: Box<Expr>,
        option: Box<Expr>,
    },
    RangeExpr {
        from: Box<Expr>,
        to: Box<Expr>,
        per_option: Box<Expr>,
    },
    ListExpr(Vec<Expr>),
    CronExpr {
        mins: Box<Expr>,
        hours: Box<Expr>,
        days: Box<Expr>,
        months: Box<Expr>,
        day_of_weeks: Box<Expr>,
    },
}

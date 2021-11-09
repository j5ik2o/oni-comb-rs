#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
  Mod,
  Add,            // +
  Subtract,       // -
  Multiply,       // *
  Divide,         // /
  LessThan,       // <
  LessOrEqual,    // <=
  GreaterThan,    // >
  GreaterOrEqual, // >=
  EqualEqual,     // ==
  NotEqual,       // !=
}

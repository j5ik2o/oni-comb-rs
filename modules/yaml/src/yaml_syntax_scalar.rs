/// Syntax-level scalar representation.
///
/// Preserves the quoting style without interpreting the value as
/// `bool`, `int`, `float`, or `null`. Schema interpretation is
/// deferred to the resolver layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum YamlSyntaxScalar {
  /// Unquoted scalar (e.g. `hello`, `42`, `true`).
  Plain(String),
  /// Single-quoted scalar (e.g. `'hello'`).
  SingleQuoted(String),
  /// Double-quoted scalar (e.g. `"hello\nworld"`).
  DoubleQuoted(String),
}

use crate::{CollectionStyle, YamlSyntaxScalar};

/// A single node in the YAML syntax tree.
///
/// Phase 1 variants are `Scalar`, `Sequence`, and `Mapping`.
/// Future phases will add `Tagged`, `Anchored`, and `Alias`
/// as additional variants without breaking existing code.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum YamlSyntaxNode {
  /// A scalar value with its quoting style preserved.
  Scalar(YamlSyntaxScalar),
  /// A sequence of nodes (e.g. `[a, b, c]`).
  Sequence {
    style: CollectionStyle,
    items: Vec<YamlSyntaxNode>,
  },
  /// A mapping of key-value pairs (e.g. `{a: b}`).
  Mapping {
    style: CollectionStyle,
    entries: Vec<(YamlSyntaxNode, YamlSyntaxNode)>,
  },
}

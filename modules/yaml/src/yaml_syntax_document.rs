use crate::YamlSyntaxNode;

/// A single YAML document represented as an unresolved syntax tree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YamlSyntaxDocument {
  /// The root node of this document.
  pub root: YamlSyntaxNode,
}

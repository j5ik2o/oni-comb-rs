/// Style indicator for sequences and mappings.
///
/// Phase 1 only supports `Flow`. `Block` will be added in Phase 2+.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum CollectionStyle {
  /// Flow style: `[...]` for sequences, `{...}` for mappings.
  Flow,
}

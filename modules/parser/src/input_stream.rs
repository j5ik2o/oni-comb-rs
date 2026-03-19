use crate::error::ExpectError;

pub trait InputStream {
  type Token: Copy + Eq;
  type Slice;
  type Checkpoint: Copy + Eq + Ord;
  type Error: ExpectError;

  /// 1トークン消費して返す。EOF なら None。
  fn next_token(&mut self) -> Option<Self::Token>;
  /// 次のトークンを消費せずに返す。
  fn peek_token(&self) -> Option<Self::Token>;
  /// checkpoint から現在位置までの Slice を返す。
  fn slice_since(&self, cp: Self::Checkpoint) -> Self::Slice;

  fn checkpoint(&self) -> Self::Checkpoint;
  fn reset(&mut self, checkpoint: Self::Checkpoint);
  fn offset(&self) -> usize;
  fn remaining(&self) -> Self::Slice;
  fn is_eof(&self) -> bool;

  /// 現在の行番号 (1-origin)。`\n` で区切る。
  fn line(&self) -> usize;
  /// 現在の列番号 (1-origin)。Token 単位で数える。
  fn column(&self) -> usize;
}

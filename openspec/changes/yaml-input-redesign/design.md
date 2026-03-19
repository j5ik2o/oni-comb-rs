## Context

`docs/known-issues.md` に4件の設計課題が蓄積されている。個別に対処すると中途半端な状態が残るため、一括で解決する。parser クレートの `ExpectError` 改修と yaml クレートの `YamlInput` 導入を同時に行う。

JSON パーサーは `recursive()` ベースの純粋パイプラインに書き直し済み。YAML も同じ品質にする。

## Goals / Non-Goals

**Goals:**
- `docs/known-issues.md` の4件全てを解決し、ファイルを削除する
- 全パーサーのエラーに行/列が自動的に含まれるようにする
- 全 YAML パーサーを `fn() -> impl Parser<YamlInput, ...>` 形式にする
- YAML タグ (`!!str` 等) をパース時に認識・適用する
- 公開 API を変更しない
- 全テスト通過

**Non-Goals:**
- YAML 仕様カバレッジの拡大（既存機能の書き直しのみ）
- パフォーマンス最適化（正しさと設計の一貫性を優先）
- `from_expected` の即時削除（deprecated にして段階的に移行）

## Decisions

### D1: ExpectError に from_expected_at を追加し、from_expected は deprecated にする

**選択**: 新メソッド `from_expected_at(input: &I, expected: Expected)` を追加。既存の `from_expected(position, expected)` は `#[deprecated]` にして互換維持。

**代替案**:
- (B) `from_expected` のシグネチャを直接変更 → 全コンビネータが一斉にコンパイルエラー、段階移行不可
- (C) `Input` にデフォルトメソッド `fn make_error(expected) -> Self::Error` を追加 → Input トレイトが肥大化

**理由**: deprecated 警告で段階的に移行できる。全コンビネータを一度に書き換える必要がない。

### D2: from_expected_at で Input から line/column を自動取得する

**選択**: `ParseError::from_expected_at` は `input.offset()`, `input.line()`, `input.column()` を取得して全フィールドを埋める。`fill_location_from_src` は不要になるため削除。

**理由**: エラー生成時点の正確な行/列が取れる。後付け計算の O(n) が不要。

### D3: line_start はバイト単位のまま残し、用途をドキュメント化する

**選択**: `line_start` はバイトオフセットのまま維持。「エラー時の行テキスト切り出し用」とドキュメント化。`column` (char 単位) との不整合は意図的な設計として明示。

**代替案**:
- (B) line_start を char 単位に変更 → 行テキスト切り出しにバイトオフセットが必要で逆に不便
- (C) line_start を削除 → Checkpoint が軽くなるが将来のエラー診断機能を失う

**理由**: バイト単位の line_start は `&src[line_start..offset]` で行テキストを O(1) 取得できる。

### D4: YamlInput は StrInput をラップし Input を委譲実装する

**選択**: `YamlInput<'a>` は `StrInput<'a>` をフィールドに持ち、`Input` の全メソッドを `self.inner` に委譲する。YAML 固有状態（アンカーマップ、インデントスタック）は追加フィールド。

**代替案**:
- (B) `Input` トレイトに YAML 固有メソッドを追加 → 汎用トレイトを汚染
- (C) `StrInput` を継承（Rust に継承はない）

**理由**: 委譲パターンは Rust で最も自然。parser クレートへの変更が最小限。

### D5: インデントスタックを YamlInput に持ち with_indent で操作する

**選択**: `YamlInput` に `indent_stack: Vec<usize>` を持つ。`with_indent(n, parser)` コンビネータがスタックに push し、内部パーサー実行後に pop する。

**代替案**:
- (B) Checkpoint にインデントを含めて reset で戻す → `or` の backtrack で意図せずインデントが巻き戻る
- (C) guard のみで制御 → インデントレベルの「設定」ができない

**理由**: スタックベースは再帰的ネストと1対1対応。RAII 的に pop 忘れがない。

### D6: save_anchor / resolve_alias は専用コンビネータ

**選択**: `save_anchor(parser)` は `&name` + 値パース + アンカー保存。`resolve_alias()` は `*name` からクローン返却。

**理由**: パース時点でアンカー登録が必要。専用コンビネータなら外部から純粋パーサーに見える。

### D7: with_tag コンビネータで YAML タグをパース時に認識する

**選択**: `with_tag(value_parser)` は `!tag` / `!!tag` プレフィックスを検出し、内部パーサーで値を取得した後、`apply_tag(tag, value)` で型変換する。

**理由**: `save_anchor` と同じパターン。パースパイプラインに自然に合成できる。

### D8: Checkpoint にインデントスタックは含めない

**選択**: `YamlInput` の `Checkpoint` は `StrCheckpoint` をそのまま使う。インデントは `with_indent` のスコープで管理。

**理由**: backtrack でインデントが巻き戻るとブロックパースが壊れる。

## Risks / Trade-offs

### [R1] ExpectError 変更の影響範囲が広い
→ **軽減策**: deprecated で段階移行。全コンビネータの `from_expected` → `from_expected_at` は機械的置換。

### [R2] YAML 全ファイル書き直しのリグレッション
→ **軽減策**: 既存35テストを維持。段階的に書き直し、各段階でテスト通過確認。

### [R3] uri / crond の修正漏れ
→ **軽減策**: deprecated 警告で未移行箇所が検出される。

### [R4] with_indent のパニックリスク
→ **軽減策**: `pop_indent` が空スタックの場合はデフォルト値 0 を返す。

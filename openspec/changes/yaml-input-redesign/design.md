## Context

oni-comb-yaml は oni-comb-parser 上に構築された YAML 1.2 パーサー。現在、パース状態（アンカーマップ、インデントレベル）を `ParseContext` と `min_indent: usize` として関数引数で引き回している。この設計では `Parser` トレイト (`fn parse_next(&mut self, input: &mut I) -> PResult<O, E>`) に乗らず、`sep_by0`, `recursive`, `or` 等のコンビネータが使えない。結果、`parse_next` を手動で呼び出し戻り値を捨てる手続き的コードになっている。

JSON パーサーは `recursive()` を使って純粋パイプラインスタイルで書き直し済み。YAML でも同じスタイルを実現したい。

## Goals / Non-Goals

**Goals:**
- 全 YAML パーサーを `fn() -> impl Parser<YamlInput, ...>` 形式にし、コンビネータパイプラインで記述する
- `parse_next` の直接呼び出しを公開 API (`parse`, `parse_documents`) のみに限定する
- 既存の35テストが全て通ること
- 公開 API を変更しないこと

**Non-Goals:**
- parser クレートの `Input` トレイトを変更すること（YamlInput は YAML クレート内で実装）
- パフォーマンス最適化（まず正しさと設計の一貫性を優先）
- YAML 仕様カバレッジの拡大（既存機能の書き直しのみ）

## Decisions

### D1: YamlInput は StrInput をラップし Input トレイトを委譲実装する

**選択**: `YamlInput<'a>` は `StrInput<'a>` をフィールドに持ち、`Input` の全メソッドを `self.inner` に委譲する。YAML 固有状態（アンカーマップ、インデントスタック）は追加フィールド。

**代替案**:
- (B) `Input` トレイトに YAML 固有メソッドを追加 → 汎用トレイトを汚染
- (C) `StrInput` を継承（Rust に継承はない）

**理由**: 委譲パターンは Rust で最も自然。parser クレートへの変更が不要。YamlInput 固有メソッドは `guard(|input: &YamlInput| ...)` のクロージャ内でアクセスできる。

### D2: インデントスタックを YamlInput に持ち、with_indent コンビネータで操作する

**選択**: `YamlInput` に `indent_stack: Vec<usize>` を持つ。`with_indent(n, parser)` コンビネータがスタックに push し、内部パーサー実行後に pop する。`indent_guard()` は先頭を参照して判定。

**代替案**:
- (B) Checkpoint にインデントを含めて reset で戻す → Checkpoint サイズが増え、`or` の backtrack で意図せずインデントが巻き戻る
- (C) guard のみで制御 → インデントレベルの「設定」ができない

**理由**: スタックベースはブロックスタイルの再帰的ネスト構造と1対1対応する。`with_indent` が RAII 的にスコープを管理するので、pop 忘れがない。

### D3: save_anchor は専用コンビネータとして実装する

**選択**: `save_anchor(parser)` は `&name` プレフィックスをパースし、内部パーサーで値を取得した後、`input.set_anchor(name, value.clone())` を呼ぶ。

**代替案**:
- (B) `flat_map` で実装 → `flat_map` のクロージャ内では `input` にアクセスできない
- (C) パース後の後処理で全ツリーを走査 → anchor が定義と同一ドキュメント内の後方参照で使われる場合に対応できない

**理由**: パース時点でアンカーを登録する必要がある（前方参照なし）。専用コンビネータなら `parse_next` 内で `input` を直接操作でき、外部から見ると純粋なパーサーに見える。

### D4: resolve_alias は YamlInput から値をクローンして返すパーサー

**選択**: `resolve_alias()` は `*name` をパースし、`input.get_anchor(name).cloned()` を返す。

**理由**: 単純で、他のコンビネータと自然に合成できる（`save_anchor(value_parser).or(resolve_alias()).or(...)`）。

### D5: Checkpoint は StrCheckpoint をそのまま使う（インデントスタックは含めない）

**選択**: `YamlInput` の `Checkpoint` は `StrCheckpoint` をそのまま使い、インデントスタックは `with_indent` の RAII パターンで管理する。

**理由**: `or` の backtrack でインデントレベルが巻き戻ると、ブロックパースのセマンティクスが壊れる。インデントは構造的に管理すべきで、位置ベースの Checkpoint には入れない。

## Risks / Trade-offs

### [R1] 全ファイル書き直しによるリグレッションリスク
→ **軽減策**: 既存35テストを全て維持。書き直しは段階的に行い、各段階でテスト通過を確認。

### [R2] with_indent のパニックリスク（pop 時にスタックが空）
→ **軽減策**: `pop_indent` が空スタックの場合はデフォルト値 0 を返す。テストで空スタックケースをカバー。

### [R3] Recursive + YamlInput の型推論の複雑さ
→ **軽減策**: `recursive()` は `Box<dyn Parser>` ベースなので YamlInput でも動作する。型推論が効かない場合は `fn_parser` でフォールバック。

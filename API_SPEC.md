# oni-comb-parser-rs API Specification (Extraction)

このドキュメントは、旧 `main` ブランチに存在した外部公開 API（主に `prelude` モジュール経由で提供されていた関数・型）を整理したものです。`reboot` ブランチでは実装をゼロから再構築するため、本ファイルのみを残し、仕様書として参照できるようにしました。

## 基本型

| 型 | 説明 |
| --- | --- |
| `Parser<'a, I, A>` | 入力シーケンス `&'a [I]` から値 `A` を構築するパーサー。複数のコンビネータで合成可能。 |
| `ParseResult<'a, I, A>` | パース結果。`Success { value, length }` または `Failure { error, committed_status }` を保持する。`to_result()` で `Result<A, ParseError>` に変換可能。 |
| `ParseError<'a, I>` | 失敗理由。`ParseResult::failure()` などで取得。 |
| `ParseState<'a, I>` | 内部で用いられる入力・オフセット情報。 |
| `CommittedStatus` | 失敗時に入力を消費済みかどうかを示す列挙体。 |

## `prelude` が再公開していた主な要素

### 基本パーサー

| 関数 | シグネチャ | 説明 |
| --- | --- | --- |
| `unit()` / `empty()` | `Parser<'a, I, ()>` | 常に成功し、入力を消費しない。 |
| `begin()` | `Parser<'a, I, ()>` | 入力先頭でのみ成功する。 |
| `end()` | `Parser<'a, I, ()>` | 残り入力が空のとき成功する。 |
| `successful(value)` | `Parser<'a, I, A>` | 指定した値を成功として返す。入力は消費しない。 |
| `successful_lazy(f)` | `Parser<'a, I, A>` | クロージャ `f` を呼び出して成功値を得る。 |
| `failed(error, commit)` | `Parser<'a, I, A>` | 常に失敗するパーサーを生成。コミット状態を明示。 |
| `failed_with_commit(error)` | `Parser<'a, I, A>` | コミット済み失敗。 |
| `failed_with_uncommit(error)` | `Parser<'a, I, A>` | 非コミット失敗。 |

### 主要コンビネータ（概略）

- `map(parser, f)` : パーサー成功時の値に関数 `f` を適用。
- `map_err(parser, f)` : 失敗時の `ParseError` を変換。
- `flat_map(parser, f)` : パーサー成功後に新たなパーサーへ遷移。
- `filter(parser, predicate)` : 成功値に対して述語を適用し、偽なら失敗。
- `attempt(parser)` : 成功しても入力をコミット扱いしない（バックトラック可能）。
- `exists(parser)` / `not(parser)` : 述語的な補助パーサー。
- `optional(parser)` : 非コミット失敗を `Option` に変換。
- `unwrap_or(parser, default)` / `unwrap_or_else(parser, f)` : 非コミット失敗時に既定値を返す。
- `optional(parser)` : 非コミット失敗を `Option` に変換。
- `unwrap_or(parser, default)` / `unwrap_or_else(parser, f)` : 非コミット失敗時に既定値を返す。
- `ok_or(parser, err)` / `ok_or_else(parser, f)` : `Option` を `Result` に変換。
- `expect(parser, message)` : 非コミット失敗をメッセージ付きコミット失敗へ変換。
- `chain_left1`, `chain_right1` : 左結合・右結合の演算子チェーンを構築。
- `chain_left0`, `chain_right0` : 零回許容の演算子チェーンを構築し `Option` を返す。
- `many0`, `many1`, `repeat`, `repeat_sep` : 繰り返し系。
- `skip_many0`, `skip_many1` : 結果を破棄しつつ入力を消費。
- `many_till(parser, end)` / `skip_till(parser, end)` : 終端条件付きの繰り返し。
- `skip_left`, `skip_right`, `surround` : 前後のパーサーを評価しつつ中央の値のみ返す。
- `peek(parser)` / `peek_not(parser)` : 入力を消費せずに先読み。`peek_not` は対象が失敗したときのみ成功。
- `cache(parser)` : 入力位置に対する結果をメモ化。
- `log_map`, `name`, `expect` : ログやエラーメッセージ拡張。

`choice(iter)` は `IntoIterator<Item = Parser>` を受け取り、内部で `Parser::or_list` を用いて順次論理和を評価します。

これらは `ParsersImpl` 内で実装され、`prelude` から再公開されていました。

### 文字/トークン系ユーティリティ（例）

- `elm(c)` / `elm_ref(c)` : 特定の要素を 1 つ読む。
- `seq("abc")` : 連続した要素列を読む。
- `take(n)` / `take_while0`, `take_while1` : 取り出し系。
- `regex(pattern)` : 正規表現マッチング（`regex` クレート依存）。
- `skip(n)` : `n` 文字スキップ。

## 成功・失敗結果の扱い

- `ParseResult::successful(value, length)` : 値と消費長を保持。
- `ParseResult::add_commit(is_committed)` : 既存のコミット状態に論理和で上書き。
- `ParseResult::with_uncommitted()` : コミット済み失敗を非コミット扱いに変更。
- `ParseResult::advance_success(n)` : 成功結果の消費長を加算。
- `ParseResult::flat_map`, `map`, `map_err` : 型変換やエラー変換に利用。

## 主要なモジュール

- `core` : 基本型 (`Parser`, `ParseResult`, `ParseState` など) とベース実装。
- `extension::parser::*` : ユーザ定義 DSL を構築するための構造体ラッパー。
- `extension::parsers::*` : 具体的なコンビネータ群。
- `internal::*` : 実際の実装 (`ParsersImpl`) が置かれていたモジュール。
- `utils` : 範囲ユーティリティ、集合型などの補助。

## 公開 API の設計思想（旧実装のまま）

- Scala のパーサーコンビネータ的な記述スタイルを Rust で再現することを目標としていた。
- `Parser` は値として扱え、`map` / `flat_map` / 演算子オーバーロードなどで合成可能。
- 失敗時のバックトラック制御（コミット/非コミット）を `CommittedStatus` で管理。
- JSON, URI, Cron, HOCON 等のサブクレートは、この `Parser` をベースに実装されていた。

## 今後の再実装に向けて

- API 互換性を保しつつ、内部実装はゼロから再構築する前提。
- パフォーマンス改善のため、`ParseState` はすでに残り入力を直接参照する形へ調整済み。
- `ParseResult` は今後、借用ベース (`ParseSuccess` など) へ段階的に移行予定。
- 詳細なリファクタリング計画は `docs/refactor_plan.md`（旧ブランチ）に記録されていた内容を参照。

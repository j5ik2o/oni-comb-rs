## Why

oni-comb-parser-rs を使って JSON (RFC 8259) / YAML 1.2 のフルスペックパーサーを実用ライブラリとして実装したい。しかし現在の parser クレートには、YAML のインデントベース構文に必要な行/列追跡、否定先読み、ガードコンビネータが欠けている。また API が StrInput / ByteInput で分断されており、pom ライブラリのような `sym`/`seq` のジェネリック記述や演算子オーバーロードによる直感的な文法記述ができない。parser-rs を先に整備し、その上に JSON → YAML の順で実装する。

## What Changes

### parser クレート (Phase 0: API 拡充)
- ジェネリック関数 `sym(token)`, `seq(slice)`, `any()`, `not_a(pred)` を追加。StrInput/ByteInput 両方で同一関数を使用可能に
- 新コンビネータ `not(parser)`, `peek(parser)`, `repeat(n..m)`, `collect()`, `discard()`, `position()` を追加
- `float()` パーサー (RFC 8259 準拠の f64) を text モジュールに追加
- 演算子オーバーロード: `+` (zip), `-` (zip_left), `*` (zip_right), `|` (or), `!` (not), 単項`-` (peek), `>>` (flat_map) を Parser トレイト実装者に blanket impl
- `quoted_string` の `\uXXXX` サロゲートペア対応を検証・修正

### parser クレート (Phase 1: Input 拡張)
- **BREAKING**: `Input` トレイトに `line() -> usize`, `column() -> usize` メソッドを追加
- **BREAKING**: `StrInput`/`ByteInput` の `Checkpoint` を `usize` から構造体 (offset + line + column + line_start) に変更。`Ord` は offset で比較
- `StrInput`: column は char (codepoint) 単位。`ByteInput`: column は byte 単位。いずれも `\n` で行を区切り
- `guard(pred)` コンビネータ追加 (`Fn(&I) -> bool`)
- `ParseError` に line/column フィールドを追加しエラー報告を改善

### JSON クレート (Phase 2)
- `modules/json` クレートを新規作成。RFC 8259 フルスペック準拠の JSON パーサー
- Phase 0/1 の成果 (sym/seq、演算子、float、行/列エラー) をフル活用した宣言的実装

### YAML クレート (Phase 3)
- `modules/yaml` クレートを新規作成。YAML 1.2 フルスペック準拠
- guard + line/column でインデントベースのブロック構造をパース
- フロースタイル / ブロックスタイル / マルチライン文字列 / アンカー・エイリアス / タグ対応

## Capabilities

### New Capabilities
- `generic-parsers`: ジェネリック関数 (sym/seq/any/not_a) および演算子オーバーロードによる入力型非依存のパーサー記述
- `lookahead-combinators`: not/peek/guard コンビネータによる先読み・条件付きパース
- `line-column-tracking`: Input トレイトの行/列追跡、Checkpoint 拡張、ParseError の行/列対応
- `repeat-and-collect`: repeat(n..m) 回数指定繰り返し、collect (Slice 返却)、discard、position
- `float-parser`: RFC 8259 準拠の浮動小数点数パーサー
- `json-parser`: RFC 8259 フルスペック JSON パーサークレート
- `yaml-parser`: YAML 1.2 フルスペックパーサークレート

### Modified Capabilities
<!-- なし。既存の公開 API は互換維持しつつ拡張する。Input トレイトへのメソッド追加と Checkpoint 型変更は BREAKING だが、外部実装者はほぼいない想定 -->

## Impact

- **parser クレート**: Input トレイト、StrInput、ByteInput、ParseError に破壊的変更あり。全既存テスト・ベンチマークの修正が必要
- **uri / crond クレート**: parser クレートに依存しているため、Checkpoint 型変更・Input トレイト変更への対応が必要
- **Cargo workspace**: `modules/json`, `modules/yaml` メンバー追加
- **パフォーマンス**: next_token に \n チェック分岐が追加される。分岐予測で吸収可能と予想するが、ベンチマークで検証が必要

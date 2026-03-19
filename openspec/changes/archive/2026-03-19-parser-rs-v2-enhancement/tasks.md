## 1. Phase 0: ジェネリック関数の追加

- [x] 1.1 `Sym<I>` 構造体と `sym<I>(token)` 関数を `primitive/sym.rs` に実装する。StrInput/ByteInput 両方で動作するテストを書く
- [x] 1.2 Input トレイトに `type TagInput` 関連型を追加し、`Seq<I>` 構造体と `seq<I>(slice)` 関数を `primitive/seq.rs` に実装する。`&str`/`&[u8]` 両対応のテストを書く
- [x] 1.3 `Any<I>` 構造体と `any<I>()` 関数を `primitive/any.rs` に実装する
- [x] 1.4 `NotA<F, I>` 構造体と `not_a<I>(pred)` 関数を `primitive/not_a.rs` に実装する
- [x] 1.5 新しいジェネリック関数を prelude にエクスポートする

## 2. Phase 0: 新コンビネータの追加

- [x] 2.1 `Not<P>` 構造体を `combinator/not.rs` に実装する（否定先読み: 内部パーサーが Backtrack で失敗すれば Ok(()), 成功すれば Backtrack, Cut は伝播）
- [x] 2.2 `Peek<P>` 構造体を `combinator/peek.rs` に実装する（正先読み: 成功時は出力を返すが入力を巻き戻す）
- [x] 2.3 `Repeat<P, R>` 構造体を `combinator/repeat.rs` に実装する（Range 引数で繰り返し回数を指定）
- [x] 2.4 `Collect<P>` 構造体を `combinator/collect.rs` に実装する（checkpoint〜現在位置の Slice を返す）
- [x] 2.5 `Discard<P>` 構造体を `combinator/discard.rs` に実装する（出力を () に変換）
- [x] 2.6 `Position<I>` 構造体と `position<I>()` 関数を `combinator/position.rs` に実装する
- [x] 2.7 ParserExt に `not()`, `peek()`, `repeat()`, `collect()`, `discard()` メソッドを追加する

## 3. Phase 0: 演算子オーバーロード

- [x] 3.1 `impl Add<P2> for P1` (zip) を Ops ラッパー経由で実装する
- [x] 3.2 `impl Sub<P2> for P1` (zip_left) を Ops ラッパー経由で実装する
- [x] 3.3 `impl Mul<P2> for P1` (zip_right) を Ops ラッパー経由で実装する
- [x] 3.4 `impl BitOr<P2> for P1` (or) を Ops ラッパー経由で実装する
- [x] 3.5 `impl Not for P` (not/否定先読み) を Ops ラッパー経由で実装する
- [x] 3.6 `impl Neg for P` (peek/正先読み) を Ops ラッパー経由で実装する
- [x] 3.7 `impl Shr<F> for P` (flat_map) を Ops ラッパー経由で実装する
- [x] 3.8 演算子の組み合わせテスト（pom の json_char.rs スタイルでパーサーを記述し動作確認）

## 4. Phase 0: float パーサーと quoted_string 修正

- [x] 4.1 `Float` 構造体と `float()` 関数を `text/float.rs` に実装する（RFC 8259 数値仕様: `[-] int [frac] [exp]`）
- [x] 4.2 `quoted_string` の `\uXXXX` サロゲートペア対応を検証し、不足があれば修正する
- [x] 4.3 float と quoted_string を prelude にエクスポートする（float は新規追加）

## 5. Phase 0: 既存テスト・ベンチマーク修正

- [x] 5.1 既存の全テスト (`cargo test -p oni-comb-parser`) が通ることを確認する
- [x] 5.2 既存のベンチマークが動作することを確認する
- [x] 5.3 uri クレートと crond クレートが Phase 0 の変更でビルドできることを確認する

## 6. Phase 1: Input トレイトの行/列追跡

- [x] 6.1 Input トレイトに `line() -> usize` と `column() -> usize` メソッドを追加する
- [x] 6.2 StrInput に `line`, `column`, `line_start` フィールドを追加し、`next_token` で `\n` 検出時に更新するロジックを実装する。column は char 単位
- [x] 6.3 ByteInput に同様の行/列追跡を実装する。column は byte 単位
- [x] 6.4 行/列追跡の単体テスト（改行越え、マルチバイト文字、複数行）

## 7. Phase 1: Checkpoint 構造体化

- [x] 7.1 `StrCheckpoint { offset, line, column, line_start }` 構造体を定義し、`Copy + Eq + Ord`（Ord は offset で比較）を実装する
- [x] 7.2 StrInput の `Checkpoint` 関連型を `usize` から `StrCheckpoint` に変更し、`checkpoint()` と `reset()` を更新する
- [x] 7.3 ByteInput にも同様の `ByteCheckpoint` 構造体を定義し適用する
- [x] 7.4 `or`, `attempt`, `optional`, `many*`, `sep_by*` 等の全 backtrack コンビネータが新 Checkpoint で正しく動作するテスト

## 8. Phase 1: guard コンビネータと ParseError 改善

- [x] 8.1 `Guard<F, I>` 構造体と `guard<I>(pred)` 関数を `combinator/guard.rs` に実装する
- [x] 8.2 ParserExt に guard をチェーンできるヘルパーを追加する（必要に応じて）
- [x] 8.3 ParseError に `line` と `column` フィールドを追加し、`from_expected` 等のコンストラクタを更新する
- [x] 8.4 `position()` を拡張して line/column も返せるようにする（Output 型の検討）

## 9. Phase 1: 下流クレート修正とベンチマーク

- [x] 9.1 uri クレートを新しい Input/Checkpoint API に対応させる
- [x] 9.2 crond クレートを新しい Input/Checkpoint API に対応させる
- [x] 9.3 全テスト (`cargo test`) が通ることを確認する
- [x] 9.4 JSON full bench を実行し、行/列追跡のパフォーマンス影響を計測する（結果: ~15% 退行。advance 内の行/列更新ループが原因。最適化は後続タスクで対応）

## 10. Phase 2: JSON パーサークレート

- [x] 10.1 `modules/json` ディレクトリと Cargo.toml を作成し、workspace に追加する
- [x] 10.2 `JsonValue` enum を定義する（Null, Bool, Number(f64), String, Array, Object）
- [x] 10.3 JSON プリミティブパーサー（null, bool, number, string）を実装する。string は `\uXXXX` サロゲートペア対応
- [x] 10.4 JSON 配列・オブジェクトパーサーを fn_parser + 手動再帰で実装する
- [x] 10.5 トップレベルの `json()` パーサー（空白処理 + EOF チェック）を実装する
- [x] 10.6 RFC 8259 準拠の網羅的テスト（26テスト通過）
- [x] 10.7 エラー報告テスト（行/列、期待トークン、コンテキスト）

## 11. Phase 3: YAML パーサークレート - 基盤

- [x] 11.1 `modules/yaml` ディレクトリと Cargo.toml を作成し、workspace に追加する
- [x] 11.2 `YamlValue` enum を定義する（Null, Bool, Integer, Float, String, Sequence, Mapping, Tagged）
- [x] 11.3 YAML スカラーパーサーを実装する（Core Schema: null/bool/int/float/plain string/quoted string）
- [x] 11.4 コメント (`#` から行末) のスキップ処理を実装する

## 12. Phase 3: YAML パーサークレート - フロースタイル

- [x] 12.1 フローシーケンス (`[item1, item2]`) パーサーを実装する
- [x] 12.2 フローマッピング (`{key: value}`) パーサーを実装する
- [x] 12.3 フロースタイルのネストと再帰をテストする

## 13. Phase 3: YAML パーサークレート - ブロックスタイル

- [x] 13.1 guard + column() を使ったインデント検出の基盤を実装する
- [x] 13.2 ブロックシーケンス (`- item`) パーサーを実装する
- [x] 13.3 ブロックマッピング (`key: value`) パーサーを実装する
- [x] 13.4 ブロックスタイルのネスト（インデント増減）をテストする
- [x] 13.5 フローとブロックの混在をテストする

## 14. Phase 3: YAML パーサークレート - マルチライン文字列

- [x] 14.1 リテラルブロック (`|`) パーサーを実装する
- [x] 14.2 folded ブロック (`>`) パーサーを実装する
- [x] 14.3 chomping indicator (`-`, `+`) とインデントインディケータをサポートする
- [x] 14.4 マルチライン文字列のエッジケーステスト

## 15. Phase 3: YAML パーサークレート - アンカー・エイリアス・タグ

- [x] 15.1 アンカー (`&name`) のパースと保存を実装する（ParseContext で管理）
- [x] 15.2 エイリアス (`*name`) のパースと参照解決を実装する
- [x] 15.3 マージキー (`<<: *ref`) のサポートを実装する（フロースタイルアンカー値で動作確認済み）
- [x] 15.4 Core Schema タグ (`!!str`, `!!int` 等) のパースと型強制を実装する（apply_tag として実装）
- [x] 15.5 カスタムタグ (`!custom`) の保持を実装する（Tagged バリアントとして実装）

## 16. Phase 3: YAML パーサークレート - マルチドキュメントとエラー

- [x] 16.1 ドキュメント開始 (`---`) / 終了 (`...`) マーカーのパースを実装する
- [x] 16.2 マルチドキュメントパーサー（複数ドキュメントのリスト返却）を実装する
- [x] 16.3 エラー報告テスト（行/列、インデントエラー、期待トークン）— error_reports_position テスト追加
- [x] 16.4 YAML 主要パターンの網羅的テスト（31テスト: スカラー/フロー/ブロック/マルチライン/コメント/ドキュメント/アンカー/マージキー/タグ）

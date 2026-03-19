## Context

oni-comb-parser-rs は Rust 製パーサーコンビネータライブラリ。trait + 具象コンビネータ型で構成され、動的ディスパッチ・ヒープ確保を排した設計。現在 StrInput (char) と ByteInput (u8) の2入力型をサポートし、Applicative/Alternative 主体の API を提供する。

現状の制約:
- テキスト専用パーサー (`char`, `tag`) と汎用パーサー (`satisfy`, `take_while`) が分離しており、`sym('a')` / `sym(b'a')` のようなジェネリック記述ができない
- 否定先読み (not)、正先読み (peek)、入力状態の条件判定 (guard) がない
- Input が byte offset のみ追跡し、行/列情報がない。YAML のインデント構文やエラー報告に必要
- 演算子オーバーロードがなく、pom のような `a + b | c * d` 形式の宣言的文法記述ができない

## Goals / Non-Goals

**Goals:**
- parser クレートの API を pom スタイルのジェネリック関数・演算子で記述可能にする
- Input トレイトに行/列追跡を追加し、YAML のインデント構文パースとエラー報告を可能にする
- JSON (RFC 8259) と YAML 1.2 のフルスペックパーサーを別クレートとして実装する
- 既存のパフォーマンス特性（winnow に次ぐスループット）を維持する

**Non-Goals:**
- serde 統合（将来の拡張として残す）
- ストリーミングパース（Fail::Incomplete は予約済みだが今回は実装しない）
- no_std core-only 層の feature gate 分離（今回は alloc 前提）
- パーサーの左再帰サポート / Packrat パース
- YAML のカスタムタグスキーマ（Core Schema のみ対応）

## Decisions

### D1: Input トレイトに line()/column() を直接追加する

**選択**: Input トレイトにメソッドを追加し、StrInput/ByteInput 両方で実装する。

**代替案**:
- (B) StrInput にだけ追加（トレイト外メソッド）→ guard のシグネチャが StrInput 固定になり汎用性が下がる
- (C) 外部ラッパー LocatedInput<I> → コンビネータとの統合が煩雑、二重抽象のコスト
- (D) 2パス方式（レキサー→パーサー）→ YAML 実装が複雑化、パフォーマンス懸念

**理由**: 全パーサーで line/column にアクセスでき、guard コンビネータがジェネリックに書ける。ByteInput でも `\n` 区切りで行追跡すればテキストプロトコル (HTTP, SMTP) で有用。

### D2: column は Token 単位で数える

**選択**: StrInput は char (codepoint) 単位、ByteInput は byte 単位。

**代替案**:
- バイトオフセット単位 → YAML 1.2 仕様が文字単位で列を定義しているため不適合
- grapheme cluster 単位 → 複雑で、unicode-segmentation クレート依存が必要

**理由**: `next_token` が1回呼ばれるたびに column +1 するだけなので、Token 型の定義と一致し実装がシンプル。YAML 1.2 仕様にも準拠。

### D3: Checkpoint を構造体に拡張する

**選択**: `StrCheckpoint { offset, line, column, line_start }` 構造体。`Ord` は offset のみで比較。

**代替案**:
- reset 時にオフセットから再計算 → O(n) で backtrack 頻度が高い YAML では致命的
- Checkpoint は usize のまま、line/column は reset で捨てる → or/attempt の後で行/列がずれる

**理由**: Copy + Eq + Ord を満たしつつ O(1) の reset を維持。32 バイトのスタックコピーは許容範囲。

### D4: 演算子オーバーロードは blanket impl で実現する

**選択**: `impl<I, P1, P2> Add<P2> for P1 where P1: Parser<I>, P2: Parser<I>` の形で blanket impl。

**代替案**:
- Ops ラッパー型 → `Ops(sym('a')) + Ops(sym('b'))` が煩雑
- マクロベースの DSL → デバッグ困難、エラーメッセージ悪化

**理由**: Parser トレイトを実装する全ての具象型に自動的に演算子が使える。既存のメソッドチェーン (.zip, .or 等) と共存可能。orphan rule の問題はない（全具象型が oni-comb クレート内で定義されるため）。

### D5: sym/seq はジェネリック関数として追加し、既存の char/tag は残す

**選択**: `sym<I: Input>(token: I::Token)` と `seq<I: Input>(slice: ???)` を追加。`char()`, `tag()` は StrInput 固定のショートカットとして維持。

**理由**: 戻り値型から Input 型が推論される場面では turbofish 不要でクリーンに書ける。テキストパーサーでは `char('x')`, `tag("hello")` のほうが意図が明確。

### D6: seq のスライス型は Input トレイトに関連型を追加して対応する

**選択**: `Input` に `type TagInput` (比較対象のスライス型) を追加。StrInput では `&str`、ByteInput では `&[u8]`。

**理由**: `seq` が `seq("hello")` (StrInput) / `seq(b"hello")` (ByteInput) の両方で動作するためには、Input 型からスライスリテラルの型を導出する必要がある。

### D7: Phase 順序は 0→1→2→3 の直列

**選択**: parser-rs を完全に整備 (Phase 0+1) してから JSON (Phase 2) → YAML (Phase 3) の順で実装。

**代替案**:
- Phase 0 → Phase 2 (JSON) → Phase 1 → Phase 3 (YAML) → JSON は先に動くが行/列エラーなし
- Phase 0 → Phase 1 → Phase 2+3 統合 (JSON は YAML のサブセット) → 大きすぎて YAML 完成まで何も使えない

**理由**: parser-rs の整備が終わった状態で JSON を書くと、行/列エラー報告を含む完全な JSON パーサーが一発で書ける。JSON 実装で API の使い勝手を検証してから YAML に進める。

### D8: not/peek の Fail 伝播セマンティクス

**選択**:
- `not(p)`: p が Backtrack → `Ok(())`, p が成功 → `Err(Backtrack)`, p が Cut → `Err(Cut)` 伝播
- `peek(p)`: 成功時は checkpoint に巻き戻して `Ok(output)`, 失敗時はそのまま伝播

**理由**: Cut の意味（コミット済み、backtrack 禁止）を尊重する。not/peek が Cut を握りつぶすと、エラー回復の意図が壊れる。

## Risks / Trade-offs

### [R1] next_token の \n チェックによるパフォーマンス退行
→ **軽減策**: 分岐予測で吸収可能と予想。Phase 1 完了後に既存ベンチマークで計測し、JSON full bench で 5% 以上の退行があれば対策を検討（例: ASCII fast path で改行チェックを分岐なしに最適化）

### [R2] Checkpoint サイズ増加 (8→32 バイト) による backtrack コスト増
→ **軽減策**: Checkpoint は常にスタック上の Copy。backtrack 頻度の高い YAML でもキャッシュラインに収まる。ベンチマークで検証。

### [R3] Input トレイト変更による下流クレート (uri, crond) の修正
→ **軽減策**: 変更は line()/column() メソッド追加と Checkpoint 型変更のみ。uri/crond は Input を直接実装していないので、StrInput の型変更に追従するだけ。Phase 1 の一部として修正。

### [R4] 演算子オーバーロードの blanket impl が将来のトレイト追加と衝突する可能性
→ **軽減策**: Rust の orphan rule により、外部クレートが Parser を実装した型に Add 等を実装しようとしても衝突しない。oni-comb 内部でのみ注意が必要。

### [R5] YAML 1.2 フルスペックの実装量が大きい
→ **軽減策**: Phase 3 は更にサブフェーズに分割可能（フロースタイル → ブロックスタイル → マルチライン → アンカー/エイリアス → タグ）。各サブフェーズで動作する YAML サブセットが得られる。

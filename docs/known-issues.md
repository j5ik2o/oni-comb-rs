# Known Design Issues

## 1. `line_start` 単体ではなく checkpoint 可能な layout context 全体で考える

**要点**:
`yaml-ready-parser` の検討で分かった本質は、`line_start` の単位不整合そのものよりも、
layout-sensitive grammar に必要な文脈を checkpoint と一緒に安全に巻き戻せるかどうかだった。

現在の parser core では、入力位置 (`offset` / `line` / `column` / `line_start`) に加えて、
checkpoint 可能な state を `InputStream` / `Checkpoint` の拡張で保持できる。これにより、
`modules/parser/tests/yaml_ready_acceptance.rs` の litmus grammar 群は、YAML 専用の Layout API を
`modules/parser` に追加せずに通過している。

**現時点の整理**:
- `line_start` は char 列番号ではなく、現在行の先頭を指す byte anchor として扱う
- `column` は人間向けの列番号であり、`line_start` と同じ単位に揃えることは目的にしない
- layout-sensitive grammar の成立性は、`line_start` 単体ではなく、checkpoint 可能な下流所有 state と既存公開契約の組み合わせで担保する

**この issue が意味すること**:
- `line_start` の byte / char 差は責務の違いとして受け入れる
- 問題が再発するなら、それは「専用 Layout API がないこと」ではなく、既存の公開契約だけでは表現不能な grammar が残っているかどうかで判断する
- 追加 API が必要になった場合でも、YAML 固有語彙ではなく最小の generic capability として評価する

## 2. layout context の ergonomic helper は意図的に保留している

**要点**:
現時点では `YAML-ready` の受け入れ条件は満たしているが、downstream grammar が独自 `InputStream`
wrapper や helper を組み合わせて layout context を構成する前提になっている。これは設計上の不足ではなく、
先回りで public Layout API を固定しないための意図的な保留である。

**現時点の整理**:
- `4.1` では、行頭判定・期待インデント判定・flow/block 文脈判定・boolean flag 判定を、既存の公開契約と downstream 側の合成で実現できることを設計に反映した
- `4.2` では、litmus grammar で表現不能なケースが残る場合に限り、YAML 非依存の最小 generic primitive / combinator を検討する方針にした
- `4.3` では、block list、indent nesting、flow/block switching、multiline block、block scalar header、document boundary、simple-key gating、simple-key backtrack、flow plain scalar boundary、indent error を既存契約だけで実証した

**次に追加を検討すべき条件**:
- 既存の公開契約と downstream 側の合成では表現不能な litmus grammar が具体的に再現する
- その不足が YAML 固有ではなく、他の layout-sensitive grammar にも再利用可能な generic capability として説明できる
- 追加 API が `parse_next` 直呼びや `fn_parser` への逃避を減らす明確な根拠を持つ

## 3. 位置付きエラー生成は既知 issue ではなくなった

**現状**:
以前は `ParseError` の `line` / `column` が生成時点で埋まらず、後付け計算が必要だったが、
現在は `InputPosition` と `ExpectError::from_position(...)` により、主要経路のエラー生成時点で
位置情報を保持できる。

**補足**:
- `ParseError` は `offset` / `line` / `column` / `line_start` / `context` を保持できる
- `MergeError` と `ContextError` も新しい位置文脈モデルに合わせて更新済み
- layout-sensitive grammar の診断は、後付け走査ではなくエラー生成時点の文脈を優先する

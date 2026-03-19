# YAML Parser Roadmap

## 概要

`oni-comb-yaml` は最終的に YAML 1.2 の主要機能を扱えるパーサーを目指すが、最初から「構文解析」「スキーマ解釈」「アンカー解決」「merge 適用」を一度に実装すると、`oni-comb-parser` 本体の設計課題と YAML 側の責務分離の問題が混ざる。

このロードマップでは、まず YAML を `syntax parser` と `resolver` に分離して進める。`syntax parser` は YAML の表記を保持した構文木を返し、`resolver` が plain scalar の型解釈、tag 適用、anchor / alias / merge を処理する。

## 背景

- JSON は現行の parser combinator のチェインだけで自然に実装できている
- YAML は block indentation と document-local state を要求する
- 現行 `Parser<I>` は input だけを rewind し、parser 内部状態は rewind しない
- そのため anchor table のような mutable state を parser 側に直接持たせると `or` / `attempt` と整合しない

## 基本方針

### 1. 構文解析と意味解決を分ける（D1）

```text
src
 ↓
parse_syntax()          → Result<YamlSyntaxDocument, ParseError>
 ↓
YamlSyntaxDocument
 ↓
resolve()               → Result<YamlValue, ResolveError>
 ↓
YamlValue
```

- `parse_syntax(src)` / `parse_syntax_documents(src)` は additive な低レベル API として導入する
- `parse(src)` / `parse_documents(src)` は resolved API の責務を維持し、最終的に `parse_syntax` + `resolve` の合成にする
- `parse_syntax` 系の導入によって `parse` 系を syntax-only に格下げしない

### 2. parser core の拡張は後段で再評価する（D4）

最初から `oni-comb-parser` に user state や parameterized recursion を追加しない。まず syntax-only の YAML を試し、その結果から本当に必要な拡張だけを絞る。

### 3. 最初の実装単位は小さく切る（D3）

Phase 1 では、scalar と flow style を中心に syntax-only の API と AST 境界を確立する。block syntax と resolver は後続フェーズに分離する。

### 4. エラー型は parse と resolve で分離する（D6）

- `ParseError`: syntax parsing の失敗を表す
- `ResolveError`: resolver 段での失敗を表す（未定義 anchor/alias、不正 merge key、tag 適用失敗、schema 整合性違反）
- 高レベル `parse()` のエラー統合方針は後続フェーズで設計する

## レイヤ構成

### Syntax Layer

- YAML の見た目を保持する
- plain scalar をまだ `int` や `bool` に確定しない（D2）
- anchor / alias / tag は未解決の構文ノードとして保持する

Phase 1 で固定する AST 形:

- `YamlSyntaxDocument { root: YamlSyntaxNode }`
- `YamlSyntaxNode::Scalar(YamlSyntaxScalar)`
- `YamlSyntaxNode::Sequence { style, items }`
- `YamlSyntaxNode::Mapping { style, entries }`
- `YamlSyntaxScalar::Plain(String)`
- `YamlSyntaxScalar::SingleQuoted(String)`
- `YamlSyntaxScalar::DoubleQuoted(String)`

後続フェーズで追加予定のノード:

- `YamlSyntaxNode::Alias`
- `YamlSyntaxNode::Anchored`
- `YamlSyntaxNode::Tagged`

### Resolve Layer

- plain scalar の Core Schema 解釈
- `!!str` などの tag 適用
- anchor 登録
- alias 解決
- merge key `<<` 適用
- `ResolveError` による意味解決失敗の表現

### Convenience API Layer

- `parse(src)` は最終的に `parse_syntax(src)` と `resolve(doc)` の合成 API にする
- 実装段階では `parse_syntax` を先に安定させる
- `parse_syntax` 系 API の導入は additive change とし、既存の `parse` / `parse_documents` を syntax-only API に格下げしない

## メモリ前提

- YAML の syntax AST と resolver は `String`、`Vec`、`BTreeMap` 等を自然に必要とするため、現時点では `alloc` 前提で進める
- `oni-comb-parser` 本体の `core-only` 方針とは分けて考える
- 将来 `alloc` 依存を減らすとしても、Phase 1 の主要判断軸にはしない

## フェーズ

### Phase 1. Syntax Foundation

対象:

- `parse_syntax(src)` / `parse_syntax_documents(src)` の導入
- syntax-only AST の導入（上記「Phase 1 で固定する AST 形」）
- plain / single-quoted / double-quoted scalar
- flow sequence / flow mapping
- 基本 comment（行コメント）
- document marker（`---` / `...`）の基本

除外:

- block mapping / block sequence
- block scalar
- alias 解決
- merge key 適用
- tag による型強制
- convenience `parse()` の完成

未対応機能の扱い:

- Phase 1 対象外の構文に遭遇した場合、`parse_syntax` 系 API は `ParseError` を返す
- silent ignore ではなく明示的なエラーとする

成果物:

- OpenSpec change（`split-yaml-parse-and-resolve`）
- Phase 1 spec（`yaml-syntax-phase1`）
- 実装後に block syntax へ進むための AST 境界

完了後: Gate A の評価を開始する

### Phase 2. Block Syntax

対象:

- block mapping
- block sequence
- nested block structure
- indentation による親子関係の構築

観測ポイント:

- `guard + line/column` だけで自然に書けるか
- `recursive()` だけで十分か
- `node(indent)` 相当の parameterized recursion が必要か

完了後: Gate A を評価し、core 拡張の要否を判断する

### Phase 3. Extended Syntax

対象:

- literal / folded block scalar
- chomping indicator
- tag syntax
- anchor syntax
- alias syntax
- multi-document syntax の拡張

完了後: Gate B の評価材料が揃う

### Phase 4. Resolver

対象:

- plain scalar の schema 解釈
- tag 適用
- anchor / alias 解決
- merge key 適用

完了後: Gate B, Gate C を評価する

### Phase 5. Parser Core Review

Phase 2-4 の実装結果から、`oni-comb-parser` 本体の拡張要否を再評価する。

候補:

- backtrack-safe user state
- parameterized recursion
- dynamic expectation / indentation error support

## 判断ゲート

### Gate A. Syntax-only の block 実装は自然か

評価タイミング: Phase 2 完了後

以下のいずれかに当てはまる場合、core 拡張を検討する。

- block mapping / sequence が手続き型 `parse_next` だらけになる
- indent 引き回しのために不自然な `fn_parser` 依存が増える
- syntax-only でも parser 内部状態が欲しくなる

Phase 1 実装後の評価:

- 現時点では `oni-comb-parser` 本体の拡張は採用しない
- `modules/yaml/src/syntax_parser/` 配下の `SyntaxParser { src, pos }` 実装は flow subset と document marker を処理できており、syntax parsing はすでに YAML モジュール内のローカル実装として成立している
- block scalar / anchor / alias / tag / block collection は未対応機能として明示的に切り分けられているため、次段階ではまず `SyntaxParser` に indentation 引数や補助メソッドを足す試作で十分に評価できる
- anchor table や merge 解決は resolver 側の責務として分離する方針を維持できるため、Gate A の時点で backtrack-safe user state を parser core に入れる根拠はまだない

次の判断基準:

- Phase 2 の block mapping / sequence 試作が `SyntaxParser` のローカル状態だけで不自然になるまでは core を広げない
- 上の判定条件に実際に当てはまった時点で、parameterized recursion や indentation-aware error support を Phase 5 の候補として再評価する

### Gate B. Resolver 分離で alias / merge が素直になるか

評価タイミング: Phase 4 完了後

以下の条件を満たすなら、alias / merge を parser core に押し込まない。

- syntax AST から一意に resolve できる
- document-local anchor table を resolver 側で管理できる
- parse 時の backtracking と state を混ぜずに済む

### Gate C. 最終 API は理解しやすいか

評価タイミング: Phase 4 完了後

以下の構成が維持できるかを確認する。

- 低レベル API: `parse_syntax`
- 中レベル API: `resolve`
- 高レベル API: `parse`

## Phase 1 の非目標

- full YAML 1.2 準拠
- block syntax 全対応
- alias / merge 解決
- parser core の拡張決定

## オープンクエスチョン

- syntax AST で `<<` を通常キーとして持つか、専用ノードにするか
- plain scalar を `Scalar::Plain(String)` に寄せるか、raw span を残すか
- block scalar の正規化を syntax 層でやるか、resolver 層に送るか
- Phase 1 の間、既存の `parse` / `parse_documents` を `todo!()` のまま維持するか、明示的に未実装として扱うか

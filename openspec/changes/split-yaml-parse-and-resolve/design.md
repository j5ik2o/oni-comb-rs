## Context

`oni-comb-parser` は stateless な input rewind を前提にした parser combinator core であり、JSON のような文法には自然に適用できる。一方 YAML は block indentation、anchor / alias、merge key、tag による型強制を含み、構文解析だけでなく document-local な意味解決も必要になる。

現在の `Parser<I>` は input しか rewind しないため、anchor table のような mutable state を parser 側に直接持たせると `or` / `attempt` と整合しない。このため、まず YAML を syntax parsing と resolving に分離して進め、core 拡張の必要性を段階的に見極める。

## Goals / Non-Goals

**Goals:**

- YAML 実装の責務を `parse_syntax` と `resolve` に分離する
- Phase 1 の実装範囲を syntax-only の subset に限定する
- syntax AST の境界を先に固定し、後続の block syntax と resolver を載せやすくする
- Phase 1 完了後に parser core の拡張要否を再評価できる状態にする

**Non-Goals:**

- Phase 1 で full YAML 1.2 を実装すること
- Phase 1 で alias / merge / tag 解決を完了すること
- Phase 1 の時点で `oni-comb-parser` 本体に user state や parameterized recursion を導入すること
- 既存のフル `yaml-parser` spec をこの change だけで実装完了に持ち込むこと

## Decisions

### D1. YAML は `syntax parser` と `resolver` に分離する

**選択**:

- `parse_syntax(src) -> Result<YamlSyntaxDocument, ParseError>`
- `parse_syntax_documents(src) -> Result<Vec<YamlSyntaxDocument>, ParseError>`
- `resolve(doc) -> Result<YamlValue, ResolveError>`
- `parse(src)` / `parse_documents(src)` は最終的に二段を束ねる convenience API にする
- Phase 1 では `parse_syntax` 系を追加しても、既存の `parse` / `parse_documents` の最終責務を `syntax-only` に変更しない

**理由**:

- 構文解析と意味解決の責務を分離できる
- alias / merge を parser-time state で扱わずに済む
- backtracking と document-local state の衝突を避けられる
- 既存の `parse` 系 API を full YAML の到達点として残せる

**代替案**:

- `parse()` 一発で `YamlValue` を返す
  - 問題: syntax と resolve が密結合になり、parser core の設計課題が見えにくい

### D2. Syntax AST は「見えた表記」を保持し、解釈しすぎない

**選択**:

- plain scalar は Phase 1 ではまだ `bool` や `int` にしない
- tag / anchor / alias は未解決ノードとして保持できる前提にする
- Phase 1 の parser がそれらをまだ生成しなくても、syntax AST は後続フェーズで `Tagged` / `Anchored` / `Alias` を無理なく追加できる形にする
- Phase 1 で最低限固定する AST 形は以下とする
  - `YamlSyntaxDocument { root: YamlSyntaxNode }`
  - `YamlSyntaxNode::Scalar(YamlSyntaxScalar)`
  - `YamlSyntaxNode::Sequence { style, items }`
  - `YamlSyntaxNode::Mapping { style, entries }`
  - `YamlSyntaxScalar::Plain(String)`
  - `YamlSyntaxScalar::SingleQuoted(String)`
  - `YamlSyntaxScalar::DoubleQuoted(String)`

**理由**:

- `!!str 42` と plain scalar schema 解釈の衝突を避けられる
- resolver に責務を寄せやすい
- block syntax と flow syntax の AST を統一しやすい

**代替案**:

- parse 時点で `YamlValue` へ直接正規化する
  - 問題: schema 解釈と構文保持が混ざる

### D3. Phase 1 は syntax-only の最小 subset に絞る

**選択**:

- 対象: scalar、flow sequence、flow mapping、基本 comment、基本 document marker
- 非対象: block mapping、block sequence、block scalar、alias 解決、merge key、tag 強制
- 非対象機能に遭遇した `parse_syntax` 系 API は、曖昧な未対応結果ではなく `ParseError` を返す

**理由**:

- 最初の変更単位を小さく保てる
- AST と API 境界を先に固められる
- flow subset は現行 combinator で自然に表現しやすい
- 呼び出し側の契約を固定できる

**代替案**:

- block syntax まで一気に含める
  - 問題: parameterized recursion 問題と resolver 問題が同時に立ち上がる

### D4. parser core の再評価は block syntax 実装後に行う

**選択**:

- Phase 1 では core 拡張を行わない
- block syntax の試作結果を観測してから user state / parameterized recursion を検討する

**理由**:

- 先に core を広げると、必要以上の抽象を導入しやすい
- syntax-only の小さい成功例を先に作った方が判断材料が増える

**代替案**:

- 先に `Parser<Input, State>` へ拡張する
  - 問題: 仕様がまだ粗い段階で parser 全体の性格を変えてしまう

### D5. 全体方針は OpenSpec だけでなく docs にも残す

**選択**:

- OpenSpec change は Phase 1 の実行単位を定義する
- `docs/yaml-parser-roadmap.md` に中長期ロードマップと判断ゲートを書く

**理由**:

- OpenSpec の小さい change を保ちながら全体文脈を共有できる
- 後続フェーズで方針の連続性を持てる

### D6. Resolve error は parse error と分離する

**選択**:

- `ParseError` は syntax parsing の失敗を表す
- `ResolveError` は resolver 段での失敗を表す
- `ResolveError` の責務には少なくとも以下を含める
  - 未定義 anchor / alias
  - 不正な merge key
  - tag 適用失敗
  - schema 解釈に関する整合性違反

**理由**:

- 構文失敗と意味解決失敗を分離できる
- `parse_syntax` と `resolve` の責務境界が明確になる
- 高レベル `parse()` のエラー統合方針を後続で設計しやすい

## Risks / Trade-offs

- [Phase 1 だけでは block YAML への答えが出ない] → ロードマップに判断ゲートを明示し、Phase 2 で core 拡張要否を再評価する
- [syntax AST が将来不足する] → tag / anchor / alias を収容できる余地を設計時点で残す
- [`yaml-parser` の長期 spec と Phase 1 spec が二重管理に見える] → 長期目標は既存 spec、短期の実行単位は新 capability と役割を分ける
- [`parse()` の最終 API が遅れて見える] → Phase 1 では低レベル API を先に安定させる意図を明示する
- [既存 `parse` 系 API と Phase 1 API の関係が曖昧になる] → `yaml-parser` capability 側に delta spec を追加し、`parse` は resolved API、`parse_syntax` は additive API として整理する

## Migration Plan

1. `docs/yaml-parser-roadmap.md` を追加し、段階導入と判断ゲートを明文化する
2. `yaml-syntax-phase1` spec を追加し、Phase 1 の実装範囲を固定する
3. `yaml-parser` capability に delta spec を追加し、既存 `parse` / `parse_documents` と `parse_syntax` 系 API の役割分担を固定する
4. Phase 1 実装では `parse_syntax` 系 API と syntax AST を導入する
5. Phase 2 以降で block syntax と resolver を追加する
6. block syntax の実装結果を踏まえて parser core の拡張要否を判断する

## Open Questions

- `YamlSyntaxNode` に `Anchored` / `Tagged` / `Alias` を Phase 1 から入れるか、Phase 3 で拡張するか
- `parse()` を Phase 1 時点で未公開にするか、限定的な convenience API にするか
- `<<` を syntax AST で通常キーとして保持するか、専用ノードにするか
- block scalar のテキスト正規化を syntax 層に置くか resolver 層に置くか

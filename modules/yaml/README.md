# oni-comb-yaml

YAML 1.2 parser built on `oni-comb-parser`.

## 短期目標（Phase 1: Syntax Foundation）

Phase 1 では syntax-only の parsing API と最小の syntax AST を確立する。

- `parse_syntax(src)` / `parse_syntax_documents(src)` を提供する
- plain / single-quoted / double-quoted scalar を構文上区別して保持する
- flow sequence / flow mapping をパースする
- 行コメントを無視する
- `---` / `...` の基本 document marker を扱う
- Phase 1 対象外の構文（block syntax、alias、merge key、tag）に遭遇した場合は `ParseError` を返す

`parse_syntax` 系は plain scalar を `int` や `bool` に解釈しない。`parse` / `parse_documents` は
Phase 1 で対応した flow subset に限って resolved value を返し、意味解決の本体は後続フェーズの
resolver に移す。

## 長期目標

最終的には YAML 1.2 の主要機能を扱えるパーサーを提供する。

- block mapping / block sequence / block scalar の構文解析
- anchor / alias 構文の解析
- tag 構文の解析
- resolver による plain scalar の schema 解釈、tag 適用、alias 解決、merge key 適用
- `parse(src)` / `parse_documents(src)` を `parse_syntax` + `resolve` の合成 API として拡張する

長期目標の詳細は [`docs/yaml-parser-roadmap.md`](../../docs/yaml-parser-roadmap.md) を参照。

## 短期と長期の境界

| 機能 | 短期（Phase 1） | 長期（Phase 2+） |
|------|:---:|:---:|
| plain / quoted scalar | ✓ | ✓ |
| flow sequence / mapping | ✓ | ✓ |
| comment / document marker | ✓ | ✓ |
| block mapping / sequence | — | ✓ |
| block scalar | — | ✓ |
| anchor / alias 構文 | — | ✓ |
| tag 構文 | — | ✓ |
| resolver（型解釈、alias 解決、merge） | — | ✓ |
| `parse()` / `parse_documents()` の flow subset 対応 | ✓ | ✓ |

## API 概要

### Syntax API（Phase 1 で導入）

```rust
// 単一ドキュメントの syntax tree を返す
pub fn parse_syntax(src: &str) -> Result<YamlSyntaxDocument, ParseError>;

// 複数ドキュメントの syntax tree を返す
pub fn parse_syntax_documents(src: &str) -> Result<Vec<YamlSyntaxDocument>, ParseError>;
```

### Resolved API（Phase 1 は flow subset のみ）

```rust
// 単一ドキュメントを解釈済みの YamlValue として返す
pub fn parse(src: &str) -> Result<YamlValue, ParseError>;

// 複数ドキュメントを解釈済みの YamlValue の列として返す
pub fn parse_documents(src: &str) -> Result<Vec<YamlValue>, ParseError>;
```

`parse` / `parse_documents` は resolved API の責務を維持する。`parse_syntax` 系の導入によって syntax-only API に格下げしない。

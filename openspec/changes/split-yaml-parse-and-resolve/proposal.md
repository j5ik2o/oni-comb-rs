## Why

現状の YAML 要件は、構文解析と意味解決が一つの `parse()` に混ざっている。そのまま実装を始めると、`oni-comb-parser` 本体の設計制約と YAML 側の責務分離の問題が区別できず、不要に大きい設計変更へ進みやすい。

## What Changes

- YAML 実装方針を `syntax parser` と `resolver` の二段階に分離する
- 全体計画を `docs/yaml-parser-roadmap.md` に明文化する
- Phase 1 の実装範囲を syntax-only の小さい subset に限定する
- Phase 1 では `parse_syntax` 系 API と syntax AST を導入し、scalar / flow style / 基本 comment / 基本文書マーカーを対象にする
- block syntax、anchor / alias 解決、merge key、tag による型強制は後続フェーズへ明示的に defer する
- Phase 1 完了後に、block syntax の実装結果を材料として `oni-comb-parser` 本体の拡張要否を再評価する

## Capabilities

### New Capabilities
- `yaml-syntax-phase1`: YAML を syntax-only で解析する Phase 1 API と要件を定義する

### Modified Capabilities
- `yaml-parser`: 既存の `parse` / `parse_documents` を最終的な resolved API として維持しつつ、`parse_syntax` 系の段階導入を追加する

## Impact

- `docs/yaml-parser-roadmap.md`: 全体ロードマップ、フェーズ、判断ゲートを追加
- `modules/yaml`: `parse_syntax` 系 API、syntax AST、Phase 1 subset の実装計画に影響
- `modules/yaml/src/lib.rs`: 既存の `parse` / `parse_documents` と新規 `parse_syntax` 系 API の責務分離を明文化する必要がある
- `modules/yaml/tests/yaml_parse.rs`: 既存の full YAML 前提テストと Phase 1 実装範囲の扱いを整理する必要がある
- `openspec/specs/yaml-parser/spec.md`: フル YAML の長期目標として参照しつつ、短期の実装単位は別 capability に分離
- 今回の change 自体はまず設計と要件整理が中心で、parser core の拡張は含まない

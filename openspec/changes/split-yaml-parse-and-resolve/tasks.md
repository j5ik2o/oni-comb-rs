## 1. Roadmap And Scope

- [x] 1.1 `docs/yaml-parser-roadmap.md` を追加し、`syntax parser + resolver` 分離方針、フェーズ、判断ゲートを記述する
- [x] 1.2 `modules/yaml` の短期目標と長期目標の境界を README またはモジュールドキュメントで明文化する
- [x] 1.3 既存の `parse` / `parse_documents` と新規 `parse_syntax` 系 API の責務分離を `modules/yaml` の公開 API 方針として明文化する

## 2. Syntax API And AST

- [x] 2.1 `modules/yaml` に Phase 1 用の syntax AST 型を追加する
- [x] 2.2 `parse_syntax(src)` と `parse_syntax_documents(src)` の公開 API を追加する
- [x] 2.3 plain / single-quoted / double-quoted scalar を syntax-only で保持する
- [x] 2.4 `YamlSyntaxDocument` / `YamlSyntaxNode` / `YamlSyntaxScalar` の最小構造を spec に沿って実装する
- [x] 2.5 syntax AST が後続フェーズで `Tagged` / `Anchored` / `Alias` を追加できる形になっていることを確認する

## 3. Phase 1 Parsing

- [x] 3.1 flow sequence の syntax parser を実装する
- [x] 3.2 flow mapping の syntax parser を実装する
- [x] 3.3 comment と基本 document marker の処理を実装する
- [x] 3.4 Phase 1 対象外の block syntax と alias 系で `ParseError` を返す
- [x] 3.5 既存の `parse` / `parse_documents` を Phase 1 でどう扱うかを決める
- [x] 3.6 既存の full YAML 前提テストの扱い方針を決める
- [x] 3.7 方針に従って必要なテスト整理を行う

## 4. Verification

- [x] 4.1 `yaml-syntax-phase1` spec に対応するテストを追加する
- [x] 4.2 実装後に block syntax の試作結果を振り返り、parser core 拡張の要否を評価する

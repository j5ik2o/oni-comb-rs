## Why

現在の JSON ベンチ実装では `whitespace0()` を値の前後や区切り記号のたびに繰り返し呼んでおり、JSON subset / full JSON の両方で fixed cost が残っている。`fn_parser`、`peek_byte`、ゼロコピー化で大きなボトルネックはすでに潰れているため、次の改善対象は空白処理の重複スキャンを減らすことになる。

## What Changes

- JSON フルベンチ用パーサーで、空白消費を grammar boundary ごとに一度だけ行う構造へ整理する
- `alloc_count.rs` の JSON パーサーを同じ空白処理方針に揃え、ベンチとアロケーション計測の実装差をなくす
- JSON subset ベンチの `ws()` ベース構成を見直し、不要な前後空白処理の重複を減らす
- 既存の受理言語と AST 形状を維持したまま、空白処理の責務を `value` / `member` / `delimiter` の境界に再配置する
- ベンチ結果と README を更新し、fixed cost 改善の有無を記録する

## Capabilities

### New Capabilities
- `json-whitespace-optimization`: ベンチ用 JSON パーサーが、JSON の許容する空白を維持しつつ、重複した `whitespace0()` 呼び出しを減らして fixed cost を抑える

### Modified Capabilities

（既存 spec に対する要件変更なし）

## Impact

- `modules/parser/benches/json_full.rs`: 手書き JSON パーサーの空白処理境界を整理
- `modules/parser/benches/alloc_count.rs`: `json_full.rs` と同じ空白処理方針へ追従
- `modules/parser/benches/workloads/json.rs`: `ws()` 依存の構成を見直し、subset ベンチの空白固定コストを削減
- `modules/parser/benches/README.md`
- `modules/parser/benches/README.ja.md`
- 必要に応じて JSON ベンチ用の補助関数を追加するが、公開 API 変更は行わない

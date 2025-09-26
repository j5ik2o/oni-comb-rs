# JSON ベンチマーク計画

## 目的
- `oni-comb` が `nom` と `pom` に対してどの程度の性能・可読性を持つかを、大規模 JSON パースを通じて確認する。
- 連続数字/CSV に偏っている既存ベンチに、構造的・文字列処理が多いワークロードを追加する。
- 成功ケースだけでなく、エラー生成コスト（メッセージ品質含む）も比較できる基礎を作る。

## 測定対象
| 項目 | 説明 |
| --- | --- |
| `oni_comb` | 今回実装する JSON パーサー (コンビネータ活用) |
| `nom` | `nom` 7 系列の標準コンビネータで書いた JSON パーサー |
| `pom` | `pom` 3 系列の JSON パーサー |
| `serde_json` (任意) | 参照実装としての `serde_json::from_slice`（オプション） |

## 入力データ
- `benches/data/heavy.json`: 巨大 JSON 1 本（深いネスト + 大配列 + 長文字列 + Unicode）。
- `benches/data/fail/` 以下に故意にエラーになる JSON を複数用意（`missing_comma.json`, `unclosed_brace.json` など）。
- 必要に応じて `build.rs` で自動生成も検討（ただし determinism 優先なら固定ファイル）。

## 共通 AST
- `serde_json::Value` を使用。各ライブラリ版は `Value` を返す or 変換する。
- これにより、成功ケースでの結果比較が容易。

## 実装タスク
1. `parser/benches/json/oni_comb.rs` に `oni_comb_json_value(input: &[u8]) -> ParseResult<Value>` を実装。
2. `parser/benches/json/nom.rs` / `pom.rs` で同等処理を実装（既存サンプルを最小限に調整）。
3. `parser/benches/json.rs` で Criterion ベンチを定義:
   - `parse_heavy_success`: 各実装が `heavy.json` をパース。
   - `parse_failures`: 失敗入力をループし、エラーメッセージ生成時間を計測。
   - 可能なら `serde_json::from_slice` を比較軸に追加。
4. ベンチ入力を `include_bytes!` もしくは `std::fs::read` で読み込む（I/O が測定に影響しないよう `lazy_static` / `once_cell` でキャッシュ）。
5. README/PLAN を更新して `cargo bench --bench json` の使い方を提示。

## 拡張案
- AST を構造体でも保持し、`Value` 変換コストを避けたベンチ（純粋速度比較）。
- 解析後に `peek_not` や `expect` でエラーメッセージを追加するケースの測定。
- JSON 以外の DSL（例: HOCON, GraphQL）でも同様のベンチを展開。

## 測定結果 (2025-09-26)

### 成功ケース `heavy.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 425 µs |
| `nom` | 約 23.1 µs |
| `pom` | 約 329 µs |
| `serde_json` | 約 5.71 µs |

### 失敗ケース `missing_comma.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 7.46 µs |
| `nom` | 約 0.80 µs |
| `pom` | 約 6.38 µs |
| `serde_json` | 約 0.107 µs |

### 失敗ケース `unclosed_brace.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 17.3 µs |
| `nom` | 約 1.25 µs |
| `pom` | 約 15.1 µs |
| `serde_json` | 約 0.105 µs |

備考:
- `pom` ベンチは 100 サンプル取得に 1.7s 程度かかったため、Criterion から計測時間延長の警告が出た。
- `nom` 実装は参照実装のパーサーを元に `serde_json::Value` へ変換する形で再構築。今後の高速化余地あり。
- `oni_comb` は現状で `heavy.json` 成功ケースが `pom` 比でおよそ 1.3x、`nom` 比で約 18x の遅延。最適化タスク候補。

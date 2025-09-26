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

## 測定結果 (2025-09-27 03:20 JST)

### 成功ケース `heavy.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 131 µs |
| `nom` | 約 21.7 µs |
| `pom` | 約 328 µs |
| `serde_json` | 約 5.72 µs |

### 失敗ケース `missing_comma.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 2.61 µs |
| `nom` | 約 0.735 µs |
| `pom` | 約 6.37 µs |
| `serde_json` | 約 0.106 µs |

### 失敗ケース `unclosed_brace.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 6.02 µs |
| `nom` | 約 1.31 µs |
| `pom` | 約 14.7 µs |
| `serde_json` | 約 0.104 µs |

備考:
- `pom` ベンチは 100 サンプル取得に 1.6〜1.7s 程度要し、Criterion から計測時間延長の警告が出る点は変わらず。
- `nom` 実装は参照実装ベースで `serde_json::Value` に変換する構成。さらなる高速化余地あり。
- `oni_comb` は `heavy.json` 成功ケースで `pom` 比 ~0.40x、`nom` 比 ~6.0x。`missing_comma` で前回よりわずかに遅延（+約2%）だがノイズ範囲内。

## 再計測 (2025-09-27 03:35 JST, measurement-time = 2s)

### 成功ケース `heavy.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 131 µs (中央値 131.0 µs) |
| `nom` | 約 21.8 µs |
| `pom` | 約 326 µs |
| `serde_json` | 約 5.67 µs |

### 失敗ケース `missing_comma.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 2.62 µs |
| `nom` | 約 0.762 µs |
| `pom` | 約 6.31 µs |
| `serde_json` | 約 0.108 µs |

### 失敗ケース `unclosed_brace.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 6.13 µs |
| `nom` | 約 1.30 µs |
| `pom` | 約 15.1 µs |
| `serde_json` | 約 0.106 µs |

備考:
- 測定時間延長により推定誤差は縮小、`oni_comb` / `pom` は前回値とほぼ同等。`nom` 失敗ケースは +約3% の揺らぎが観測されたが、Criterion 判定では統計的回帰。
- 依然として `pom` は 100 サンプル取得に 3 秒前後を要し、高ノイズ状態が継続。

## 追加調査 (2025-09-27 03:50 JST, sampling-mode = flat)

環境変数 `CRITERION_SAMPLING_MODE=flat`（併せて `CRITERION_SAMPLE_SIZE=200` を設定したが、現行 Criterion では 100 サンプル固定の模様）で再測定。

### 成功ケース `heavy.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 131 µs |
| `nom` | 約 22.0 µs |
| `pom` | 約 325 µs |
| `serde_json` | 約 5.62 µs |

### 失敗ケース `missing_comma.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 2.61 µs |
| `nom` | 約 0.740 µs |
| `pom` | 約 6.27 µs |
| `serde_json` | 約 0.107 µs |

### 失敗ケース `unclosed_brace.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 6.05 µs |
| `nom` | 約 1.21 µs |
| `pom` | 約 14.7 µs |
| `serde_json` | 約 0.104 µs |

備考:
- flat sampling により `nom` 失敗ケースのブレは減少し、回帰警告は「改善」側へ転じた（約 -3〜-4%）。
- sample-size 200 の CLI オプションは未対応。より厳密な解析が必要な場合はベンチ側コードで `Criterion::default().sample_size(200)` を指定する対応を検討。

## 設定更新 (2025-09-27 04:10 JST)

- `parser/benches/json.rs` の `criterion_group!` にて `Criterion::default().sample_size(200)` / `measurement_time(3s)` / `warm_up_time(1s)` をデフォルト設定に追加。これにより CLI 指定なしでも高密度サンプリングが行われる。
- ベンチグループごとに `SamplingMode::Flat` を指定し、サンプル収集のブレを抑制。
- 追加で `json_success_quick` / `json_failures_quick` グループを用意し、`sample_size=120`・`measurement_time=1.2s`・`warm_up_time=600ms` の軽量計測を選択できるようにした。
- `measurement_time` を 3 秒へ延長することで、重いケースでも 200 サンプル取得時の警告が解消（CLI で `--measurement-time 2` を明示した場合のみ警告が再現）。

### 高密度計測結果 (sample_size = 200, measurement_time = 3s, sampling_mode = flat)

#### 成功ケース `heavy.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 130 µs |
| `nom` | 約 22.1 µs |
| `pom` | 約 326 µs |
| `serde_json` | 約 5.77 µs |

#### 失敗ケース `missing_comma.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 2.62 µs |
| `nom` | 約 0.74 µs |
| `pom` | 約 6.37 µs |
| `serde_json` | 約 0.107 µs |

#### 失敗ケース `unclosed_brace.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 6.08 µs |
| `nom` | 約 1.22 µs |
| `pom` | 約 14.9 µs |
| `serde_json` | 約 0.104 µs |

備考:
- サンプル増に伴い `nom` / `serde_json` に統計的差分が出る場合があるが、多くは ±2% 程度で外れ値の影響と考えられる。
- CLI 側で `--measurement-time 2` を指定すると再び警告が表示されるため、必要に応じて `--measurement-time 3` 以上を利用。

### クイック計測結果 (sample_size = 120, measurement_time = 1.2s, sampling_mode = flat)

#### 成功ケース `heavy.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 130 µs |
| `nom` | 約 21.7 µs |
| `pom` | 約 326 µs |
| `serde_json` | 約 5.80 µs |

#### 失敗ケース `missing_comma.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 2.63 µs |
| `nom` | 約 0.73 µs |
| `pom` | 約 6.31 µs |
| `serde_json` | 約 0.107 µs |

#### 失敗ケース `unclosed_brace.json`
| 実装 | 時間 (平均) |
| --- | --- |
| `oni_comb` | 約 6.08 µs |
| `nom` | 約 1.20 µs |
| `pom` | 約 14.7 µs |
| `serde_json` | 約 0.104 µs |

備考:
- クイック計測では高密度版より外れ値率がわずかに高いが、`SamplingMode::Flat` とウォームアップ 600ms の組み合わせで極端な値は抑制。
- クイック実行でより短時間に傾向を把握し、詳細確認が必要な際はフル版を併用するフローを推奨。
- クイック計測 (`json_success_quick` / `json_failures_quick`) は 約 1.2 秒・120 サンプルで実行でき、CI や高速確認向け。コマンド例: `cargo bench --bench json json_success_quick`。

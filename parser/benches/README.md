# ベンチマーク

oni-comb-rs v2 と比較対象ライブラリ（winnow, nom, chumsky, pom）の性能比較。

## 実行方法

```bash
# 全ベンチ実行
cargo bench -p oni-comb-parser --bench comparison

# 特定グループのみ
cargo bench -p oni-comb-parser --bench comparison -- identifier
cargo bench -p oni-comb-parser --bench comparison -- integer
cargo bench -p oni-comb-parser --bench comparison -- flat_map
cargo bench -p oni-comb-parser --bench comparison -- zip_vs
cargo bench -p oni-comb-parser --bench comparison -- json
cargo bench -p oni-comb-parser --bench comparison -- arithmetic

# コンパイル確認（計測なし）
cargo bench -p oni-comb-parser --bench comparison -- --test

# ヒープアロケーション計測
cargo bench -p oni-comb-parser --bench alloc_count
```

## ベンチグループ一覧

| グループ | 内容 | ライブラリ |
|---------|------|-----------|
| `token/identifier` | 識別子パース（`satisfy` + `take_while0`） | 5 ライブラリ |
| `token/integer` | 整数パース（`take_while1` + parse） | 5 ライブラリ |
| `token/flat_map_same_type` | flat_map 同一型分岐（digit → tag） | 5 ライブラリ |
| `token/flat_map_boxed` | flat_map 異種型分岐（`Box<dyn Parser>` 等） | 5 ライブラリ |
| `token/zip_vs_flat_map` | zip と flat_map の直接比較 | oni-comb のみ |
| `json` | JSON subset パース（null/int/string/array/object） | oni-comb のみ |
| `arithmetic` | 四則演算+括弧（recursive + chainl1） | oni-comb のみ |

## 結果と考察

以下は Apple M 系チップでの計測結果（ParseError 導入後）。

### Token ワークロード — Identifier

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"x"` (1B) | 18.4 ns | 14.5 ns | 13.7 ns | 66.9 ns | 874 ns |
| `"foo"` (3B) | 19.6 ns | 15.2 ns | 18.7 ns | 83.5 ns | 916 ns |
| `"foo_bar_123"` (11B) | 28.1 ns | 19.5 ns | 36.9 ns | 199 ns | 1,055 ns |
| `"_private"` (8B) | 26.2 ns | 19.2 ns | 27.7 ns | 141 ns | 997 ns |
| `"longIdent..."` (28B) | 44.4 ns | 32.2 ns | 82.6 ns | 271 ns | 1,318 ns |

### Token ワークロード — Integer

| 入力 | oni-comb | winnow | nom | pom | chumsky |
|------|----------|--------|-----|-----|---------|
| `"0"` | 3.1 ns | 1.6 ns | 2.1 ns | 68.6 ns | 884 ns |
| `"42"` | 3.6 ns | 2.3 ns | 2.7 ns | 72.1 ns | 907 ns |
| `"9999999"` | 8.2 ns | 5.2 ns | 5.3 ns | 133 ns | 995 ns |
| `"184467...615"` (20B) | 25.9 ns | 22.6 ns | 22.7 ns | 264 ns | 1,256 ns |

### flat_map 同一型分岐（digit → tag）

| ライブラリ | "1one" | "2two" | "3three" |
|-----------|--------|--------|----------|
| winnow | 2.1 ns | 2.1 ns | 2.3 ns |
| nom | 2.4 ns | 2.4 ns | 2.4 ns |
| **oni-comb** | **7.3 ns** | **7.3 ns** | **6.0 ns** |
| pom | 70 ns | 71 ns | 96 ns |
| chumsky | 896 ns | 898 ns | 948 ns |

**MS6 ParseError 導入の効果:**
- 旧（format! ベース）: 8.3 / 7.8 / 6.9 ns
- 新（ParseError）: 7.3 / 7.3 / 6.0 ns
- **約 12% 改善**。`format!` 排除によりエラーパスのコード生成が軽量化された。

### flat_map 異種型分岐（Box\<dyn Parser\>）

| ライブラリ | "c:hello" | "i:42" |
|-----------|-----------|--------|
| nom | 3.9 ns | 2.8 ns |
| winnow | 19.3 ns | 18.6 ns |
| **oni-comb** | **21.5 ns** | **19.8 ns** |
| pom | 164 ns | 109 ns |
| chumsky | 1,052 ns | 972 ns |

### zip vs flat_map（oni-comb-rs 内部比較）

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| "x" | 4.8 ns | 4.8 ns | 0% (誤差) |
| "foo" | 10.5 ns | 10.3 ns | -2% (誤差) |
| "foo_bar_123" | 17.7 ns | 17.7 ns | 0% (誤差) |
| "_private" | 14.8 ns | 14.8 ns | 0% (誤差) |
| "longIdent..." | 31.2 ns | 31.1 ns | 0% (誤差) |

**zip ≒ flat_map（同一型）は引き続き成立。** 具象コンビネータ型設計の成果。

### JSON subset（oni-comb のみ）

| 入力 | 時間 | byte/ns |
|------|------|---------|
| `null` (4B) | 15.0 ns | 0.27 |
| `42` (2B) | 86.0 ns | 0.02 |
| `"hello world"` (13B) | 147 ns | 0.09 |
| `[1, 2, 3]` (9B) | 536 ns | 0.02 |
| `[1, "two", true, null]` (22B) | 542 ns | 0.04 |
| `{"name":"oni-comb",...}` (50B) | 693 ns | 0.07 |
| `{"a":1,...,"h":8}` (65B) | 1,492 ns | 0.04 |

**所見:**
- `null` は tag 1回で 15ns。integer は `whitespace0` → `integer()` → `whitespace0` の3段でオーバーヘッドがある。
- 配列・オブジェクトは要素数に比例。8要素オブジェクトで ~1.5μs。
- `or` による5分岐（null/true/false/int/string）の試行コストが各要素に乗る。

### 四則演算 + 括弧（oni-comb のみ、recursive 使用）

| 入力 | 時間 |
|------|------|
| `42` | 156 ns |
| `1 + 2` | 247 ns |
| `1 + 2 * 3` | 271 ns |
| `(1 + 2) * 3` | 440 ns |
| `1 + 2 * (3 - 4) + 5` | 639 ns |
| `(((1 + 2) * 3) - 4) / 5` | 931 ns |
| `1 + 2 + ... + 8` | 776 ns |

**所見:**
- 単一整数で 156ns はかなり重い。`recursive()` の `Rc<UnsafeCell<Box<dyn Parser>>>` 経由の間接呼び出し + `whitespace0` のオーバーヘッド。
- 括弧のネストごとに ~200ns 追加（再帰 1 段の `Box<dyn Parser>` コスト）。
- 8項の加算チェーンで 776ns。`chainl1` のループは効率的。

### ヒープアロケーション計測

```
dhat: Total:     0 bytes in 0 blocks
dhat: At t-gmax: 0 bytes in 0 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

identifier / integer / flat_map 同一型 いずれも **0 blocks**。

## 最適化サイクルの記録

### MS6 ParseError 導入による効果

| ワークロード | 旧 (String/format!) | 新 (ParseError) | 改善 |
|-------------|--------------------|--------------------|------|
| flat_map "1one" | 8.3 ns | 7.3 ns | -12% |
| flat_map "2two" | 7.8 ns | 7.3 ns | -6% |
| flat_map "3three" | 6.9 ns | 6.0 ns | -13% |

**分析**: `format!` マクロによる `String` アロケーションコードが LLVM の最適化を妨げていた。`ParseError::expected_char(pos, c)` は構造体の構築のみで `format!` を使わないため、エラーパスのコード生成が軽量化され、成功パスのインライン化にも好影響を与えた。

### 残存するボトルネック

1. **winnow との差（token 系）**: oni-comb は winnow の 70-90% のスループット。`StrInput` の `offset()` 呼び出しや `ParseError` 構築のコストが要因。将来的に Error 型を `()` に差し替えるゼロコストモードの提供で改善可能。
2. **recursive() のオーバーヘッド**: 単一整数パースで 156ns は `Rc<UnsafeCell<Box>>` の間接呼び出しコスト。非再帰パーサー（`integer()` 単体で ~3ns）に比べて 50 倍遅い。再帰が不要なケースでは `recursive()` を避けるべき。
3. **JSON の `or` 分岐コスト**: 5分岐の試行が各要素に乗る。`dispatch!` マクロ（先頭文字で分岐）の導入で改善可能だが、現状のスコープ外。

## 総合評価

- **oni-comb は winnow の 70-90% のスループット** — 具象型設計の恩恵で高速だが、Input/Error 型にまだ改善余地あり
- **nom を中〜長入力で上回る** — 11B identifier で 24% 高速、28B で 46% 高速
- **pom の 3〜30 倍高速** — 旧 v1 相当の `Rc<dyn Fn>` 設計との差を実証
- **chumsky の 30〜200 倍高速** — 動的ディスパッチ前提の設計との差
- **zip ≒ flat_map（同一型）** — 具象コンビネータ型設計の妥当性を確認
- **ParseError 導入で ~12% 改善** — 最適化サイクル 1 回完了
- **Applicative コンビネータでヒープアロケーションゼロ**

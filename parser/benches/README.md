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

## 結果と考察

以下は Apple M 系チップでの計測結果。絶対値は環境依存だが、ライブラリ間の比率は安定する。

### flat_map 同一型分岐（digit → tag）

入力 `"1one"`, `"2two"`, `"3three"` に対し、digit を1文字読んで結果に応じた tag を返す。
全分岐が同一型を返すため、oni-comb では Box 不要。

| ライブラリ | "1one" | "2two" | "3three" |
|-----------|--------|--------|----------|
| nom | 2.4 ns | 2.4 ns | 2.3 ns |
| winnow | 2.6 ns | 2.5 ns | 2.3 ns |
| oni-comb | 8.3 ns | 7.8 ns | 6.9 ns |
| pom | 69 ns | 68 ns | 93 ns |
| chumsky | 924 ns | 922 ns | 983 ns |

**所見:**

- **nom / winnow は ~2.5ns でほぼ同等。** `flat_map` のクロージャ + tag マッチが数命令にインライン化されている。
- **oni-comb は ~8ns で nom/winnow の約 3 倍。** `flat_map` 自体のオーバーヘッドではなく、`tag` のエラーパス内 `format!` による `String` 生成コードと `StrInput` の checkpoint 管理が要因と推測。エラー型を `&'static str` や enum に変更すれば改善が見込める（Milestone 6 で対応予定）。
- **pom は ~70ns（oni-comb の約 9 倍）。** `Vec<char>` への入力変換コストと `Box<dyn Fn>` 経由の間接呼び出しチェーンが主因。
- **chumsky は ~930ns（oni-comb の約 115 倍）。** `then_with` 内で `just(Vec<char>)` を毎回構築するアロケーションコストが支配的。chumsky 0.9 はエラー報告重視の設計であり、hot path 性能はトレードオフ。

### flat_map 異種型分岐（Box\<dyn Parser\> / 動的ディスパッチ）

入力 `"c:hello"`, `"i:42"` に対し、先頭文字に応じて異なる型のパーサーを選択。

| ライブラリ | "c:hello" | "i:42" | 備考 |
|-----------|-----------|--------|------|
| nom | 3.8 ns | 2.7 ns | 手動二段パース（`nom::Parser` が dyn 非互換） |
| winnow | 19.4 ns | 18.8 ns | `Box<dyn Parser>` |
| oni-comb | 21.9 ns | 20.9 ns | `Box<dyn Parser>` |
| pom | 160 ns | 110 ns | 元々全動的ディスパッチ |
| chumsky | 1,139 ns | 1,053 ns | `.boxed()` で型統一 |

**所見:**

- **nom が ~3ns と圧倒的に速いが、不公平な比較。** `nom::Parser` が dyn 非互換のため手動二段パースを採用しており、Box 確保・vtable 間接呼び出しが発生しない。
- **winnow と oni-comb はほぼ同等（~19-22ns）。** 両者とも `Box<dyn Parser>` で動的ディスパッチ。
- **動的ディスパッチのコストは ~15ns。** winnow の同一型（2.5ns）→ boxed（19ns）の差分が Box 確保 + vtable 間接呼び出しのオーバーヘッドに相当。再帰パーサー（Milestone 5）で boxed recursion を使う際の基準値として有用（再帰深度 × 15ns が追加コスト）。

### zip vs flat_map（oni-comb-rs 内部比較）

同じ処理（`satisfy(alpha).zip(take_while0(alnum))` vs `satisfy(alpha).flat_map(|_| take_while0(alnum))`）を比較。

| 入力 | zip | flat_map | 差分 |
|------|-----|----------|------|
| "x" | 4.7 ns | 4.9 ns | +4% |
| "foo" | 10.4 ns | 10.3 ns | -1% (誤差) |
| "foo_bar_123" | 17.5 ns | 17.3 ns | -1% (誤差) |
| "_private" | 14.7 ns | 15.1 ns | +3% |
| "longIdent..." | 30.7 ns | 31.0 ns | +1% (誤差) |

**所見:**

- **zip と flat_map のオーバーヘッド差はほぼゼロ**（統計的有意差なし）。同一型の場合、`FlatMap<Satisfy<F>, G>` が具象型であるため LLVM が zip と同等にインライン化・最適化する。
- **これは具象コンビネータ型設計の成果。** 旧 v1 の `Rc<dyn Fn>` ベースではこの結果は得られない。
- **flat_map を「コストの高い escape hatch」として制限する必要は薄い。** 同一型を返す限り zip と同等性能。

### ヒープアロケーション計測

`alloc_count` ベンチの結果:

```
dhat: Total:     0 bytes in 0 blocks
dhat: At t-gmax: 0 bytes in 0 blocks
dhat: At t-end:  0 bytes in 0 blocks
```

identifier（`zip`）、integer（`take_while1`）、flat_map 同一型（`satisfy` + `tag`）いずれも **0 blocks**。
コンビネータ合成自体はヒープを一切使わない。

## 総合的な示唆

1. **oni-comb の同一型 flat_map は nom/winnow の 3 倍遅い。** ボトルネックは flat_map ではなく `tag` / `satisfy` のエラー生成パス。Milestone 6 でエラー型を改善すれば大幅に縮まる可能性がある。
2. **Box\<dyn Parser\> のコストは ~15ns。** 再帰パーサー設計時の見積もり基準値。
3. **chumsky / pom は構造的に 1-2 桁遅い。** 動的ディスパッチ + アロケーション前提の設計。これらの強みはエラー報告や API の人間工学。
4. **oni-comb の zip ≒ flat_map は設計の妥当性を裏付ける。** 最適化の焦点は combinator 構造ではなく Input / Error 型の効率化に置くべき。

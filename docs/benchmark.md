## ベンチマーク実装プロンプト

```markdown
# oni-comb-rs ベンチマーク: 他OSSパーサーコンビネータとの比較

## 目的

oni-comb-rs (v2) のパース性能を、主要な Rust パーサーコンビネータライブラリと比較するベンチマークを作成する。

## 比較対象ライブラリ

| ライブラリ | 理由 |
|-----------|------|
| **winnow** | 最速クラス。`Parser` trait + `parse_next(&mut I)` で oni-comb-rs と設計が最も近い |
| **nom** | 最古参・最多利用。事実上のデファクト標準 |
| **chumsky** | エラーリカバリ特化。trait ベースのコンビネータで API スタイルが近い |
| **pom** | 演算子オーバーロード中心。旧 oni-comb-rs v1 に近い設計。性能下限の参考 |

pest は PEG コード生成型で比較軸が異なるため除外。combine は winnow に吸収されたため除外。

## ワークロード

以下の 3 つを実装する。それぞれ異なるコンビネータ特性を測る。

### 1. Arithmetic Expression（四則演算+括弧）

```
入力例: "1 + 2 * (3 - 4) / 5"
```

- 測るもの: recursive descent, 優先順位, choice, 数値パース
- oni-comb-rs 側は Milestone 5 (recursive/expression parser) の成果物を使う
- 各ライブラリで **同等のナイーブな実装** にする（最適化テクニックで差を作らない）

### 2. JSON Subset

```json
入力例: {"name": "test", "values": [1, 2, 3], "nested": {"a": true}}
```

- 測るもの: choice + recursive + string escape + number + whitespace skip
- 完全な JSON spec 準拠は不要。object, array, string, number, bool, null をサポート
- 入力データは複数サイズ用意: small (~100B), medium (~10KB), large (~100KB)
- 各ライブラリで **AST 型を共通化** し、構築コストの差を排除

### 3. Identifier/Integer（token hot path）

```
入力例: "foo_bar_123" / "9999999"
```

- 測るもの: 単一トークンパーサーの最小オーバーヘッド
- ループ内で大量の短い入力をパースする形式
- コンビネータ合成のオーバーヘッド（concrete type vs Rc<dyn Fn> vs 関数ポインタ）が直接見える

## ベンチマーク基盤

### Criterion.rs を使用

```toml
# modules/parser/Cargo.toml の [dev-dependencies] に追加
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
nom = "8"
winnow = "0.6"
chumsky = "0.9"
pom = "3"

[[bench]]
name = "comparison"
harness = false
```

### ディレクトリ構成

```
modules/parser/
  benches/
    comparison.rs          # Criterion main
    workloads/
      mod.rs
      arithmetic.rs        # 各ライブラリの arithmetic 実装
      json.rs              # 各ライブラリの JSON 実装
      token.rs             # 各ライブラリの token 実装
    data/
      json_small.json      # ~100B
      json_medium.json     # ~10KB
      json_large.json      # ~100KB
    impls/
      mod.rs
      oni_comb.rs          # oni-comb-rs 実装
      winnow_impl.rs       # winnow 実装
      nom_impl.rs          # nom 実装
      chumsky_impl.rs      # chumsky 実装
      pom_impl.rs          # pom 実装
```

### 各ベンチマーク関数の構成

```rust
// 例: JSON ベンチマーク
fn json_benchmark(c: &mut Criterion) {
    let small = include_str!("data/json_small.json");
    let medium = include_str!("data/json_medium.json");
    let large = include_str!("data/json_large.json");

    let mut group = c.benchmark_group("json");
    
    for (name, input) in [("small", small), ("medium", medium), ("large", large)] {
        group.throughput(Throughput::Bytes(input.len() as u64));
        
        group.bench_with_input(
            BenchmarkId::new("oni-comb", name), input,
            |b, input| b.iter(|| impls::oni_comb::parse_json(black_box(input)))
        );
        group.bench_with_input(
            BenchmarkId::new("winnow", name), input,
            |b, input| b.iter(|| impls::winnow_impl::parse_json(black_box(input)))
        );
        // ... nom, chumsky, pom も同様
    }
    group.finish();
}
```

## 観測項目

| 項目 | 方法 | 目的 |
|------|------|------|
| **Throughput (MB/s)** | Criterion `Throughput::Bytes` | 主要比較指標 |
| **Latency (µs)** | Criterion デフォルト | 絶対速度 |
| **Allocation count** | `dhat-rs` を別 bench binary で | oni-comb-rs の zero-alloc 目標の検証 |

## 実装上の制約

### 公平性の担保

- 各ライブラリで **同じ文法・同じ AST** を構築する
- ライブラリ固有の最適化テクニック（nom の `recognize` 等）は使わない。各ライブラリの「標準的な書き方」で実装
- パース結果の正当性を `#[test]` で全ライブラリ横断で検証する
- 入力データは全ライブラリで共通

### oni-comb-rs 側の注意

- 現時点で未実装のコンビネータ（`sep_by`, `many1`, `satisfy`, `take_while`, `recursive` 等）が必要
- ベンチマーク実装はマイルストーン進捗に合わせて段階的に有効化する
- 未実装のワークロードは `#[ignore]` で保留し、実装完了時に有効化

### 段階的な有効化計画

| マイルストーン完了時 | 有効化するベンチマーク |
|---------------------|---------------------|
| MS3 (Combinators) | token (identifier/integer) |
| MS4 (Text module) | JSON subset |
| MS5 (Recursive) | arithmetic expression |

## 実行コマンド

```bash
# 全ベンチマーク実行
cargo bench -p oni-comb-parser

# 特定ワークロードのみ
cargo bench -p oni-comb-parser -- json
cargo bench -p oni-comb-parser -- arithmetic
cargo bench -p oni-comb-parser -- token

# HTML レポート生成（target/criterion/ 以下）
cargo bench -p oni-comb-parser -- --output-format=criterion

# allocation count（別バイナリ）
cargo bench -p oni-comb-parser --bench alloc_count
```

## 成功基準

- winnow の **50% 以内** の throughput（初期目標。concrete type 化の恩恵で改善余地あり）
- nom と **同等以上** の throughput
- pom を **大幅に上回る**（旧 v1 相当の設計との差を実証）
- oni-comb-rs の **コンビネータ合成・パース実行自体で heap allocation ゼロ**（token ワークロードで検証。結果の String 構築等のユーザーコードは除く）
```

---

プロンプトのポイント：

- **比較対象を4つに絞った**: winnow（最速・設計が近い）、nom（デファクト）、chumsky（trait ベース）、pom（性能下限）
- **ワークロード3種**: それぞれ異なるコンビネータ特性（再帰、choice+文字列、単一トークン）を測定
- **公平性ルール**: 同じ AST、同じ入力、ライブラリ固有の最適化テクニックを使わない
- **段階的有効化**: 現状の未実装コンビネータを考慮し、マイルストーンに連動
# StaticParser Phase 2 移行計画

## 概要

Phase 2の目標は、既存のRcベースのParserの使用箇所をStaticParserに置き換えることです。この移行は段階的に行い、各ステップでテストを実行して機能を検証します。

## 移行対象の特定

以下のファイルでParserが使用されています：

### コアライブラリ内
- parser/src/core/parser.rs
- parser/src/extension/parser/*.rs
- parser/src/internal/parser_impl/*.rs
- parser/src/internal/parsers_impl.rs
- parser/src/lib.rs

### 依存クレート内
- crond/src/cron_parser.rs
- hocon/src/parsers.rs
- toys/src/parsers.rs
- uri/src/parsers/*.rs

### 例とベンチマーク
- parser/examples/*.rs
- parser/benches/*.rs

## 移行戦略

### 1. 移行の優先順位

移行は以下の優先順位で行います：

1. **コアライブラリの基本機能**：
   - parser/src/lib.rs
   - parser/src/internal/parsers_impl.rs

2. **例とベンチマーク**：
   - parser/examples/*.rs
   - parser/benches/*.rs

3. **依存クレート**：
   - crond/src/cron_parser.rs
   - hocon/src/parsers.rs
   - toys/src/parsers.rs
   - uri/src/parsers/*.rs

### 2. 移行アプローチ

#### 2.1 直接置換アプローチ

単純なケースでは、`Parser<'a, I, O>`を`StaticParser<'a, I, O>`に直接置き換えます。

例：
```rust
// 変更前
fn number<'a>() -> Parser<'a, char, f64> {
    // ...
}

// 変更後
fn number<'a>() -> StaticParser<'a, char, f64> {
    // ...
}
```

#### 2.2 段階的移行アプローチ

複雑なケースでは、既存のコードを維持しながら、新しいStaticParser版の関数を追加します。

例：
```rust
// 既存の関数を維持
fn number<'a>() -> Parser<'a, char, f64> {
    // ...
}

// 新しいStaticParser版を追加
fn number_static<'a>() -> StaticParser<'a, char, f64> {
    number().static_parser()
}
```

#### 2.3 互換性レイヤーアプローチ

APIの互換性を維持するために、既存の関数シグネチャを維持しながら内部実装をStaticParserに置き換えます。

例：
```rust
// シグネチャは同じだが、内部実装はStaticParserを使用
fn number<'a>() -> Parser<'a, char, f64> {
    number_static().parser()
}

// 実際の実装はStaticParserを使用
fn number_static<'a>() -> StaticParser<'a, char, f64> {
    // StaticParserを使った実装
}
```

### 3. テスト戦略

各移行ステップで以下のテストを実行します：

1. **単体テスト**：
   ```bash
   cargo test --package oni-comb-parser-rs
   ```

2. **依存クレートのテスト**：
   ```bash
   cargo test --workspace
   ```

3. **ベンチマーク**：
   ```bash
   cargo bench --package oni-comb-parser-rs
   ```

### 4. パフォーマンス検証

移行後のパフォーマンスを検証するために、以下のベンチマークを実行します：

```bash
cargo bench --package oni-comb-parser-rs
```

結果を元のParserと比較して、パフォーマンスの向上を確認します。

## 詳細な移行計画

### フェーズ2.1：コアライブラリの基本機能の移行

#### ステップ1：preludeモジュールの更新

parser/src/lib.rsのpreludeモジュールを更新して、StaticParserを使用するようにします。

```rust
pub mod prelude {
    // 既存のエクスポート
    pub use crate::core::Parser;
    
    // StaticParserのエクスポートを追加
    pub use crate::core::StaticParser;
    
    // 既存の関数をStaticParserバージョンに置き換える
    // または、両方をエクスポートする
}
```

#### ステップ2：基本パーサー関数の移行

parser/src/internal/parsers_impl.rsの基本パーサー関数をStaticParserを使用するように更新します。

```rust
// 変更前
pub fn elm<'a, I: 'a + PartialEq + Debug + Clone>(e: I) -> Parser<'a, I, I> {
    // ...
}

// 変更後
pub fn elm<'a, I: 'a + PartialEq + Debug + Clone>(e: I) -> StaticParser<'a, I, I> {
    // ...
}

// または互換性を維持する場合
pub fn elm<'a, I: 'a + PartialEq + Debug + Clone>(e: I) -> Parser<'a, I, I> {
    elm_static(e).parser()
}

pub fn elm_static<'a, I: 'a + PartialEq + Debug + Clone>(e: I) -> StaticParser<'a, I, I> {
    // StaticParserを使った実装
}
```

### フェーズ2.2：例とベンチマークの移行

#### ステップ1：単純な例の移行

parser/examples/hello_world.rsなどの単純な例をStaticParserを使用するように更新します。

#### ステップ2：複雑な例の移行

parser/examples/json_char.rsなどの複雑な例をStaticParserを使用するように更新します。

#### ステップ3：ベンチマークの移行

parser/benches/oni_comb_json.rsなどのベンチマークをStaticParserを使用するように更新します。

### フェーズ2.3：依存クレートの移行

#### ステップ1：crondクレートの移行

crond/src/cron_parser.rsをStaticParserを使用するように更新します。

#### ステップ2：hoconクレートの移行

hocon/src/parsers.rsをStaticParserを使用するように更新します。

#### ステップ3：toysクレートの移行

toys/src/parsers.rsをStaticParserを使用するように更新します。

#### ステップ4：uriクレートの移行

uri/src/parsers/*.rsをStaticParserを使用するように更新します。

## 移行のリスクと対策

### リスク1：APIの互換性の問題

**対策**：互換性レイヤーアプローチを使用して、既存のAPIを維持しながら内部実装をStaticParserに置き換えます。

### リスク2：パフォーマンスの低下

**対策**：各移行ステップでベンチマークを実行して、パフォーマンスの低下がないことを確認します。

### リスク3：テストの失敗

**対策**：各移行ステップでテストを実行して、機能の正確性を確認します。

## タイムライン

### フェーズ2.1：コアライブラリの基本機能の移行
- 予想期間：2週間

### フェーズ2.2：例とベンチマークの移行
- 予想期間：1週間

### フェーズ2.3：依存クレートの移行
- 予想期間：2週間

## 結論

Phase 2の移行計画は、既存のParserの使用箇所をStaticParserに段階的に置き換えることを目的としています。この移行は、APIの互換性を維持しながら、パフォーマンスの向上を実現するために慎重に行われます。

各移行ステップでテストとベンチマークを実行して、機能の正確性とパフォーマンスの向上を確認します。最終的には、すべてのParserの使用箇所がStaticParserに置き換えられ、より高速なパーサーライブラリが実現されます。

# Technology Stack

## Architecture

Cargo ワークスペースによるモノレポ構成。`parser` クレートが共通基盤を提供し、ドメイン別クレートがそれに依存する一方向の依存関係。

```
parser (oni-comb-parser-rs)  ← 共通基盤
  ↑
  ├── uri   (oni-comb-uri-rs)
  ├── hocon (oni-comb-hocon-rs)
  ├── crond (oni-comb-crond-rs)
  └── toys  (実験用)
```

## Core Technologies

- **Language**: Rust (Edition 2021)
- **Toolchain**: Nightly (rustfmt + clippy 同梱、`rust-toolchain.toml` で固定)
- **Build System**: Cargo workspace (resolver v2)

## Key Libraries

- **log**: ロギング統一クレート
- **regex**: 正規表現パーサー用
- **fnv**: 高速ハッシュマップ（パーサーキャッシュ）
- **prop-check-rs**: プロパティベーステスト（dev-dependency）
- **criterion**: ベンチマーク（dev-dependency）

## Development Standards

### Code Quality

- `clippy::pedantic` を opt-in する場合は `allow` 理由をコメントで残す
- `#![warn(dead_code)]` をライブラリルートで有効化
- `cargo +nightly fmt` による統一整形

### Testing

- `cargo test --workspace --all-targets` を最小要件とする
- パーサー追加時は成功ケース・失敗ケース・プロパティテスト (prop-check-rs) の 3 点セット
- ベンチマークは Criterion で `parser/benches/` に配置

## Development Environment

### Required Tools

- Rust nightly toolchain (`rust-toolchain.toml` で自動選択)
- Renovate による依存更新管理

### Common Commands

```bash
cargo +nightly fmt                     # 整形
cargo build --workspace --all-features # 全クレートビルド
cargo test --workspace --all-targets   # テスト
cargo bench -p oni-comb-parser-rs      # ベンチマーク
cargo clippy --workspace --all-targets -- -D warnings  # lint
```

## Key Technical Decisions

- **`Rc<dyn Fn>` ベースのパーサー**: `Parser` の内部は `Rc<dyn Fn(&ParseState) -> ParseResult>` で、クローン可能かつ合成可能
- **関数型パターンの採用**: `ParserFunctor` (map), `ParserMonad` (flat_map), `ParserPure` (successful) をトレイトとして分離
- **参照版と値版の二重API**: `elm_ref()` / `elm()` のように、参照返却と値返却の両方を提供
- **CommittedStatus による制御**: バックトラック可否を `Committed` / `Uncommitted` で明示管理

---
_Document standards and patterns, not every dependency_

# Repository Guidelines

- 日本語でやりとりしてください

## プロジェクト構成とモジュール配置
ワークスペース全体は Rust 2021 edition を対象とし、`parser/` が共通の LL(k) パーサー基盤を提供します。ドメイン別のクレートは `crond/` (Cron 式)、`uri/` (URI 構文)、`hocon/` (設定ファイル)、`toys/` (実験用サンプル) に分かれ、ベンチマーク関連は `benchmark_results/` と `run-benchmark.sh` にまとまっています。共有ドキュメントは `docs/`、クロスカットな仕様メモは `docs/common/` 以下に配置してください。各クレートの `src/` が実装、`tests/` が統合テスト、`examples/` が学習用スニペットの配置場所です。

## ビルド・テスト・ローカル開発
代表的なコマンドは以下の通りです。
```bash
cargo +nightly fmt                     # rustfmt による整形
cargo build --workspace --all-features # 全クレートビルド
cargo test --workspace --all-targets   # 単体 + 統合テスト
cargo bench -p oni-comb-parser-rs       # Criterion ベンチ
cargo run -p oni-comb-parser-rs --example hello_world
```
`cargo make fmt` もサポートしているため CI と同じ整形フローをローカルで再現できます。ベンチマーク結果は `target/criterion/` に出力されるので、比較時は `git update-index --skip-worktree` を活用して差分を無視します。

## コーディングスタイルと命名
`rust-toolchain.toml` で nightly toolchain と `rustfmt` / `clippy` を固定しています。インデントは 2 スペース、`max_width = 120`、改行コードは LF。ファイル・モジュール名は `snake_case`、型・トレイトは `PascalCase`、関数と境界値チェック用ヘルパーは `snake_case` で動詞始まりとし、明示的なライフタイムや複雑なジェネリクスには `///` ドキュメントコメントを付与してください。ロガーは `log` クレートで統一し、`clippy::pedantic` を opt-in する場合は必要な `allow` 理由をコメントで残します。

## テストと品質保証
テストは `cargo test --workspace` を最小要件とし、パーサー追加時は成功ケース・失敗ケース・プロパティケース (prop-check-rs) の 3 点セットを追加します。ベンチマークを更新する際は `cargo bench -p oni-comb-parser-rs` 実行後、`run-benchmark.sh` で HTML レポートを再生成し、`docs/` へ要約をリンクします。CI 落ちを防ぐため `RUST_LOG=debug cargo test -p <crate> -- --nocapture` を使いログを確認してください。カバレッジ測定が必要な場合は `cargo llvm-cov --workspace --text` を推奨します。

## コミットとプルリクエスト
履歴は `refactor: tidy parser error path` のように Conventional Commits 形式で統一されています。件名は 72 文字以内、本文には背景・変更点・影響範囲を短文で記載し、関連 Issue や議論スレッドを `Refs:` で紐付けます。PR テンプレートがないため、説明欄に 1) Motivation、2) Changes、3) Testing を見出し付きで記述し、スクリーンショットまたはログを必要に応じて添付してください。レビュー前に `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` を走らせ、CI の再実行が不要な状態で提出します。

## セキュリティ・設定メモ
機微情報は `.gitignore` に列挙済みの `.env` や資格情報ファイルに保存し、ダンプをコミットしないでください。ツールチェーン更新は `rustup toolchain install nightly --profile minimal` を利用し、依存更新は Renovate PR を採用するか `cargo update -p <crate>` で局所的に行います。設定例は `hocon/tests/data/` を参照し、外部 API キーは `std::env::var` 経由で注入する構成を守ってください。

## エージェント向け運用ヒント
自動生成タスクでは `parser/benches/` の diff が大きくなりがちなので、作業前に `git checkout -- parser/benches/*.svg` など生成物を破棄してから開始するとレビューが容易になります。大量テキストを変更する場合は `rg --context 2 <keyword>` で影響範囲を把握し、`cargo fmt` の前後で `git diff --stat` を確認して不要なリファクタリングが混入していないかチェックしてください。

# 再実装計画 (Reboot ブランチ)

## 概要
- 現在の `reboot` ブランチは旧実装をすべて削除し、`API_SPEC.md` のみに外部仕様を残した状態から再構築をスタートしています。
- 最小限のワークスペースと `parser` クレートを整備済み（`Parser`, `ParseState`, `ParseResult`, `ParseError`, `CommittedStatus` を含む）。
- 今後は仕様に沿って、必要なコンビネータ・DSL 構造・ユーティリティを段階的に実装していく方針です。

## 現在の進捗
- ✅ `API_SPEC.md` に旧 `prelude` を中心とした公開 API を整理。
- ✅ `parser` クレートの skeleton を構築。
  - `core` モジュール: `Parser`, `ParseState`, `ParseResult`, `ParseError`, `CommittedStatus`
  - `prelude` モジュール: `unit`, `successful`, `failed`, `begin`, `end` などの基本 API
- ✅ `ParseResult` / `ParseState` は将来の借用ベース化に対応しやすい形でプレースホルダー実装済み。

## TODO（今後の実装ステップ）
1. **ParseResult API の仕上げ**
   - `ParseResult::flat_map` など複合メソッドの互換実装
   - 成功時に残り入力を扱う補助メソッド（`rest()`, `with_state` など）の拡充
2. **基本コンビネータの実装**
   - `map`, `flat_map`, `filter`, `attempt`, `exists`, `not` 等を `core` 層または `internal` 層に実装
   - `skip_left`, `skip_right`, `surround`, `many0`, `many1` など旧 API に相当する機能を段階的に追加
3. **文字／トークン処理ユーティリティ**
   - `elm`, `seq`, `take_while`, `regex` などの基礎パーサー
   - 必要に応じて trait や helper struct を導入
4. **値型の見直し**
   - 旧実装で使用していた `ParseError` のメッセージ構造や、コミット制御の API 整備
   - 入力を `&str` / `&[u8]` で扱うための補助
5. **テスト・サンプルの復元**
   - ユニットテスト／ドキュメントテストで API をカバー
   - 旧クレートの代表的なサンプル（JSON パーサーなど）を新アーキテクチャで実装しなおす
6. **ベンチマーク整備**（任意）
   - `criterion` を使って基本パーサーの性能測定を再構築
   - 新実装のボトルネックを追跡

進捗に応じて PLAN.md を更新し、細分化されたタスクを管理していく想定です。

## 1. クレートセットアップ

- [x] 1.1 `modules/uri/Cargo.toml` 作成（`oni-comb-parser` + `proptest` dev-dep）
- [x] 1.2 ワークスペース `Cargo.toml` に `modules/uri` をメンバー追加
- [x] 1.3 `modules/uri/src/lib.rs` 作成（モジュール宣言）
- [x] 1.4 `cargo build -p oni-comb-uri` が通ること

## 2. モデル定義

- [x] 2.1 `models/uri.rs`: `Uri<'a>` 構造体
- [x] 2.2 `models/authority.rs`: `Authority<'a>` 構造体
- [x] 2.3 `models/host.rs`: `Host<'a>` enum（RegName, Ipv4, Ipv6, IpvFuture）
- [x] 2.4 `models/user_info.rs`: `UserInfo<'a>` 構造体
- [x] 2.5 `models/path.rs`: `Path<'a>` enum（Abempty, Absolute, Rootless, NoScheme, Empty）+ `segments()`
- [x] 2.6 `models/query.rs`: `Query<'a>` 構造体（raw + params）
- [x] 2.7 全モデルに `Display` trait 実装

## 3. 共通パーサー

- [x] 3.1 `parsers/common.rs`: `unreserved`, `pct_encoded`, `sub_delims`, `pchar`

## 4. Scheme パーサー

- [x] 4.1 `parsers/scheme.rs`: scheme パーサー
- [x] 4.2 scheme ユニットテスト

## 5. Authority パーサー

- [x] 5.1 `parsers/host.rs`: reg-name パーサー
- [x] 5.2 `parsers/ipv4.rs`: IPv4 パーサー（dec-octet × 4）
- [x] 5.3 `parsers/ipv6.rs`: IPv6 パーサー（9 パターン + h16 + ls32）
- [x] 5.4 `parsers/host.rs`: host パーサー（IP-literal / IPv4 / reg-name 統合）
- [x] 5.5 `parsers/authority.rs`: userinfo + host + port 統合
- [x] 5.6 IPv4 / IPv6 / authority ユニットテスト

## 6. Path パーサー

- [x] 6.1 `parsers/path.rs`: segment, path-abempty, path-absolute, path-rootless, path-noscheme, path-empty
- [x] 6.2 path ユニットテスト

## 7. Query / Fragment パーサー

- [x] 7.1 `parsers/query.rs`: query パーサー（生文字列 + key-value 分解）
- [x] 7.2 `parsers/fragment.rs`: fragment パーサー
- [x] 7.3 query / fragment ユニットテスト

## 8. URI パーサー統合

- [x] 8.1 `parsers/uri.rs`: フル URI パーサー（scheme ":" hier-part ["?" query] ["#" fragment]）
- [x] 8.2 `Uri::parse()` 公開 API
- [x] 8.3 URI パーサーユニットテスト（基本ケース）

## 9. URN サポート

- [x] 9.1 `urn.rs` または `models/uri.rs` に `is_urn()`, `urn_nid()`, `urn_nss()` 実装
- [x] 9.2 URN ユニットテスト

## 10. proptest Property-Based テスト

- [x] 10.1 `tests/proptest_strategies.rs`: 共通 Strategy（unreserved, pct_encoded, pchar 等）
- [x] 10.2 scheme Strategy + round-trip テスト
- [x] 10.3 IPv4 Strategy + round-trip テスト
- [x] 10.4 IPv6 Strategy + round-trip テスト
- [x] 10.5 host Strategy + round-trip テスト
- [x] 10.6 authority Strategy + round-trip テスト
- [x] 10.7 path Strategy + round-trip テスト
- [x] 10.8 query Strategy + round-trip テスト
- [x] 10.9 URI 全体 Strategy + round-trip テスト

## 11. CI / ドキュメント

- [x] 11.1 `.github/workflows/ci.yml`: lint/test に `-p oni-comb-uri` 追加
- [x] 11.2 `.github/workflows/publish.yml`: `oni-comb-uri-v*` タグ対応
- [x] 11.3 `.github/workflows/bump-version.yml`: `oni-comb-uri` 選択肢追加
- [x] 11.4 `modules/uri/README.md` + `README.ja.md`

## 12. 最終検証

- [x] 12.1 `cargo test -p oni-comb-uri` 全テスト通過
- [x] 12.2 `RUSTFLAGS="-D warnings" cargo clippy -p oni-comb-uri` 通過
- [x] 12.3 `cargo fmt -- --check` 通過

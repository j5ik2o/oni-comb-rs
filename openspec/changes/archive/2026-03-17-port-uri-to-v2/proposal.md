## Why

oni-comb-rs v1 には RFC 3986 準拠の URI パーサークレートがあったが、v2 にはまだ移植されていない。v2 の `StrInput<'a>` + 具象コンビネータ型 API でゼロコピー URI パーサーを再実装する。また、ユーザーから「URN サポートはあるか」という issue を受けており、URN 判定 API（Level 1: `is_urn()`, `urn_nid()`, `urn_nss()`）も追加する。

## What Changes

- `uri` クレートを workspace メンバーとして新規追加（`modules/uri/`）
- RFC 3986 準拠の URI パーサーをゼロコピー（`&'a str`）で実装
  - scheme, authority（userinfo, host, port）, path, query, fragment
  - host: reg-name, IPv4, IPv6（RFC 3986 の 9 パターン全て）, IPvFuture
  - query: パース時に `&`/`=` で分解（`Vec<(&'a str, Option<&'a str>)>`）
  - path: abempty / absolute / rootless / noscheme / empty
- URN 判定 API: `uri.is_urn()`, `uri.urn_nid()`, `uri.urn_nss()`
- proptest によるプロパティベーステスト（v1 の prop-check-rs gens を参考に proptest Strategy で再実装）

## Capabilities

### New Capabilities
- `uri-parser`: RFC 3986 URI パーサー。scheme, authority, path, query, fragment の完全パース。IPv4/IPv6/IPvFuture 対応
- `uri-models`: ゼロコピー URI モデル（`Uri<'a>`, `Authority<'a>`, `Host<'a>`, `Query<'a>` 等）
- `urn-support`: URN 判定 API（`is_urn()`, `urn_nid()`, `urn_nss()`）。scheme="urn" 時に path-rootless を NID:NSS に分解

### Modified Capabilities

（なし）

## Impact

### ファイル影響
- `Cargo.toml`（workspace）: `modules/uri` をメンバーに追加
- `modules/uri/`: 新規クレート一式（`Cargo.toml`, `src/`, `tests/`）
- `.github/workflows/ci.yml`: lint/test に `-p oni-comb-uri` 追加
- `.github/workflows/publish.yml`: `oni-comb-uri-v*` タグ対応
- `.github/workflows/bump-version.yml`: `oni-comb-uri` 選択肢追加

### 依存
- `oni-comb-parser`（workspace 内パス参照）
- `proptest`（dev-dependencies）

### API
- `Uri::parse("https://user:pass@example.com:8080/path?q=1#frag")` → `Result<Uri<'_>, String>`
- `uri.scheme()`, `uri.authority()`, `uri.host()`, `uri.port()`, `uri.path()`, `uri.query()`, `uri.fragment()`
- `uri.query_params()` → `&[(&str, Option<&str>)]`
- `uri.is_urn()`, `uri.urn_nid()`, `uri.urn_nss()`

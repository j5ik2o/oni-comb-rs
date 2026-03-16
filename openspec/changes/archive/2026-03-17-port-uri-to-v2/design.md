# Port URI to v2 — 設計

## Context

v1 URI クレートは RFC 3986 準拠で `u8` スライス + v1 API（`+`/`-`/`*` 演算子、`.cache()`）で実装されていた。全フィールドが `String` にコピーされる設計。v2 では `StrInput<'a>` + ゼロコピー（`&'a str`）で再実装する。

v1 には prop-check-rs による property-based テストが充実していた（各パーサーに Gen + round-trip テスト）。v2 では proptest に移行し、同等の Strategy を再構築する。

## Goals / Non-Goals

**Goals:**
- RFC 3986 URI の完全パース（scheme, authority, path, query, fragment）
- IPv4 / IPv6（9 パターン全て）/ IPvFuture のホストパース
- ゼロコピーモデル（`Uri<'a>` で `&'a str` 参照を保持）
- URN Level 1 サポート（`is_urn()`, `urn_nid()`, `urn_nss()`）
- proptest による property-based テスト（v1 gens 相当の Strategy）
- `Display` trait による round-trip（パース → 文字列化 → 元と一致）

**Non-Goals:**
- RFC 8141 完全準拠（URN r-component, q-component 等）
- IRI（国際化 URI, RFC 3987）
- URI の正規化・解決（RFC 3986 Section 5）
- URL エンコーディング/デコーディングのユーティリティ関数

## Decisions

### 1. ゼロコピーモデル

```rust
pub struct Uri<'a> {
    scheme: Option<&'a str>,
    authority: Option<Authority<'a>>,
    path: Path<'a>,
    query: Option<Query<'a>>,
    fragment: Option<&'a str>,
}

pub struct Authority<'a> {
    user_info: Option<UserInfo<'a>>,
    host: Host<'a>,
    port: Option<u16>,
}

pub struct UserInfo<'a> {
    user_name: &'a str,
    password: Option<&'a str>,
}

pub enum Host<'a> {
    RegName(&'a str),
    Ipv4(std::net::Ipv4Addr),
    Ipv6(std::net::Ipv6Addr),
    IpvFuture(&'a str),
}

pub enum Path<'a> {
    Abempty(Vec<&'a str>),   // /seg1/seg2 (authority 後)
    Absolute(Vec<&'a str>),  // /seg1/seg2 (authority なし)
    Rootless(Vec<&'a str>),  // seg1/seg2
    NoScheme(Vec<&'a str>),  // seg1/seg2 (scheme なし)
    Empty,
}

pub struct Query<'a> {
    raw: &'a str,
    params: Vec<(&'a str, Option<&'a str>)>,
}
```

**v1 との差分:**
- `String` → `&'a str` でゼロコピー
- `Ipv4Addr` / `Ipv6Addr` は値型でそのまま保持
- `Path` の `type_name` フィールドを削除（enum variant 自体が型情報）
- `Query` は `raw` 生文字列 + `params` 分解済みの両方を保持
- `HierPart` は廃止（`Uri` に `authority` と `path` を直接保持）

### 2. パーサー構成

v1 の `u8` ベースパーサーを `StrInput<'a>` ベースに書き換え。`fn_parser` + `satisfy` + `take_while` + `tag` が主要部品。

```
parsers/
├── common.rs     ── pchar, pct_encoded, unreserved, sub_delims 等
├── scheme.rs     ── ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )
├── authority.rs  ── [userinfo "@"] host [":" port]
├── host.rs       ── IP-literal / IPv4address / reg-name
├── ipv4.rs       ── dec-octet "." dec-octet "." dec-octet "." dec-octet
├── ipv6.rs       ── 9 パターン（RFC 3986 Appendix A）
├── path.rs       ── path-abempty / path-absolute / path-rootless / etc.
├── query.rs      ── *( pchar / "/" / "?" ) + "&"/"=" 分解
├── fragment.rs   ── *( pchar / "/" / "?" )
└── uri.rs        ── scheme ":" hier-part ["?" query] ["#" fragment]
```

### 3. URN サポート（Level 1）

```rust
impl<'a> Uri<'a> {
    pub fn is_urn(&self) -> bool {
        self.scheme.map(|s| s.eq_ignore_ascii_case("urn")).unwrap_or(false)
    }

    pub fn urn_nid(&self) -> Option<&'a str> {
        if !self.is_urn() { return None; }
        // path-rootless の最初の ":" までが NID
        self.path.as_str().split_once(':').map(|(nid, _)| nid)
    }

    pub fn urn_nss(&self) -> Option<&'a str> {
        if !self.is_urn() { return None; }
        self.path.as_str().split_once(':').map(|(_, nss)| nss)
    }
}
```

パーサーレベルでは URN 固有のパースは不要。RFC 3986 の `path-rootless` として自然にパースされ、URN 固有の分解はモデル側のメソッドで行う。

### 4. proptest Strategy

v1 の `Gen` ベースの gens を proptest `Strategy` で再構築:

```rust
fn scheme_strategy() -> impl Strategy<Value = String> {
    "[a-zA-Z][a-zA-Z0-9+\\-.]{0,10}"
}

fn ipv4_strategy() -> impl Strategy<Value = String> {
    (0u8..=255, 0u8..=255, 0u8..=255, 0u8..=255)
        .prop_map(|(a, b, c, d)| format!("{}.{}.{}.{}", a, b, c, d))
}

fn uri_strategy() -> impl Strategy<Value = String> {
    // scheme, authority, path, query, fragment を合成
}
```

round-trip テスト: `uri_strategy()` で文字列生成 → `Uri::parse()` → `uri.to_string()` → 元と一致

### 5. IPv6 パーサー

RFC 3986 の 9 パターンを全てサポート。v1 は 590 行だったが、v2 では `StrInput` の `take_while` + `fn_parser` で簡潔化（推定 ~400 行）。

```
IPv6address =                            6( h16 ":" ) ls32
            /                       "::" 5( h16 ":" ) ls32
            / [               h16 ] "::" 4( h16 ":" ) ls32
            / [ *1( h16 ":" ) h16 ] "::" 3( h16 ":" ) ls32
            / [ *2( h16 ":" ) h16 ] "::" 2( h16 ":" ) ls32
            / [ *3( h16 ":" ) h16 ] "::"    h16 ":"   ls32
            / [ *4( h16 ":" ) h16 ] "::"              ls32
            / [ *5( h16 ":" ) h16 ] "::"              h16
            / [ *6( h16 ":" ) h16 ] "::"
```

## Risks / Trade-offs

- **IPv6 パーサーの複雑さ**: 9 パターンの組み合わせは最も実装コストが高い部分。ただし v1 の実装が参考になる
- **ゼロコピーのライフタイム伝播**: `Uri<'a>` のライフタイムが API 全体に伝播するが、URI パーサーの使い方（パース → 即使用）では問題にならない
- **Query の Vec アロケーション**: パース時に `Vec<(&'a str, Option<&'a str>)>` を構築するため、完全なゼロアロケーションではない。ただし文字列自体はゼロコピー
- **proptest の Strategy 構築コスト**: v1 の gens を参考にするが、proptest API に合わせた書き直しが必要

## モジュール構成

```
modules/uri/
├── Cargo.toml
├── README.md
├── README.ja.md
└── src/
    ├── lib.rs
    ├── models/
    │   ├── mod.rs
    │   ├── uri.rs
    │   ├── authority.rs
    │   ├── host.rs
    │   ├── path.rs
    │   ├── query.rs
    │   └── user_info.rs
    ├── parsers/
    │   ├── mod.rs
    │   ├── common.rs
    │   ├── scheme.rs
    │   ├── authority.rs
    │   ├── host.rs
    │   ├── ipv4.rs
    │   ├── ipv6.rs
    │   ├── path.rs
    │   ├── query.rs
    │   ├── fragment.rs
    │   └── uri.rs
    └── urn.rs            ── is_urn(), urn_nid(), urn_nss()
```

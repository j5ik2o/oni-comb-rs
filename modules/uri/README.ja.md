# oni-comb-uri

[oni-comb-parser](../parser/) v2 コンビネータ API で構築したゼロコピー RFC 3986 URI パーサー。URN サポート付き。

## 特徴

- **RFC 3986 準拠** — URI の完全パース（scheme, authority, path, query, fragment）
- **ゼロコピー** — `Uri<'a>` は入力 `&str` を借用し、文字列フィールドのアロケーションなし
- **IPv4 / IPv6 / IPvFuture** — 完全なホスト型サポート
- **Query 分解** — パース時に key-value パラメータを分解
- **URN サポート** — `urn:` スキーム URI に対して `is_urn()`, `urn_nid()`, `urn_nss()`
- **プロパティベーステスト** — proptest Strategy による round-trip 検証

## クイックスタート

```rust
use oni_comb_uri::Uri;

let uri = Uri::parse("http://user:pass@example.com:8080/path?key=value#frag").unwrap();

assert_eq!(uri.scheme(), Some("http"));
assert_eq!(uri.port(), Some(8080));
assert_eq!(uri.path().to_string(), "/path");
assert_eq!(uri.query_params(), &[("key", Some("value"))]);
assert_eq!(uri.fragment(), Some("frag"));
assert_eq!(uri.to_string(), "http://user:pass@example.com:8080/path?key=value#frag");

// URN サポート
let urn = Uri::parse("urn:isbn:0451450523").unwrap();
assert!(urn.is_urn());
assert_eq!(urn.urn_nid(), Some("isbn"));
assert_eq!(urn.urn_nss(), Some("0451450523"));
```

## ビルド・テスト

```bash
cargo build -p oni-comb-uri
cargo test -p oni-comb-uri
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

# oni-comb-uri

[日本語](README.ja.md)

A zero-copy RFC 3986 URI parser built on [oni-comb-parser](../parser/) v2 combinator API, with URN support.

## Features

- **RFC 3986 compliant** — full URI parsing (scheme, authority, path, query, fragment)
- **Zero-copy** — `Uri<'a>` borrows from input `&str`, no string allocation for parsed fields
- **IPv4 / IPv6 / IPvFuture** — complete host type support
- **Query decomposition** — key-value params parsed at parse time
- **URN support** — `is_urn()`, `urn_nid()`, `urn_nss()` for `urn:` scheme URIs
- **Property-based testing** — proptest strategies for round-trip verification

## Quickstart

```rust
use oni_comb_uri::Uri;

let uri = Uri::parse("http://user:pass@example.com:8080/path?key=value#frag").unwrap();

assert_eq!(uri.scheme(), Some("http"));
assert_eq!(uri.port(), Some(8080));
assert_eq!(uri.path().to_string(), "/path");
assert_eq!(uri.query_params(), &[("key", Some("value"))]);
assert_eq!(uri.fragment(), Some("frag"));
assert_eq!(uri.to_string(), "http://user:pass@example.com:8080/path?key=value#frag");

// URN support
let urn = Uri::parse("urn:isbn:0451450523").unwrap();
assert!(urn.is_urn());
assert_eq!(urn.urn_nid(), Some("isbn"));
assert_eq!(urn.urn_nss(), Some("0451450523"));
```

## Build & Test

```bash
cargo build -p oni-comb-uri
cargo test -p oni-comb-uri
```

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../../LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](../../LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

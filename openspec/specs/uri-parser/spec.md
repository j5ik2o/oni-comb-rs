## ADDED Requirements

### Requirement: Parse scheme
パーサーは RFC 3986 の scheme（`ALPHA *( ALPHA / DIGIT / "+" / "-" / "." )`）をパースしなければならない（SHALL）。

#### Scenario: Parse "http"
- **WHEN** `"http://example.com"` をパースする
- **THEN** scheme は `"http"` を返す

#### Scenario: Parse "ftp+ssl"
- **WHEN** `"ftp+ssl://host"` をパースする
- **THEN** scheme は `"ftp+ssl"` を返す

### Requirement: Parse authority with host, port, userinfo
パーサーは `[userinfo "@"] host [":" port]` 形式の authority をパースしなければならない（SHALL）。

#### Scenario: Full authority
- **WHEN** `"http://user:pass@example.com:8080/path"` をパースする
- **THEN** user_name=`"user"`, password=`Some("pass")`, host=RegName(`"example.com"`), port=`Some(8080)`

#### Scenario: Host only
- **WHEN** `"http://localhost/path"` をパースする
- **THEN** host=RegName(`"localhost"`), port=`None`, user_info=`None`

### Requirement: Parse IPv4 host
パーサーは IPv4 アドレス（`dec-octet "." dec-octet "." dec-octet "." dec-octet`）をパースしなければならない（SHALL）。

#### Scenario: Parse "192.168.1.1"
- **WHEN** `"http://192.168.1.1/"` をパースする
- **THEN** host は `Host::Ipv4(Ipv4Addr::new(192, 168, 1, 1))`

### Requirement: Parse IPv6 host
パーサーは RFC 3986 の IPv6 アドレス 9 パターン全てをパースしなければならない（SHALL）。

#### Scenario: Parse full IPv6
- **WHEN** `"http://[2001:db8::1]/"` をパースする
- **THEN** host は `Host::Ipv6` で対応する `Ipv6Addr`

#### Scenario: Parse IPv6 with embedded IPv4
- **WHEN** `"http://[::ffff:192.168.1.1]/"` をパースする
- **THEN** host は `Host::Ipv6` で対応する `Ipv6Addr`

### Requirement: Parse IPvFuture host
パーサーは `"v" 1*HEXDIG "." 1*( unreserved / sub-delims / ":" )` をパースしなければならない（SHALL）。

#### Scenario: Parse IPvFuture
- **WHEN** `"http://[v1.test]/"` をパースする
- **THEN** host は `Host::IpvFuture("v1.test")`

### Requirement: Parse path variants
パーサーは path-abempty, path-absolute, path-rootless, path-noscheme, path-empty の全 5 種をパースしなければならない（SHALL）。

#### Scenario: path-abempty
- **WHEN** `"http://host/a/b/c"` をパースする
- **THEN** path は `Path::Abempty(["", "a", "b", "c"])`

#### Scenario: path-rootless
- **WHEN** `"mailto:user@example.com"` をパースする
- **THEN** path は `Path::Rootless(["user@example.com"])`

#### Scenario: path-empty
- **WHEN** `"http://host"` をパースする
- **THEN** path は `Path::Empty`

### Requirement: Parse query with key-value decomposition
パーサーは `?` 以降の query 文字列をパースし、`&` と `=` で key-value ペアに分解しなければならない（SHALL）。

#### Scenario: Multiple params
- **WHEN** `"http://host?k1=v1&k2=v2"` をパースする
- **THEN** query.params は `[("k1", Some("v1")), ("k2", Some("v2"))]`

#### Scenario: Key without value
- **WHEN** `"http://host?flag&k=v"` をパースする
- **THEN** query.params は `[("flag", None), ("k", Some("v"))]`

### Requirement: Parse fragment
パーサーは `#` 以降の fragment をパースしなければならない（SHALL）。

#### Scenario: Fragment
- **WHEN** `"http://host/path#section1"` をパースする
- **THEN** fragment は `Some("section1")`

### Requirement: Parse full URI
パーサーは `scheme ":" hier-part [ "?" query ] [ "#" fragment ]` の完全な URI をパースしなければならない（SHALL）。

#### Scenario: Full URI
- **WHEN** `"http://user:pass@localhost:8080/example?key=value#frag"` をパースする
- **THEN** 全フィールドが正しくパースされる

#### Scenario: Reject invalid URI
- **WHEN** 不正な文字列をパースする
- **THEN** エラーを返す

### Requirement: Round-trip via Display
`Uri::parse(s).to_string()` は元の文字列 `s` と一致しなければならない（SHALL）。

#### Scenario: proptest round-trip
- **WHEN** proptest Strategy で生成した任意の有効 URI 文字列 `s` に対して
- **THEN** `Uri::parse(s).unwrap().to_string() == s`

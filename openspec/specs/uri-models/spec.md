## ADDED Requirements

### Requirement: Zero-copy URI model
`Uri<'a>` は入力文字列を借用し、文字列フィールドは `&'a str` で保持しなければならない（SHALL）。

#### Scenario: No string allocation for scheme/path/fragment
- **WHEN** `Uri::parse("http://host/path#frag")` を呼ぶ
- **THEN** `scheme`, `path` 内のセグメント文字列, `fragment` は入力 `&str` のスライス

### Requirement: Host enum with typed IP addresses
`Host<'a>` は `RegName(&'a str)`, `Ipv4(Ipv4Addr)`, `Ipv6(Ipv6Addr)`, `IpvFuture(&'a str)` を区別しなければならない（SHALL）。

#### Scenario: RegName host
- **WHEN** host が `"example.com"` のとき
- **THEN** `Host::RegName("example.com")`

#### Scenario: IPv4 host
- **WHEN** host が `"127.0.0.1"` のとき
- **THEN** `Host::Ipv4(Ipv4Addr::new(127, 0, 0, 1))`

### Requirement: Query provides raw string and decomposed params
`Query<'a>` は生文字列（`raw: &'a str`）と分解済みパラメータ（`params: Vec<(&'a str, Option<&'a str>)>`）の両方を保持しなければならない（SHALL）。

#### Scenario: Access raw query
- **WHEN** `"http://host?a=1&b=2"` をパースする
- **THEN** `query.raw()` は `"a=1&b=2"` を返す

#### Scenario: Access params
- **WHEN** `"http://host?a=1&b=2"` をパースする
- **THEN** `query.params()` は `[("a", Some("1")), ("b", Some("2"))]` を返す

### Requirement: Display trait for round-trip
全モデル型は `Display` を実装し、パース結果を文字列に戻せなければならない（SHALL）。

#### Scenario: Uri Display
- **WHEN** `Uri::parse(s)` の結果に `to_string()` を呼ぶ
- **THEN** 元の文字列 `s` と一致する

#### Scenario: Authority Display
- **WHEN** `Authority { user_info, host, port }` に `to_string()` を呼ぶ
- **THEN** `"user:pass@host:8080"` 形式の文字列を返す

### Requirement: Path enum with segment access
`Path<'a>` は variant ごとにセグメントのスライスを返す `segments()` メソッドを提供しなければならない（SHALL）。

#### Scenario: Abempty path segments
- **WHEN** path が `/a/b/c` のとき
- **THEN** `segments()` は `["", "a", "b", "c"]` を返す

#### Scenario: Empty path
- **WHEN** path が空のとき
- **THEN** `segments()` は空スライスを返す

# oni-comb-uri-rs

A Rust crate for URI.

## Specification

This crate is based on the following specifications.

- `RFC3986: Uniform Resource Identifier (URI): Generic Syntax`

## Usage

```rust
use oni_comb_uri::uri::Uri;

let s = "http://user1:pass1@localhost:8080/example?key1=value1&key2=value2&key1=value2#f1";

match Uri::parse(s) {
  Ok(uri) => {
    uri.schema().into_iter().for_each(|s| assert_eq!(s.to_string(), "http"));
    uri
      .host_name()
      .into_iter()
      .for_each(|hn| assert_eq!(hn.to_string(), "localhost"));
    uri.port().into_iter().for_each(|p| assert_eq!(p, 8080));
    uri.user_info().into_iter().for_each(|ui| {
      assert_eq!(ui.user_name(), "user1");
      assert_eq!(ui.password(), Some("pass1"));
    });
    uri
      .path()
      .into_iter()
      .for_each(|p| assert_eq!(p.to_string(), "/example"));
    uri.query().into_iter().for_each(|q| {
      q.get_param("key1".to_string()).into_iter().for_each(|v| {
        assert_eq!(v.len(), 2);
        assert_eq!(v[0], "value1");
        assert_eq!(v[1], "value2");
      });
      q.get_param("key2".to_string()).into_iter().for_each(|v| {
        assert_eq!(v.len(), 1);
        assert_eq!(v[0], "value2");
      });
    });
    uri.fragment().into_iter().for_each(|f| assert_eq!(f, "f1"));
    println!("{:?}", uri);
  }
  Err(e) => println!("{:?}", e),
}
```
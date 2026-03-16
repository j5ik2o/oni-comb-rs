use oni_comb_uri::{Host, Path, Uri};
use std::net::{Ipv4Addr, Ipv6Addr};

#[test]
fn parse_full_uri() {
  let uri = Uri::parse("http://user:pass@localhost:8080/example?key1=value1&key2=value2#f1").unwrap();
  assert_eq!(uri.scheme(), Some("http"));
  assert_eq!(uri.user_info().unwrap().user_name(), "user");
  assert_eq!(uri.user_info().unwrap().password(), Some("pass"));
  assert_eq!(uri.host(), Some(&Host::RegName("localhost")));
  assert_eq!(uri.port(), Some(8080));
  assert_eq!(uri.path().to_string(), "/example");
  let params = uri.query_params();
  assert_eq!(params[0], ("key1", Some("value1")));
  assert_eq!(params[1], ("key2", Some("value2")));
  assert_eq!(uri.fragment(), Some("f1"));
}

#[test]
fn parse_simple_http() {
  let uri = Uri::parse("http://example.com").unwrap();
  assert_eq!(uri.scheme(), Some("http"));
  assert_eq!(uri.host(), Some(&Host::RegName("example.com")));
  assert_eq!(uri.port(), None);
}

#[test]
fn parse_with_path() {
  let uri = Uri::parse("http://host/a/b/c").unwrap();
  assert_eq!(uri.path().to_string(), "/a/b/c");
}

#[test]
fn parse_ipv4() {
  let uri = Uri::parse("http://192.168.1.1/path").unwrap();
  assert_eq!(uri.host(), Some(&Host::Ipv4(Ipv4Addr::new(192, 168, 1, 1))));
}

#[test]
fn parse_ipv6() {
  let uri = Uri::parse("http://[::1]/path").unwrap();
  assert_eq!(uri.host(), Some(&Host::Ipv6(Ipv6Addr::LOCALHOST)));
}

#[test]
fn parse_ipv6_full() {
  let uri = Uri::parse("http://[2001:db8:85a3::8a2e:370:7334]/").unwrap();
  match uri.host() {
    Some(Host::Ipv6(_)) => {}
    other => panic!("expected Ipv6, got {:?}", other),
  }
}

#[test]
fn parse_query_key_without_value() {
  let uri = Uri::parse("http://host?flag&k=v").unwrap();
  let params = uri.query_params();
  assert_eq!(params[0], ("flag", None));
  assert_eq!(params[1], ("k", Some("v")));
}

#[test]
fn parse_fragment_only() {
  let uri = Uri::parse("http://host#section").unwrap();
  assert_eq!(uri.fragment(), Some("section"));
}

#[test]
fn parse_mailto() {
  let uri = Uri::parse("mailto:user@example.com").unwrap();
  assert_eq!(uri.scheme(), Some("mailto"));
  match uri.path() {
    Path::Rootless(segs) => assert_eq!(segs[0], "user@example.com"),
    other => panic!("expected Rootless, got {:?}", other),
  }
}

#[test]
fn parse_urn() {
  let uri = Uri::parse("urn:isbn:0451450523").unwrap();
  assert!(uri.is_urn());
  assert_eq!(uri.urn_nid(), Some("isbn"));
  assert_eq!(uri.urn_nss(), Some("0451450523"));
}

#[test]
fn parse_urn_case_insensitive() {
  let uri = Uri::parse("URN:example:resource").unwrap();
  assert!(uri.is_urn());
  assert_eq!(uri.urn_nid(), Some("example"));
}

#[test]
fn non_urn_returns_none() {
  let uri = Uri::parse("http://example.com").unwrap();
  assert!(!uri.is_urn());
  assert_eq!(uri.urn_nid(), None);
  assert_eq!(uri.urn_nss(), None);
}

#[test]
fn display_round_trip() {
  let cases = [
    "http://user:pass@localhost:8080/example?key=value#frag",
    "http://example.com",
    "http://192.168.1.1/path",
    "mailto:user@example.com",
    "urn:isbn:0451450523",
    "http://host/a/b/c?q=1&q=2#top",
    "ftp://ftp.example.com/pub/file.txt",
  ];
  for s in cases {
    let uri = Uri::parse(s).unwrap();
    assert_eq!(uri.to_string(), s, "round-trip failed for: {}", s);
  }
}

#[test]
fn parse_bare_slash_path() {
  let uri = Uri::parse("http://host/").unwrap();
  assert_eq!(uri.to_string(), "http://host/");
  assert_eq!(uri.path().to_string(), "/");
}

#[test]
fn parse_absolute_path_only() {
  let uri = Uri::parse("scheme:/").unwrap();
  assert_eq!(uri.to_string(), "scheme:/");
}

#[test]
fn urn_nss_with_slashes() {
  let uri = Uri::parse("urn:example:a/b/c").unwrap();
  assert!(uri.is_urn());
  assert_eq!(uri.urn_nid(), Some("example"));
  assert_eq!(uri.urn_nss(), Some("a/b/c"));
}

#[test]
fn reject_invalid() {
  assert!(Uri::parse("://missing-scheme").is_err());
}

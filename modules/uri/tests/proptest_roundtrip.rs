use oni_comb_uri::Uri;
use proptest::prelude::*;

// --- Common Strategies ---

fn unreserved_char() -> BoxedStrategy<char> {
  prop_oneof![
    prop::char::range('a', 'z'),
    prop::char::range('A', 'Z'),
    prop::char::range('0', '9'),
    Just('-'),
    Just('.'),
    Just('_'),
    Just('~'),
  ]
  .boxed()
}

fn unreserved_string(min: usize, max: usize) -> BoxedStrategy<String> {
  prop::collection::vec(unreserved_char(), min..=max)
    .prop_map(|v| v.into_iter().collect())
    .boxed()
}

// --- Scheme ---

fn scheme_strategy() -> BoxedStrategy<String> {
  (
    prop::char::range('a', 'z'),
    prop::collection::vec(
      prop_oneof![
        prop::char::range('a', 'z'),
        prop::char::range('0', '9'),
        Just('+'),
        Just('-'),
        Just('.'),
      ],
      0..=8,
    ),
  )
    .prop_map(|(head, tail): (char, Vec<char>)| {
      let mut s = String::with_capacity(1 + tail.len());
      s.push(head);
      s.extend(tail);
      s
    })
    .boxed()
}

// --- IPv4 ---

fn ipv4_strategy() -> BoxedStrategy<String> {
  (0u8..=255, 0u8..=255, 0u8..=255, 0u8..=255)
    .prop_map(|(a, b, c, d)| format!("{}.{}.{}.{}", a, b, c, d))
    .boxed()
}

// --- IPv6 ---

fn ipv6_strategy() -> BoxedStrategy<String> {
  prop::collection::vec(any::<u16>(), 8..=8)
    .prop_map(|segs| {
      format!(
        "{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
        segs[0], segs[1], segs[2], segs[3], segs[4], segs[5], segs[6], segs[7]
      )
    })
    .boxed()
}

// --- Host ---

fn reg_name_strategy() -> BoxedStrategy<String> {
  unreserved_string(1, 10)
}

// --- Authority ---

fn userinfo_strategy() -> BoxedStrategy<String> {
  (unreserved_string(1, 6), prop::option::of(unreserved_string(1, 6)))
    .prop_map(|(user, pass): (String, Option<String>)| match pass {
      Some(p) => format!("{}:{}", user, p),
      None => user,
    })
    .boxed()
}

fn authority_strategy() -> BoxedStrategy<String> {
  (
    prop::option::of(userinfo_strategy()),
    reg_name_strategy(),
    prop::option::of(1u16..=65535),
  )
    .prop_map(|(ui, host, port): (Option<String>, String, Option<u16>)| {
      let mut s = String::new();
      if let Some(u) = ui {
        s.push_str(&u);
        s.push('@');
      }
      s.push_str(&host);
      if let Some(p) = port {
        s.push(':');
        s.push_str(&p.to_string());
      }
      s
    })
    .boxed()
}

// --- Path ---

fn path_segment_strategy() -> BoxedStrategy<String> {
  unreserved_string(1, 8)
}

fn path_abempty_strategy() -> BoxedStrategy<String> {
  prop::collection::vec(path_segment_strategy(), 0..=4)
    .prop_map(|segs| {
      if segs.is_empty() {
        String::new()
      } else {
        segs.iter().map(|s| format!("/{}", s)).collect()
      }
    })
    .boxed()
}

// --- Query ---

fn query_param_strategy() -> BoxedStrategy<String> {
  (unreserved_string(1, 6), prop::option::of(unreserved_string(1, 6)))
    .prop_map(|(k, v): (String, Option<String>)| match v {
      Some(val) => format!("{}={}", k, val),
      None => k,
    })
    .boxed()
}

fn query_strategy() -> BoxedStrategy<String> {
  prop::collection::vec(query_param_strategy(), 1..=4)
    .prop_map(|params| params.join("&"))
    .boxed()
}

// --- Fragment ---

fn fragment_strategy() -> BoxedStrategy<String> {
  unreserved_string(1, 10)
}

// --- Full URI ---

fn uri_strategy() -> BoxedStrategy<String> {
  (
    scheme_strategy(),
    authority_strategy(),
    path_abempty_strategy(),
    prop::option::of(query_strategy()),
    prop::option::of(fragment_strategy()),
  )
    .prop_map(
      |(scheme, auth, path, query, frag): (String, String, String, Option<String>, Option<String>)| {
        let mut s = format!("{}://{}{}", scheme, auth, path);
        if let Some(q) = query {
          s.push('?');
          s.push_str(&q);
        }
        if let Some(f) = frag {
          s.push('#');
          s.push_str(&f);
        }
        s
      },
    )
    .boxed()
}

// --- Tests ---

proptest! {
  #[test]
  fn roundtrip_scheme(s in scheme_strategy()) {
    let uri_str = format!("{}://host", s);
    let uri = Uri::parse(&uri_str).unwrap();
    prop_assert_eq!(uri.scheme(), Some(s.as_str()));
  }

  #[test]
  fn roundtrip_ipv4(addr in ipv4_strategy()) {
    let uri_str = format!("http://{}/", addr);
    let uri = Uri::parse(&uri_str).unwrap();
    prop_assert_eq!(uri.to_string(), uri_str);
  }

  #[test]
  fn roundtrip_ipv6(addr in ipv6_strategy()) {
    let uri_str = format!("http://[{}]/", addr);
    let uri = Uri::parse(&uri_str).unwrap();
    prop_assert!(uri.host().is_some());
  }

  #[test]
  fn roundtrip_host(h in reg_name_strategy()) {
    let uri_str = format!("http://{}", h);
    let uri = Uri::parse(&uri_str).unwrap();
    prop_assert_eq!(uri.to_string(), uri_str);
  }

  #[test]
  fn roundtrip_authority(auth in authority_strategy()) {
    let uri_str = format!("http://{}/", auth);
    let uri = Uri::parse(&uri_str).unwrap();
    prop_assert_eq!(uri.to_string(), uri_str);
  }

  #[test]
  fn roundtrip_path(segs in prop::collection::vec(path_segment_strategy(), 1..=4)) {
    let path: String = segs.iter().map(|s| format!("/{}", s)).collect();
    let uri_str = format!("http://host{}", path);
    let uri = Uri::parse(&uri_str).unwrap();
    prop_assert_eq!(uri.to_string(), uri_str);
  }

  #[test]
  fn roundtrip_query(q in query_strategy()) {
    let uri_str = format!("http://host?{}", q);
    let uri = Uri::parse(&uri_str).unwrap();
    prop_assert_eq!(uri.to_string(), uri_str);
  }

  #[test]
  fn roundtrip_full_uri(s in uri_strategy()) {
    let uri = Uri::parse(&s).unwrap();
    prop_assert_eq!(uri.to_string(), s);
  }
}

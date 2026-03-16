#[path = "../benches/impls/mod.rs"]
mod impls;

#[test]
fn identifier_cross_library_agreement() {
  let inputs = ["foo", "_private", "foo_bar_123", "x"];
  for input in inputs {
    let oni = impls::oni_comb::parse_identifier(input);
    let win = impls::winnow_impl::parse_identifier(input);
    let n = impls::nom_impl::parse_identifier(input);
    let ch = impls::chumsky_impl::parse_identifier(input);
    let p = impls::pom_impl::parse_identifier(input);
    assert_eq!(oni, win, "oni-comb vs winnow disagree on {input:?}");
    assert_eq!(oni, n, "oni-comb vs nom disagree on {input:?}");
    assert_eq!(oni, ch, "oni-comb vs chumsky disagree on {input:?}");
    assert_eq!(oni, p, "oni-comb vs pom disagree on {input:?}");
  }
}

#[test]
fn identifier_cross_library_rejection() {
  let inputs = ["123abc", ""];
  for input in inputs {
    let oni = impls::oni_comb::parse_identifier(input);
    let win = impls::winnow_impl::parse_identifier(input);
    let n = impls::nom_impl::parse_identifier(input);
    let ch = impls::chumsky_impl::parse_identifier(input);
    let p = impls::pom_impl::parse_identifier(input);
    assert_eq!(oni, None, "oni-comb should reject {input:?}");
    assert_eq!(win, None, "winnow should reject {input:?}");
    assert_eq!(n, None, "nom should reject {input:?}");
    assert_eq!(ch, None, "chumsky should reject {input:?}");
    assert_eq!(p, None, "pom should reject {input:?}");
  }
}

#[test]
fn integer_cross_library_agreement() {
  let inputs = ["0", "42", "9999999"];
  for input in inputs {
    let oni = impls::oni_comb::parse_integer(input);
    let win = impls::winnow_impl::parse_integer(input);
    let n = impls::nom_impl::parse_integer(input);
    let ch = impls::chumsky_impl::parse_integer(input);
    let p = impls::pom_impl::parse_integer(input);
    assert_eq!(oni, win, "oni-comb vs winnow disagree on {input:?}");
    assert_eq!(oni, n, "oni-comb vs nom disagree on {input:?}");
    assert_eq!(oni, ch, "oni-comb vs chumsky disagree on {input:?}");
    assert_eq!(oni, p, "oni-comb vs pom disagree on {input:?}");
  }
}

#[test]
fn integer_cross_library_rejection() {
  let inputs = ["abc", ""];
  for input in inputs {
    let oni = impls::oni_comb::parse_integer(input);
    let win = impls::winnow_impl::parse_integer(input);
    let n = impls::nom_impl::parse_integer(input);
    let ch = impls::chumsky_impl::parse_integer(input);
    let p = impls::pom_impl::parse_integer(input);
    assert_eq!(oni, None, "oni-comb should reject {input:?}");
    assert_eq!(win, None, "winnow should reject {input:?}");
    assert_eq!(n, None, "nom should reject {input:?}");
    assert_eq!(ch, None, "chumsky should reject {input:?}");
    assert_eq!(p, None, "pom should reject {input:?}");
  }
}

#[test]
fn identifier_partial_consumption_agreement() {
  let input = "foo bar";
  let oni = impls::oni_comb::parse_identifier(input);
  let win = impls::winnow_impl::parse_identifier(input);
  let n = impls::nom_impl::parse_identifier(input);
  let ch = impls::chumsky_impl::parse_identifier(input);
  let p = impls::pom_impl::parse_identifier(input);
  assert_eq!(oni, win, "oni-comb vs winnow disagree on partial {input:?}");
  assert_eq!(oni, n, "oni-comb vs nom disagree on partial {input:?}");
  assert_eq!(oni, ch, "oni-comb vs chumsky disagree on partial {input:?}");
  assert_eq!(oni, p, "oni-comb vs pom disagree on partial {input:?}");
}

#[test]
fn integer_partial_consumption_agreement() {
  let input = "123abc";
  let oni = impls::oni_comb::parse_integer(input);
  let win = impls::winnow_impl::parse_integer(input);
  let n = impls::nom_impl::parse_integer(input);
  let ch = impls::chumsky_impl::parse_integer(input);
  let p = impls::pom_impl::parse_integer(input);
  assert_eq!(oni, win, "oni-comb vs winnow disagree on partial {input:?}");
  assert_eq!(oni, n, "oni-comb vs nom disagree on partial {input:?}");
  assert_eq!(oni, ch, "oni-comb vs chumsky disagree on partial {input:?}");
  assert_eq!(oni, p, "oni-comb vs pom disagree on partial {input:?}");
}

#[test]
fn winnow_identifier_head_success_empty_tail() {
  assert_eq!(impls::winnow_impl::parse_identifier("a!"), Some("a".to_string()),);
  assert_eq!(impls::winnow_impl::parse_identifier("Z "), Some("Z".to_string()),);
  assert_eq!(impls::winnow_impl::parse_identifier("_"), Some("_".to_string()),);
}

#[test]
fn bench_scaffold_files_exist() {
  assert!(
    std::path::Path::new("benches/workloads/token.rs").exists()
      || std::path::Path::new("parser/benches/workloads/token.rs").exists(),
    "token workload scaffold missing"
  );
  assert!(
    std::path::Path::new("benches/workloads/json.rs").exists()
      || std::path::Path::new("parser/benches/workloads/json.rs").exists(),
    "json workload scaffold missing"
  );
  assert!(
    std::path::Path::new("benches/workloads/arithmetic.rs").exists()
      || std::path::Path::new("parser/benches/workloads/arithmetic.rs").exists(),
    "arithmetic workload scaffold missing"
  );
  assert!(
    std::path::Path::new("benches/alloc_count.rs").exists()
      || std::path::Path::new("parser/benches/alloc_count.rs").exists(),
    "alloc_count bench missing"
  );
}

#[test]
#[ignore = "Requires recursive/sep_by/string parsers — deferred to MS4"]
fn json_subset_correctness() {}

#[test]
#[ignore = "Requires recursive/precedence parser — deferred to MS5"]
fn arithmetic_expression_correctness() {}

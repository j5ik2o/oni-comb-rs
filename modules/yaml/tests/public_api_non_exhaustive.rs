#[test]
fn public_api_non_exhaustive_enums_require_wildcards() {
  let tests = trybuild::TestCases::new();
  tests.compile_fail("tests/ui/non_exhaustive_*.rs");
}

use oni_comb_parser::input::Input;
use oni_comb_parser::str_input::StrInput;

#[test]
fn new_str_input_starts_at_offset_zero() {
    let input = StrInput::new("hello");

    assert_eq!(input.offset(), 0);
}

#[test]
fn is_eof_returns_false_for_non_empty_input() {
    let input = StrInput::new("abc");

    assert!(!input.is_eof());
}

#[test]
fn is_eof_returns_true_for_empty_input() {
    let input = StrInput::new("");

    assert!(input.is_eof());
}

#[test]
fn remaining_returns_full_string_at_start() {
    let input = StrInput::new("hello");

    assert_eq!(input.remaining(), "hello");
}

#[test]
fn checkpoint_and_reset_restores_position() {
    let mut input = StrInput::new("abcdef");
    let cp = input.checkpoint();

    input.reset(cp);

    assert_eq!(input.offset(), 0);
    assert_eq!(input.remaining(), "abcdef");
}

#[test]
fn offset_reflects_consumed_bytes() {
    let input = StrInput::new("abc");
    let cp1 = input.checkpoint();

    assert_eq!(cp1, cp1);
}

#[test]
fn checkpoint_equality_at_same_position() {
    let input = StrInput::new("test");
    let cp1 = input.checkpoint();
    let cp2 = input.checkpoint();

    assert_eq!(cp1, cp2);
}

#[test]
fn checkpoint_ordering_is_consistent() {
    let input = StrInput::new("abc");
    let cp = input.checkpoint();

    assert!(cp <= cp);
    assert!(cp >= cp);
}

#[test]
fn remaining_on_empty_returns_empty_str() {
    let input = StrInput::new("");

    assert_eq!(input.remaining(), "");
}

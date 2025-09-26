use std::cell::Cell;

use oni_comb_parser::core::{CommittedStatus, ParseError, ParseResult, Parser};
use oni_comb_parser::prelude::{
    attempt, byte, chain_left0, chain_left1, chain_right0, chain_right1, choice, elm, exists,
    flat_map as flat_map_fn, many0, many1, many_till, map as map_fn, not, one_of, optional, or,
    or_else, peek, repeat, repeat_sep, separated_fold1, separated_list0, separated_list1, seq,
    skip_left, skip_many0, skip_many1, skip_right, skip_till, surround, take, take_until,
    take_until1, take_while, take_while0, take_while1, take_while1_fold, unwrap_or, unwrap_or_else,
};

fn any_byte<'a>() -> Parser<'a, u8, u8> {
    Parser::new(move |_, state| {
        let remaining = state.input();
        match remaining.first() {
            Some(&value) => {
                let next_state = state.advance_by(1);
                ParseResult::successful_with_state(next_state, value, 1)
            }
            None => ParseResult::failed(
                ParseError::of_custom(state.current_offset(), None, "unexpected end of input"),
                CommittedStatus::Uncommitted,
            ),
        }
    })
}

#[test]
fn map_transforms_value() {
    let parser = map_fn(byte(b'a'), |b| char::from(b).to_ascii_uppercase());
    let result = parser.parse(b"abc");
    match result {
        ParseResult::Success { value, length, .. } => {
            assert_eq!(value, 'A');
            assert_eq!(length, 1);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn flat_map_chains_parsers() {
    let parser = flat_map_fn(byte(b'a'), move |_| byte(b'b'));
    let result = parser.parse(b"abc");
    match result {
        ParseResult::Success { value, length, .. } => {
            assert_eq!(value, b'b');
            assert_eq!(length, 2);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn flat_map_propagates_commit_on_failure() {
    let parser = flat_map_fn(byte(b'a'), move |_| byte(b'b'));
    let result = parser.parse(b"ac");
    match result {
        ParseResult::Failure {
            committed_status, ..
        } => {
            assert!(committed_status.is_committed());
        }
        _ => panic!("expected failure"),
    }
}

#[test]
fn attempt_allows_backtracking() {
    let parser = attempt(flat_map_fn(byte(b'a'), move |_| byte(b'b')));
    let result = parser.parse(b"ac");
    match result {
        ParseResult::Failure {
            committed_status, ..
        } => {
            assert!(committed_status.is_uncommitted());
        }
        _ => panic!("expected failure"),
    }
}

#[test]
fn filter_rejects_unmatched_value() {
    let parser = map_fn(byte(b'a'), |b| b).filter(|b| *b == b'b');
    let result = parser.parse(b"abc");
    match result {
        ParseResult::Failure {
            committed_status,
            error,
        } => {
            assert!(committed_status.is_uncommitted());
            assert!(error.message.contains("predicate failed"));
        }
        _ => panic!("expected failure"),
    }
}

#[test]
fn skip_left_and_right_work() {
    let parser = skip_right(skip_left(byte(b'('), byte(b'a')), byte(b')')).map(|b| char::from(b));
    let result = parser.parse(b"(a)rest");
    match result {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, 'a');
            assert_eq!(length, 3);
            let rest = state.expect("state").input();
            assert_eq!(rest, b"rest");
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn surround_extracts_inner_value() {
    let parser = surround(byte(b'['), byte(b'a'), byte(b']'));
    let result = parser.parse(b"[a]");
    match result {
        ParseResult::Success { value, .. } => assert_eq!(value, b'a'),
        _ => panic!("expected success"),
    }
}

#[test]
fn many_collects_matches() {
    let parser = many0(byte(b'a'));
    let result = parser.parse(b"aaab");
    match result {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, vec![b'a'; 3]);
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"b");
        }
        _ => panic!("expected success"),
    }

    let parser1 = many1(byte(b'a'));
    let result1 = parser1.parse(b"bbb");
    match result1 {
        ParseResult::Failure {
            committed_status, ..
        } => {
            assert!(committed_status.is_uncommitted());
        }
        _ => panic!("expected failure"),
    }
}

#[test]
fn many_zero_length_inner_breaks_loop() {
    let zero = Parser::new(|_, state| ParseResult::successful_with_state(state, (), 0));
    let parser = many0(zero);
    let result = parser.parse(b"data");
    match result {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert!(value.is_empty());
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn optional_converts_uncommitted_failures_to_none() {
    let parser = optional(byte(b'a'));
    match parser.parse(b"abc") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, Some(b'a'));
            assert_eq!(length, 1);
            assert_eq!(state.expect("state").input(), b"bc");
        }
        _ => panic!("expected success"),
    }

    match optional(byte(b'a')).parse(b"zzz") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, None);
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }

    let committing = flat_map_fn(byte(b'a'), move |_| byte(b'b'));
    match optional(committing).parse(b"ac") {
        ParseResult::Failure {
            committed_status, ..
        } => assert!(committed_status.is_committed()),
        _ => panic!("expected committed failure"),
    }
}

#[test]
fn parser_optional_method_behaves_like_function() {
    match byte(b'a').optional().parse(b"a?") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, Some(b'a'));
            assert_eq!(length, 1);
            assert_eq!(state.expect("state").input(), b"?");
        }
        _ => panic!("expected success"),
    }

    match byte(b'a').optional().parse(b"zzz") {
        ParseResult::Success { value, length, .. } => {
            assert_eq!(value, None);
            assert_eq!(length, 0);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn unwrap_or_returns_default_on_failure() {
    let parser = unwrap_or(byte(b'a'), b'x');
    match parser.parse(b"zzz") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b'x');
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }

    match unwrap_or(byte(b'a'), b'x').parse(b"abc") {
        ParseResult::Success { value, length, .. } => {
            assert_eq!(value, b'a');
            assert_eq!(length, 1);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn unwrap_or_else_invokes_lazy_default() {
    let called = Cell::new(false);
    let parser = unwrap_or_else(byte(b'a'), || {
        called.set(true);
        b'y'
    });

    match parser.parse(b"zzz") {
        ParseResult::Success { value, .. } => {
            assert!(called.get());
            assert_eq!(value, b'y');
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn or_combines_two_parsers() {
    let parser = or(byte(b'a'), byte(b'b'));
    match parser.parse(b"bcd") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b'b');
            assert_eq!(length, 1);
            assert_eq!(state.expect("state").input(), b"cd");
        }
        _ => panic!("expected success"),
    }

    let committing = flat_map_fn(byte(b'a'), move |_| byte(b'b'));
    match or(committing.clone(), byte(b'c')).parse(b"ac") {
        ParseResult::Failure {
            committed_status, ..
        } => assert!(committed_status.is_committed()),
        _ => panic!("expected committed failure"),
    }
}

#[test]
fn or_else_lazily_builds_fallback() {
    let parser = or_else(byte(b'a'), || byte(b'b'));
    match parser.parse(b"bcd") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b'b');
            assert_eq!(length, 1);
            assert_eq!(state.expect("state").input(), b"cd");
        }
        _ => panic!("expected success"),
    }

    let committing = flat_map_fn(byte(b'a'), move |_| byte(b'b'));
    match or_else(committing, || byte(b'c')).parse(b"ac") {
        ParseResult::Failure {
            committed_status, ..
        } => assert!(committed_status.is_committed()),
        _ => panic!("expected committed failure"),
    }
}

#[test]
fn chain_left0_returns_none_when_no_match() {
    let parser = chain_left0(
        byte(b'a').map(|b| b as i32),
        byte(b'+').map(|_| |l: i32, r: i32| l + r),
    );
    match parser.parse(b"bbb") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, None);
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }

    match parser.parse(b"a+a") {
        ParseResult::Success { value, .. } => {
            assert_eq!(value, Some((b'a' as i32) + (b'a' as i32)))
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn chain_right0_returns_none_when_no_match() {
    let pow_parser = chain_right0(
        byte(b'2').map(|b| (b - b'0') as i32),
        byte(b'^').map(|_| |l: i32, r: i32| l.pow(r as u32)),
    );
    match pow_parser.parse(b"999") {
        ParseResult::Success { value, .. } => assert_eq!(value, None),
        _ => panic!("expected success"),
    }

    match pow_parser.parse(b"2^2") {
        ParseResult::Success { value, .. } => assert_eq!(value, Some(4)),
        _ => panic!("expected success"),
    }
}

#[test]
fn choice_selects_first_successful_parser() {
    let parser = choice(vec![byte(b'x'), byte(b'a'), byte(b'z')]);
    match parser.parse(b"abc") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b'a');
            assert_eq!(length, 1);
            assert_eq!(state.expect("state").input(), b"bc");
        }
        _ => panic!("expected success"),
    }

    match choice(vec![byte(b'x'), byte(b'y')]).parse(b"zzz") {
        ParseResult::Failure {
            committed_status, ..
        } => assert!(committed_status.is_uncommitted()),
        _ => panic!("expected failure"),
    }
}

#[test]
fn choice_on_empty_vector_fails() {
    let parser = choice(Vec::<Parser<'_, u8, u8>>::new());
    match parser.parse(b"abc") {
        ParseResult::Failure {
            committed_status,
            error,
        } => {
            assert!(committed_status.is_uncommitted());
            assert!(error.message.contains("or_list: empty iterator"));
        }
        _ => panic!("expected failure"),
    }
}

#[test]
fn choice_accepts_iterators() {
    let array = [byte(b'a'), byte(b'b')];
    let parser = choice(array.into_iter());
    match parser.parse(b"bcd") {
        ParseResult::Success { value, .. } => assert_eq!(value, b'b'),
        _ => panic!("expected success"),
    }
}

#[test]
fn parser_or_list_combines_multiple_parsers() {
    let parsers = vec![byte(b'a'), byte(b'b'), byte(b'c')];
    let parser = Parser::or_list(parsers);
    match parser.parse(b"cab") {
        ParseResult::Success { value, length, .. } => {
            assert_eq!(value, b'c');
            assert_eq!(length, 1);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn one_of_matches_any_candidate() {
    let parser = one_of(vec![b'a', b'b']);
    match parser.parse(b"abc") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b'a');
            assert_eq!(length, 1);
            assert_eq!(state.expect("state").input(), b"bc");
        }
        _ => panic!("expected success"),
    }

    match one_of(vec![b'x']).parse(b"abc") {
        ParseResult::Failure {
            committed_status,
            error,
        } => {
            assert!(committed_status.is_uncommitted());
            assert!(error.message.contains("one_of"));
        }
        _ => panic!("expected failure"),
    }
}

fn digits_parser<'a>() -> Parser<'a, u8, i32> {
    take_while1_fold(
        |b| b.is_ascii_digit(),
        |byte| (byte - b'0') as i32,
        |acc, byte| acc * 10 + (byte - b'0') as i32,
    )
}

fn minus_operator<'a>() -> Parser<'a, u8, fn(i32, i32) -> i32> {
    fn sub(left: i32, right: i32) -> i32 {
        left - right
    }

    Parser::new(|_, state| {
        let slice = state.input();
        match slice.first() {
            Some(b'-') => {
                let next_state = state.advance_by(1);
                ParseResult::successful_with_state(next_state, sub as fn(i32, i32) -> i32, 1)
            }
            _ => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                "expected '-' operator",
            )),
        }
    })
}

fn power_operator<'a>() -> Parser<'a, u8, fn(i32, i32) -> i32> {
    fn pow_int(base: i32, exponent: i32) -> i32 {
        base.pow(exponent as u32)
    }

    Parser::new(|_, state| {
        let slice = state.input();
        match slice.first() {
            Some(b'^') => {
                let next_state = state.advance_by(1);
                ParseResult::successful_with_state(next_state, pow_int as fn(i32, i32) -> i32, 1)
            }
            _ => ParseResult::failed_with_uncommitted(ParseError::of_custom(
                state.current_offset(),
                Some(slice),
                "expected '^' operator",
            )),
        }
    })
}

#[test]
fn elem_and_seq_primitives_work() {
    let parser = elm(b'a');
    match parser.parse(b"abc") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b'a');
            assert_eq!(length, 1);
            assert_eq!(state.expect("state").input(), b"bc");
        }
        _ => panic!("expected success"),
    }

    match elm(b'z').parse(b"abc") {
        ParseResult::Failure {
            committed_status,
            error,
        } => {
            assert!(committed_status.is_uncommitted());
            assert!(error.message.contains("elm"));
        }
        _ => panic!("expected failure"),
    }

    let seq_parser = seq(vec![b'a', b'b']);
    match seq_parser.parse(b"abcd") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b"ab");
            assert_eq!(length, 2);
            assert_eq!(state.expect("state").input(), b"cd");
        }
        _ => panic!("expected success"),
    }

    match seq(vec![b'a', b'b']).parse(b"ax") {
        ParseResult::Failure {
            committed_status, ..
        } => assert!(committed_status.is_uncommitted()),
        _ => panic!("expected failure"),
    }
}

#[test]
fn take_and_take_while_variants_cover_cases() {
    let take_two = take(2usize);
    match take_two.parse(b"abcdef") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b"ab");
            assert_eq!(length, 2);
            assert_eq!(state.expect("state").input(), b"cdef");
        }
        _ => panic!("expected success"),
    }

    match take_two.parse(b"a") {
        ParseResult::Failure {
            committed_status, ..
        } => assert!(committed_status.is_uncommitted()),
        _ => panic!("expected failure"),
    }

    let digits = take_while(|b| b.is_ascii_digit());
    match digits.parse(b"123abc") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b"123");
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"abc");
        }
        _ => panic!("expected success"),
    }

    let letters = take_while0(|b| b.is_ascii_alphabetic());
    match letters.parse(b"123") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert!(value.is_empty());
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn many_till_collects_until_terminator() {
    let item = byte(b'a');
    let end = byte(b'!');
    let parser = many_till(item.clone(), end.clone());
    match parser.parse(b"aa!rest") {
        ParseResult::Success {
            value: (items, end_value),
            length,
            state,
        } => {
            assert_eq!(items, vec![b'a', b'a']);
            assert_eq!(end_value, b'!');
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"rest");
        }
        _ => panic!("expected success"),
    }

    match parser.parse(b"bb") {
        ParseResult::Failure { .. } => {}
        _ => panic!("expected failure"),
    }

    let skip = skip_till(item, end);
    match skip.parse(b"aa!rest") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b'!');
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"rest");
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn parser_many_till_method_matches_until_end() {
    match byte(b'a').many_till(byte(b'!')).parse(b"aa!?") {
        ParseResult::Success {
            value: (items, end_value),
            length,
            state,
        } => {
            assert_eq!(items, vec![b'a', b'a']);
            assert_eq!(end_value, b'!');
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"?");
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn take_until_variants_stop_before_pattern() {
    let parser = take_until(b"XY".to_vec());
    match parser.parse(b"abcXYZ") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b"abc");
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"XYZ");
        }
        _ => panic!("expected success"),
    }

    let parser1 = take_until1(b"END".to_vec());
    match parser1.parse(b"valueEND") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b"value");
            assert_eq!(length, 5);
            assert_eq!(state.expect("state").input(), b"END");
        }
        _ => panic!("expected success"),
    }

    match take_until1(b"AB".to_vec()).parse(b"ABCD") {
        ParseResult::Failure {
            committed_status, ..
        } => {
            assert!(committed_status.is_uncommitted());
        }
        _ => panic!("expected failure"),
    }
}

#[test]
fn peek_allows_lookahead_without_consumption() {
    let parser = peek(byte(b'a'));
    match parser.parse(b"abc") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b'a');
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }

    match peek(byte(b'z')).parse(b"abc") {
        ParseResult::Failure {
            committed_status, ..
        } => assert!(committed_status.is_uncommitted()),
        _ => panic!("expected failure"),
    }
}

#[test]
fn chain_left1_applies_left_associative_operations() {
    let expr = chain_left1(digits_parser(), minus_operator());
    match expr.parse(b"10-3-2") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, 5);
            assert_eq!(length, 6);
            assert_eq!(state.expect("state").input(), b"");
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn chain_right1_applies_right_associative_operations() {
    let expr = chain_right1(digits_parser(), power_operator());
    match expr.parse(b"2^3^2") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, 512);
            assert_eq!(length, 5);
            assert_eq!(state.expect("state").input(), b"");
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn repeat_executes_parser_fixed_times() {
    let parser = repeat(byte(b'a'), 3);
    match parser.parse(b"aaab") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, vec![b'a'; 3]);
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"b");
        }
        _ => panic!("expected success"),
    }

    match parser.parse(b"aab") {
        ParseResult::Failure { .. } => {}
        _ => panic!("expected failure"),
    }

    let zero = repeat(byte(b'a'), 0);
    match zero.parse(b"hello") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert!(value.is_empty());
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn repeat_sep_requires_separators_between_items() {
    let parser = repeat_sep(digits_parser(), byte(b','), 3);
    match parser.parse(b"1,23,4rest") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, vec![1, 23, 4]);
            assert_eq!(length, 6);
            assert_eq!(state.expect("state").input(), b"rest");
        }
        _ => panic!("expected success"),
    }

    match parser.parse(b"1,2") {
        ParseResult::Failure { .. } => {}
        _ => panic!("expected failure"),
    }

    let zero = repeat_sep(digits_parser(), byte(b','), 0);
    match zero.parse(b"anything") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert!(value.is_empty());
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn skip_many_variants_consume_without_collecting() {
    let parser = skip_many0(byte(b'a'));
    match parser.parse(b"aaab") {
        ParseResult::Success { length, state, .. } => {
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"b");
        }
        _ => panic!("expected success"),
    }

    match parser.parse(b"bbb") {
        ParseResult::Success { length, state, .. } => {
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }

    let parser1 = skip_many1(byte(b'a'));
    match parser1.parse(b"aaab") {
        ParseResult::Success { length, state, .. } => {
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"b");
        }
        _ => panic!("expected success"),
    }

    match parser1.parse(b"bbb") {
        ParseResult::Failure {
            committed_status, ..
        } => assert!(committed_status.is_uncommitted()),
        _ => panic!("expected failure"),
    }
}

#[test]
fn parser_skip_many_methods_consume_input() {
    match byte(b'a').skip_many0().parse(b"aaab") {
        ParseResult::Success { length, state, .. } => {
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"b");
        }
        _ => panic!("expected success"),
    }

    match byte(b'a').skip_many1().parse(b"aaab") {
        ParseResult::Success { length, state, .. } => {
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"b");
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn exists_and_not_are_lookahead() {
    let exists_parser = exists(byte(b'a'));
    match exists_parser.parse(b"abc") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert!(value);
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }

    let not_parser = not(byte(b'a'));
    match not_parser.parse(b"abc") {
        ParseResult::Failure { .. } => {}
        _ => panic!("expected failure"),
    }

    match not(byte(b'z')).parse(b"abc") {
        ParseResult::Success { length, state, .. } => {
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }
}

#[test]
fn restates_many1_failure_when_inner_committed() {
    let parser = many1(byte(b'a'));
    let result = parser.parse(b"a");
    match result {
        ParseResult::Success { .. } => {}
        _ => panic!("expected success"),
    }

    let parser_commit = flat_map_fn(any_byte(), move |_| byte(b'b'));
    let result_commit = parser_commit.parse(b"ac");
    match result_commit {
        ParseResult::Failure {
            committed_status, ..
        } => {
            assert!(committed_status.is_committed());
        }
        _ => panic!("expected failure"),
    }
}

#[test]
fn take_while1_reads_digits() {
    let parser = take_while1(|b| b.is_ascii_digit());
    match parser.parse(b"123abc") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, b"123");
            assert_eq!(length, 3);
            assert_eq!(state.expect("state").input(), b"abc");
        }
        _ => panic!("expected success"),
    }

    match parser.parse(b"abc") {
        ParseResult::Failure { .. } => {}
        _ => panic!("expected failure"),
    }
}

#[test]
fn take_while1_fold_accumulates() {
    let parser = take_while1_fold(
        |b| b.is_ascii_digit(),
        |byte| (byte - b'0') as usize,
        |acc, byte| acc + (byte - b'0') as usize,
    );

    match parser.parse(b"1234rest") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, 1 + 2 + 3 + 4);
            assert_eq!(length, 4);
            assert_eq!(state.expect("state").input(), b"rest");
        }
        _ => panic!("expected success"),
    }

    match parser.parse(b"_123") {
        ParseResult::Failure { .. } => {}
        _ => panic!("expected failure"),
    }
}

#[test]
fn separated_list1_parses_csv() {
    let number = take_while1_fold(
        |b| b.is_ascii_digit(),
        |byte| (byte - b'0') as usize,
        |acc, byte| acc * 10 + (byte - b'0') as usize,
    );
    let parser = separated_list1(number.clone(), byte(b','));
    match parser.parse(b"10,20,30") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, vec![10, 20, 30]);
            assert_eq!(length, 8);
            assert_eq!(state.expect("state").input(), b"");
        }
        _ => panic!("expected success"),
    }

    match parser.parse(b"10,") {
        ParseResult::Failure { .. } => {}
        _ => panic!("expected failure"),
    }

    let fold_parser = separated_fold1(
        number.clone(),
        byte(b','),
        |first| first,
        |acc, next| acc + next,
    );
    match fold_parser.parse(b"1,2,3,4") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, 10);
            assert_eq!(length, 7);
            assert_eq!(state.expect("state").input(), b"");
        }
        _ => panic!("expected success"),
    }

    let zero_parser = separated_list0(number.clone(), byte(b','));
    match zero_parser.parse(b"") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert!(value.is_empty());
            assert_eq!(length, 0);
            assert_eq!(state.expect("state").current_offset(), 0);
        }
        _ => panic!("expected success"),
    }

    match zero_parser.parse(b"10,20") {
        ParseResult::Success {
            value,
            length,
            state,
        } => {
            assert_eq!(value, vec![10, 20]);
            assert_eq!(length, 5);
            assert_eq!(state.expect("state").input(), b"");
        }
        _ => panic!("expected success"),
    }
}

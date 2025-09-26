use oni_comb_parser::core::{CommittedStatus, ParseError, ParseResult, Parser};
use oni_comb_parser::prelude::{
    attempt, byte, exists, flat_map as flat_map_fn, many0, many1, map as map_fn, not,
    separated_fold1, separated_list1, skip_left, skip_right, surround, take_while1,
    take_while1_fold,
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

    let fold_parser = separated_fold1(number, byte(b','), |first| first, |acc, next| acc + next);
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
}

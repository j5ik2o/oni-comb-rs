use oni_comb_parser_rs::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn test_failed_lazy_static() {
    let input_str = "abc";
    let input = input_str.chars().collect::<Vec<_>>();

    let counter = Rc::new(Cell::new(0));
    let counter_clone = counter.clone();
    let input_clone = input.clone();
    let p: StaticParser<'_, char, char> = failed_lazy_static(move || {
        counter_clone.set(counter_clone.get() + 1);
        let error = ParseError::of_mismatch_owned(input_clone.clone(), 0, 0, format!("error message: {}", counter_clone.get()));
        (error, CommittedStatus::Uncommitted)
    });

    // The error message should be evaluated lazily
    assert_eq!(counter.get(), 0);

    let result = p.parse_as_result(&input);
    assert!(result.is_err());

    // The error message should be evaluated now
    assert_eq!(counter.get(), 1);

    // Check the error message
    match result {
        Err(ParseError::Mismatch { message, .. }) => {
            assert_eq!(message, "error message: 1");
        }
        _ => panic!("Expected Mismatch error"),
    }

    // Parsing again should evaluate the error message again
    let result2 = p.parse_as_result(&input);
    assert!(result2.is_err());
    assert_eq!(counter.get(), 2);
}
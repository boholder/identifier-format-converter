#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::{quickcheck, TestResult};

use common::*;
use naming_lib as lib;

mod common;

#[quickcheck]
fn screaming_snake_identifier_should_be_recognized(word: String) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }
    TestResult::from_bool(lib::is_screaming_snake(&build_screaming_snake_str(word)))
}

#[quickcheck]
fn snake_identifier_should_be_recognized(word: String) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }
    TestResult::from_bool(lib::is_snake(&build_snake_str(word)))
}

#[quickcheck]
fn kebab_identifier_should_be_recognized(word: String) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }
    TestResult::from_bool(lib::is_kebab(&build_kebab_str(word)))
}

#[quickcheck]
fn camel_identifier_should_be_recognized(word: String) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }
    TestResult::from_bool(lib::is_camel(&build_camel_str(word)))
}

#[quickcheck]
fn pascal_identifier_should_be_recognized(word: String) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }
    TestResult::from_bool(lib::is_pascal(&build_pascal_str(word)))
}

#[quickcheck]
fn valid_strings_that_more_than_one_word_should_only_be_recognized_as_only_one_format(word: String) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }

    // One word strings like "foo123" will be recognized by more than one identifier.
    // We'll discard these test cases.
    let test_str = build_random_format_str(word);
    if lib::is_single_word(&test_str) {
        return TestResult::discard();
    }

    let results =
        [lib::is_screaming_snake(&test_str),
            lib::is_snake(&test_str),
            lib::is_kebab(&test_str),
            lib::is_camel(&test_str),
            lib::is_pascal(&test_str)];

    TestResult::from_bool(results.iter().filter(|result| **result).count() == 1)
}

#[quickcheck]
fn string_remains_unchanged_after_being_wrapped_into_the_format(s: String) -> bool {
    s == lib::which_case(&s).to_string()
}
// We use this property based testing library for integrate testing.
// It can generate a large number of random test cases,
// which is just right for testing this crate.
// https://github.com/BurntSushi/quickcheck

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
    id_test_helper(word, lib::is_screaming_snake, build_screaming_snake_str)
}

#[quickcheck]
fn snake_identifier_should_be_recognized(word: String) -> TestResult {
    id_test_helper(word, lib::is_snake, build_snake_str)
}

#[quickcheck]
fn kebab_identifier_should_be_recognized(word: String) -> TestResult {
    id_test_helper(word, lib::is_kebab, build_kebab_str)
}

#[quickcheck]
fn camel_identifier_should_be_recognized(word: String) -> TestResult {
    id_test_helper(word, lib::is_camel, build_camel_str)
}

#[quickcheck]
fn pascal_identifier_should_be_recognized(word: String) -> TestResult {
    id_test_helper(word, lib::is_pascal, build_pascal_str)
}

fn id_test_helper(word: String,
                  checker: fn(&str) -> bool,
                  builder: fn(String) -> String) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }
    TestResult::from_bool(checker(&builder(word)))
}

#[quickcheck]
fn valid_strings_that_more_than_one_word_should_only_be_recognized_as_only_one_format(word: String) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }

    // it contains 5 different format strings.
    let strs = build_all_format_str(word);
    // One word strings like "foo123" will be recognized by more than one identifier.
    // If any of 5 strings is a single word, we'll discard this test case.
    if strs.iter()
        .map(|s| lib::is_single_word(&s))
        .reduce(|a, b| a || b)
        .unwrap() {
        return TestResult::discard();
    }

    let match_count = strs.iter()
        // for each format, generate 5 bool results.
        .map(|s|
            [lib::is_screaming_snake(&s),
                lib::is_snake(&s),
                lib::is_kebab(&s),
                lib::is_camel(&s),
                lib::is_pascal(&s)])
        .flatten()
        // count true value in total 25 results.
        .filter(|result| *result)
        .count();

    TestResult::from_bool(match_count == 5)
}

#[quickcheck]
fn string_remains_unchanged_after_being_wrapped_into_the_format(s: String) -> bool {
    s == lib::which_case(&s).to_string()
}
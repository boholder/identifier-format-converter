#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

use quickcheck::{quickcheck, TestResult};

use common::*;
use naming_lib as lib;
use naming_lib::NamingCase;

mod common;

#[quickcheck]
fn recognise_camel_case_as_hungarian_notation_and_others_as_invalid(s: String) -> TestResult {
    if is_not_valid_single_word(&s) {
        TestResult::from_bool(
            lib::from_hungarian_notation(&s) == NamingCase::Invalid(s)
        )
    } else {
        let judged_cases = build_all_format_str(s).iter()
            .map(|s| lib::from_hungarian_notation(&s))
            .collect::<Vec<NamingCase>>();

        // Can't directly use "NamingCase::Pascal == case"
        let pascal_count = judged_cases.iter()
            .filter(|case|
                if let NamingCase::Pascal(_) = case {
                    true
                } else {
                    false
                })
            .count();

        let invalid_count = judged_cases.iter()
            .filter(|case|
                if let NamingCase::Invalid(_) = case {
                    true
                } else {
                    false
                })
            .count();

        TestResult::from_bool(pascal_count == 1 && invalid_count == 4)
    }
}

#[quickcheck]
fn correctly_convert_to_screaming_snake_case(word: String) -> TestResult {
    let builder = |s: &str| lib::from(s).to_screaming_snake();
    convert_test_helper(word.clone(), lib::is_screaming_snake, builder)
}

#[quickcheck]
fn correctly_convert_to_snake_case(word: String) -> TestResult {
    let builder = |s: &str| lib::from(s).to_snake();
    convert_test_helper(word.clone(), lib::is_snake, builder)
}

#[quickcheck]
fn correctly_convert_to_kebab_case(word: String) -> TestResult {
    let builder = |s: &str| lib::from(s).to_kebab();
    convert_test_helper(word.clone(), lib::is_kebab, builder)
}

#[quickcheck]
fn correctly_convert_to_camel_case(word: String) -> TestResult {
    let builder = |s: &str| lib::from(s).to_camel();
    convert_test_helper(word.clone(), lib::is_camel, builder)
}

#[quickcheck]
fn correctly_convert_to_pascal_case(word: String) -> TestResult {
    let builder = |s: &str| lib::from(s).to_pascal();
    convert_test_helper(word.clone(), lib::is_pascal, builder)
}

fn convert_test_helper(word: String,
                       checker: fn(&str) -> bool,
                       builder: fn(&str) -> Result<String, &'static str>) -> TestResult {
    if is_not_valid_single_word(&word) {
        return TestResult::discard();
    }

    let all_strs_can_be_correctly_converted = build_all_format_str(word).iter()
        .map(|s| checker(&builder(s).unwrap()))
        .reduce(|a, b| a && b)
        .unwrap();

    TestResult::from_bool(all_strs_can_be_correctly_converted)
}
// These tests show that it is impossible to
// design a common set of extraction rules for different languages.
// Even the extraction of the desired words
// cannot be done appropriately in a single language.
//
// We learned about the limitations of this tool,
// and marked this in the document for users.

use clt_lib::{self as lib, Captor, Filter};
use naming_lib::{NamingCase, which_case};

#[ignore]
#[test]
fn java() {
    let text =
        lib::read_from_files(&["tests/data/java.txt"], None).unwrap();

    // java variables are in camel case.
    let actual = Filter::new(Some(vec!["c".to_string()]))
        .unwrap().to_naming_cases_from(
        Captor::new(
            Some(lib::to_string_vec(vec![r"\s \s*=", r"\s \s*;"]))
        ).unwrap().capture_words(text)
    );

    // =========================Failure:
    // line 1: "package data;"
    // There is no language-insensitive general rule
    // to identifying difference between lines like this
    // with common assignment statements like
    // line 51: "private String name;"
    let expect = to_naming_case_vec(Box::from(
        ["count", "targetIsLive", "liveNeighborCount", "rowSize",
            "colSize", "data", "name", "category"]));

    assert_eq!(actual, expect);
}

fn to_naming_case_vec(array: Box<[&str]>) -> Vec<NamingCase> {
    array.iter().map(|id| which_case(*id)).collect()
}

#[ignore]
#[test]
fn javascript() {
    let text =
        lib::read_from_files(&["tests/data/javascript.txt"], None).unwrap();

    // variables in example file are in camel case.
    let actual = Filter::new(Some(vec!["c".to_string()]))
        .unwrap().to_naming_cases_from(
        Captor::new(
            Some(lib::to_string_vec(vec![r"\s \s*=", r"\s \s*;"]))
        ).unwrap().capture_words(text)
    );

    // =========================Failure:
    // line 9: "for (let i = 0; i < timePoints.length - 1; i++) {" -> "i"
    // can't fix it.
    // line 16: "return a - b;" -> "b"
    // should I add logic that discard words that matched in method params?
    // no, that's unnecessary complex.
    let expect = to_naming_case_vec(Box::from(
        ["findMinDifference", "sorted", "headTail", "min", "i", "b"]));

    assert_eq!(actual, expect);
}
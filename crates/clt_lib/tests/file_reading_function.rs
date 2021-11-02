// These tests test the ability to get the original text from files.

use clt_lib as lib;

#[test]
fn read_from_single_file_without_logical_eof() {
    let text = lib::read_from_files(&["tests/data/a.txt"], None);
    assert_eq!(text.unwrap(), lib::to_string_vec(vec!["front-a stop after-a"]));
}

#[test]
fn read_from_multiple_files_without_logical_eof() {
    let files = ["tests/data/a.txt", "tests/data/b.txt"];
    let text = lib::read_from_files(&files, None);
    assert_eq!(
        text.unwrap(),
        lib::to_string_vec(vec![
            "front-a stop after-a",
            "front-b hold after-b"
        ])
    );
}

#[test]
fn read_from_file_with_logical_eof_exists_in_file() {
    let text = lib::read_from_files(&["tests/data/a.txt"], Some("stop"));
    assert_eq!(text.unwrap(), lib::to_string_vec(vec!["front-a "]));
}

#[test]
fn read_whole_file_content_when_logical_eof_not_exists_but_is_passed_to_func() {
    let text = lib::read_from_files(&["tests/data/b.txt"], Some("stop"));
    assert_eq!(text.unwrap(), lib::to_string_vec(vec!["front-b hold after-b"]));
}

#[test]
fn apply_logical_eof_on_each_file_when_read_multiple_files() {
    let files = ["tests/data/a.txt", "tests/data/b.txt"];
    let text = lib::read_from_files(&files, Some("stop"));
    // a.txt has this eof, but b.txt doesn't have.
    assert_eq!(
        text.unwrap(),
        lib::to_string_vec(vec!["front-a ", "front-b hold after-b"])
    );
}

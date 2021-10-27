use clt_lib as lib;

#[test]
fn read_from_single_file_without_logical_eof() {
    let text = lib::read_from_files(
        vec!["tests/data/a.txt".to_string()], None);
    assert_eq!(text.unwrap(), " front-a stop after-a");
}

#[test]
fn read_from_multiple_files_without_logical_eof() {
    let files = vec!["tests/data/a.txt", "tests/data/b.txt"].iter()
        .map(|file| file.to_string()).collect();
    let text = lib::read_from_files(files, None);
    assert_eq!(text.unwrap(), " front-a stop after-a front-b hold after-b");
}

#[test]
fn read_from_file_with_logical_eof_exists_in_file() {
    let text =
        lib::read_from_files(vec!["tests/data/a.txt".to_string()], Some("stop"));
    assert_eq!(text.unwrap(), " front-a ");
}

#[test]
fn read_whole_file_content_when_logical_eof_not_exists_but_is_passed_to_func() {
    let text =
        lib::read_from_files(vec!["tests/data/b.txt".to_string()], Some("stop"));
    assert_eq!(text.unwrap(), " front-b hold after-b");
}

#[test]
fn apply_logical_eof_on_each_file_when_read_multiple_files() {
    let files = vec!["tests/data/a.txt", "tests/data/b.txt"].iter()
        .map(|file| file.to_string()).collect();
    let text = lib::read_from_files(files, Some("stop"));
    // a.txt has this eof, but b.txt doesn't have.
    assert_eq!(text.unwrap(), " front-a  front-b hold after-b");
}


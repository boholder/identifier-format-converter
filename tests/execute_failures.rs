use assert_cmd::Command;
use predicates::{Predicate, str as it};

#[test]
fn no_input() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.assert().success();
}

#[test]
fn input_not_exist_file() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("not_exist.txt").assert().failure();

    let err_msg = cmd.output().unwrap().stderr;
    let err_msg = String::from_utf8_lossy(&err_msg);
    assert!(it::contains("not_exist.txt:").eval(&err_msg))
}

#[test]
fn filter_option_has_hungarian_camel_conflict() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("-f=c,h").assert().failure();

    let err_msg = cmd.output().unwrap().stderr;
    let err_msg = String::from_utf8_lossy(&err_msg);
    assert!(it::contains("--filter").eval(&err_msg))
}

#[test]
fn wrong_locator_that_can_not_be_converted_to_regex() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg(r#"-l="a""#).assert().failure();
    cmd.arg(r#"-l="a ""#).assert().failure();
    cmd.arg(r#"-l=" a""#).assert().failure();

    let err_msg = cmd.output().unwrap().stderr;
    let err_msg = String::from_utf8_lossy(&err_msg);
    assert!(it::contains("locator").eval(&err_msg));
}
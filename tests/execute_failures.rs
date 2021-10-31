use assert_cmd::Command;

#[test]
fn no_input() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.assert().success();
}

#[test]
fn input_not_exist_file() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("not_exist.txt").assert().failure();
}

#[test]
fn filter_option_has_hungarian_camel_conflict() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("-f=c,h").assert().failure();
}
use assert_cmd::Command;

#[test]
fn filter() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("--filter=c,s").arg("tests/data/all.txt").assert().success()
        .stdout("\
snake_case SNAKE_CASE snake_case snake-case snakeCase SnakeCase
camelCase CAMEL_CASE camel_case camel-case camelCase CamelCase");
}

#[test]
fn filter_hungarian_option() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("--filter=h").arg("tests/data/all.txt").assert().success()
        .stdout("Case CASE case case case Case");
}

#[test]
fn eof() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    // set part of second word as logical eof
    cmd.arg("--eof=snake").arg("tests/data/all.txt").assert().success()
        // only first word is matched
        .stdout("\
SCREAMING_SNAKE SCREAMING_SNAKE screaming_snake screaming-snake screamingSnake ScreamingSnake");
}

#[test]
fn locator() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg(format!(r"--locator=\scam Case\s,ke -case")).arg("tests/data/all.txt")
        .assert().success()
        .stdout("el EL el el el El\nbab BAB bab bab bab Bab");
}

#[test]
fn locator_invalid_when_single_valid_word_appears_in_special_position() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    // Note that in one.txt, word "userId " has a space after it.
    // If remove that space, the hardcoded pair "\A(word)\z"
    // will match the "userId" because:
    // 1. it's a valid word
    // 2. it's head position is "start of file" (\A),
    //    and tail position is "end of file" (\z)
    cmd.arg(format!(r#"--locator="\s Id""#)).arg("tests/data/one.txt").assert().success()
        .stdout("userId USER_ID user_id user-id userId UserId");
}

#[test]
fn output_option_order_affects_output_order() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("--output=c,s").arg("tests/data/one.txt").assert().success()
        .stdout("userId userId user_id");
}

#[test]
fn json_flag() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("--json").arg("tests/data/one.txt")
        .assert().success()
        .stdout(r#"{"result":[{"origin":"userId","#.to_string() +
            r#""screaming_snake":"USER_ID","snake":"user_id","# +
            r#""kebab":"user-id","camel":"userId","# +
            r#""pascal":"UserId"}]}"#);
}

#[test]
fn regex_flag() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("--regex").arg("tests/data/one.txt")
        .assert().success()
        .stdout("userId USER_ID|user_id|user-id|userId|UserId");
}

#[test]
fn json_regex() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("--json").arg("--regex").arg("tests/data/one.txt")
        .assert().success()
        .stdout(r#"{"result":[{"origin":"userId","#.to_string() +
            r#""regex":"USER_ID|user_id|user-id|userId|UserId"}]}"#);
}

#[test]
fn output_regex() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("--output=s,c").arg("--regex").arg("tests/data/one.txt")
        .assert().success()
        .stdout("userId user_id|userId");
}

#[test]
fn output_json() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("--output=k,S").arg("--json").arg("tests/data/one.txt")
        .assert().success()
        .stdout(r#"{"result":[{"origin":"userId","#.to_string() +
            r#""kebab":"user-id","screaming_snake":"USER_ID"}]}"#);
}
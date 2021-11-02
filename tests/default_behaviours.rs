use assert_cmd::Command;

#[test]
fn read_from_single_file() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("tests/data/all.txt").assert().success().stdout(
        "\
SCREAMING_SNAKE SCREAMING_SNAKE screaming_snake screaming-snake screamingSnake ScreamingSnake
snake_case SNAKE_CASE snake_case snake-case snakeCase SnakeCase
kebab-case KEBAB_CASE kebab_case kebab-case kebabCase KebabCase
camelCase CAMEL_CASE camel_case camel-case camelCase CamelCase
PascalCase PASCAL_CASE pascal_case pascal-case pascalCase PascalCase",
    );
}

#[test]
fn read_from_multiple_files() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.arg("tests/data/one.txt")
        .arg("tests/data/all.txt")
        .assert()
        .success()
        .stdout(
            "\
userId USER_ID user_id user-id userId UserId
SCREAMING_SNAKE SCREAMING_SNAKE screaming_snake screaming-snake screamingSnake ScreamingSnake
snake_case SNAKE_CASE snake_case snake-case snakeCase SnakeCase
kebab-case KEBAB_CASE kebab_case kebab-case kebabCase KebabCase
camelCase CAMEL_CASE camel_case camel-case camelCase CamelCase
PascalCase PASCAL_CASE pascal_case pascal-case pascalCase PascalCase",
        );
}

#[test]
fn read_from_stdin() {
    let mut cmd = Command::cargo_bin("naming").unwrap();
    cmd.write_stdin("userId")
        .assert()
        .success()
        .stdout("userId USER_ID user_id user-id userId UserId");
}

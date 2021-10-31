use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

use regex::{self, Regex};

use crate::to_string_vec;

/// Convert Vec<String> into a slice of &str in Rust? :
/// https://stackoverflow.com/a/41180422/11397457
pub fn read_from_files<T: AsRef<str>>(files: &[T], eof: Option<&str>)
                                      -> Result<String, String> {
    let mut text = String::new();
    for file in files {
        text.push(' ');
        text.push_str(&read_file(file.as_ref(), eof)?);
    };
    Ok(text)
}

fn read_file(file: &str, eof: Option<&str>) -> Result<String, String> {
    match fs::read_to_string(file) {
        Ok(text) => { Ok(apply_eof_on_text(eof, text)) }
        Err(msg) => { Err(format!("naming: {}: {}", file, msg)) }
    }
}

fn apply_eof_on_text(eof: Option<&str>, text: String) -> String {
    match eof {
        None => { text }
        Some(eof) => {
            let position = text.find(eof);
            text[..position.unwrap_or_else(|| text.len())].to_string()
        }
    }
}

pub fn read_from_std_in(eof: Option<&str>) -> Result<String, String> {
    read_from_input(io::stdin().lock(), eof)
}

/// How to test stdin https://stackoverflow.com/a/28370712/11397457
fn read_from_input<R>(mut input: R, eof: Option<&str>) -> Result<String, String>
    where R: BufRead {
    let mut buffer = String::new();
    match input.read_to_string(&mut buffer) {
        Ok(_) => { Ok(apply_eof_on_text(eof, buffer)) }
        Err(msg) => { Err(format!("naming: stdin: {}", msg)) }
    }
}

/// Answer user's `--before`, `--after` options,
/// capture words that match the options from given long text.
pub struct Captor {
    before_regex: Regex,
    after_regex: Regex,
}

impl Captor {
    /// TODO options should be manually escaped by user.
    pub fn new(before: Option<Vec<String>>, after: Option<Vec<String>>) -> Captor {
        // Default pass \s (already in pattern string) again
        // to avoid regex (?:|\s|\A)([a-zA-Z0-9_-]+)
        // where first group can match anything
        // due to first alternative choice is empty.
        let before_options = before.unwrap_or(
            vec![r"\s".to_string()]);
        let after_options = after.unwrap_or(
            to_string_vec(vec![r"\s*=", r"\s*;"]));

        Captor {
            before_regex: Regex::new(
                // \A for start of file position
                &format!(r"(?:{}|\A)([a-zA-Z0-9_-]+)", before_options.join("|"))
            ).unwrap(),

            after_regex: Regex::new(
                // \z for end of file position
                &format!(r"([a-zA-Z0-9_-]+)(?:{}|\z)", after_options.join("|"))
            ).unwrap(),
        }
    }

    /// Extract words from given long text string,
    /// with regular expression and given locating prefix & suffix.
    pub fn capture_words(&self, text: &str) -> Vec<String> {
        // When testing and reading documents of library "regex",
        // I found out that it executes match & capture in a "non-overlapping" way.
        //
        // which means a code line without formatting (auto-inserted spaces) like this:
        // ```
        // // String s = oneMethod(arg1,arg2);
        // //                       ^    ^
        // ```
        // Library "regex" couldn't match both `arg1` and `arg2`,
        // if I use a expression like: `(?:\(|,)([a-z0-9])(?:\)|,)`,
        // because `arg1` and `arg2` shares a common comma symbol between them,
        // which is demanded by expression.
        //
        // So I use two expressions to match the input text,
        // and calculate the union of the two result set.

        let mut before_matches: Vec<String> = self.before_regex.captures_iter(text)
            .into_iter()
            .map(|cap| cap[1].to_string())
            .collect();

        let after_matches: Vec<String> = self.after_regex.captures_iter(text)
            .into_iter()
            .map(|cap| cap[1].to_string())
            .collect();

        // calculate the union of the two result
        before_matches.retain(|word| after_matches.contains(word));

        // dedup while keep the order, what an elegant solution:
        // https://users.rust-lang.org/t/deduplicate-vector-in-place-while-preserving-order/56568/6
        let mut set = HashSet::new();
        before_matches.retain(|word| set.insert(word.clone()));
        before_matches
    }
}

#[cfg(test)]
mod stdin_reader_tests {
    use crate::extraction::read_from_input;

    #[test]
    fn read_from_input_without_logical_eof() {
        let input = b"before-a end after-a";
        let actual = read_from_input(&input[..], None);
        assert_eq!(actual.unwrap(), "before-a end after-a");
    }

    #[test]
    fn read_from_input_with_logical_eof_exists() {
        let input = b"before-a end after-a";
        let actual = read_from_input(&input[..], Some("end"));
        assert_eq!(actual.unwrap(), "before-a ");
    }

    #[test]
    fn take_whole_input_when_logical_eof_not_exists() {
        let input = b"before-a end after-a";
        let actual = read_from_input(&input[..], Some("not-exists-eof"));
        assert_eq!(actual.unwrap(), "before-a end after-a");
    }
}

#[cfg(test)]
mod captor_tests {
    use crate::to_string_vec;

    use super::Captor;

    #[test]
    fn return_empty_vec_when_no_match() {
        let text = "@can@not@be@matched";
        let actual = Captor::new(None, None).capture_words(text);
        assert_eq!(actual, Vec::<String>::new())
    }

    #[test]
    fn default_captor_works() {
        let text = "int i = 1; String s = oneMethod(arg1, arg2);";
        let actual = Captor::new(None, None).capture_words(text);
        let expect: Vec<String> =
            to_string_vec(vec!["i", "1", "s"]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn custom_captor_works() {
        let text = "@now#can$be&matched";
        // note that "$" is manually escaped.
        let before: Vec<String> = to_string_vec(vec!["@", "#", r"\$", "&"]);
        let after = before.clone();

        let actual =
            Captor::new(Some(before), Some(after)).capture_words(text);
        // notice that the result is sorted.
        let expect: Vec<String> = to_string_vec(vec!["now", "can", "be", "matched"]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn duplicating_matches_are_removed() {
        let text = "let a = 1; let b = 2; let c = 3;\n\
         let a = 1; let b = 2; let c = 3;";
        let actual = Captor::new(None, None).capture_words(text);
        let expect: Vec<String> = to_string_vec(vec!["a", "1", "b", "2", "c", "3"]);
        assert_eq!(actual, expect);
    }
}
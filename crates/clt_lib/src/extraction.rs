use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead};

use regex::{self, Regex};

/// Convert Vec<String> into a slice of &str in Rust:
/// https://stackoverflow.com/a/41180422/11397457
pub fn read_from_files<T: AsRef<str>>(files: &[T], eof: Option<&str>)
                                      -> Result<Vec<String>, String> {
    let mut text = Vec::new();
    for file in files {
        text.push(read_file(file.as_ref(), eof)?);
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

/// Answer user's `--locator` options,
/// capture words that match the options from given long text.
pub struct Captor {
    patterns: Vec<Regex>,
}

impl Captor {
    /// Options should be manually escaped by user.
    /// If there is a locator pair which couldn't be converted to regex, return an Err.
    pub fn new(locators: Option<Vec<String>>) -> Result<Captor, String> {
        let locators = locators.unwrap_or(
            vec![r"\s \s".to_string()]);

        let mut patterns = Vec::new();

        for locator in locators {
            let pair = locator.split_once(" ");
            if let None = pair {
                return Err(format!(
                    "naming: locator `{}`: can't build locator pair from this.", locator));
            }
            let pair = pair.unwrap();

            patterns.push(Regex::new(
                // \A for start of file position,
                // \z for end of file position
                &format!(r"(?:{}|\A)([a-zA-Z0-9_-]+)(?:{}|\z)",
                         pair.0, pair.1)
            ).unwrap());
        }

        Ok(Captor { patterns })
    }

    /// Extract words from given long text string,
    /// with regular expression and given locating prefix & suffix.
    pub fn capture_words(&self, text: Vec<String>) -> Vec<String> {
        // apply matching on each file's content
        let mut matches: Vec<String> = text.iter()
            .map(|text| {
                // for each file's content, apply all patterns on it.
                self.patterns.iter()
                    .map(move |pattern| {
                        pattern.captures_iter(text)
                            .into_iter()
                            .map(|cap| cap[1].to_string())
                    }).flatten()
                // now get one file's matches
            }).flatten().collect();

        // dedup while keep the order, what an elegant solution:
        // https://users.rust-lang.org/t/deduplicate-vector-in-place-while-preserving-order/56568/6
        let mut set = HashSet::new();
        matches.retain(|word| set.insert(word.clone()));
        matches
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
        let text = to_string_vec(vec!["@can@not@be@matched"]);
        let actual = Captor::new(None).unwrap().capture_words(text);
        assert_eq!(actual, Vec::<String>::new())
    }

    #[test]
    fn default_captor_works() {
        let text = to_string_vec(
            vec!["int i = 1; String s = oneMethod(arg1, arg2);"]);
        let actual = Captor::new(
            Some(to_string_vec(vec![r"\s \s*=", r"\s \s*;"]))
        ).unwrap().capture_words(text);
        let expect: Vec<String> =
            to_string_vec(vec!["i", "s", "1"]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn custom_captor_works() {
        let text = to_string_vec(vec!["@now#can$be&matched"]);
        // note that "$" is manually escaped.
        let locators: Vec<String> = to_string_vec(vec![r"# \$", "@ #", r"\$ &", r"& \z"]);

        let actual =
            Captor::new(Some(locators)).unwrap().capture_words(text);
        // notice that the result order is based on option order.
        let expect: Vec<String> = to_string_vec(vec!["can", "now", "be", "matched"]);
        assert_eq!(actual, expect);
    }

    #[test]
    fn duplicating_matches_are_removed() {
        let text = to_string_vec(vec!["let a = 1; let b = 2; let c = 3;",
                                      "let a = 1; let b = 2; let c = 3;"]);
        let actual = Captor::new(
            Some(to_string_vec(vec![r"\s \s*=", r"\s \s*;"]))
        ).unwrap().capture_words(text);
        // notice that the result order is based on option order.
        let expect: Vec<String> = to_string_vec(vec!["a", "b", "c", "1", "2", "3"]);
        assert_eq!(actual, expect);
    }
}
use std::fs;
use std::io::{self, BufRead, Error};

use regex::{self, Regex};

/// Convert Vec<String> into a slice of &str in Rust? :
/// https://stackoverflow.com/a/41180422/11397457
pub fn read_from_files<T: AsRef<str>>(files: &[T], eof: Option<&str>) -> io::Result<String> {
    let mut text = String::new();
    for file in files {
        text.push(' ');
        text.push_str(&read_file(file.as_ref(), eof)?);
    };
    Ok(text)
}

fn read_file(file: &str, eof: Option<&str>) -> io::Result<String> {
    let text = fs::read_to_string(file)?;
    apply_eof_on_text(eof, text)
}

fn apply_eof_on_text(eof: Option<&str>, text: String) -> Result<String, Error> {
    match eof {
        None => { Ok(text) }
        Some(eof) => {
            let position = text.find(eof);
            Ok(text[..position.unwrap_or_else(|| text.len())].to_string())
        }
    }
}

pub fn read_from_std_in(eof: Option<&str>) -> io::Result<String> {
    read_from_input(io::stdin().lock(), eof)
}

/// How to test stdin https://stackoverflow.com/a/28370712/11397457
fn read_from_input<R>(mut input: R, eof: Option<&str>) -> io::Result<String>
    where R: BufRead {
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    apply_eof_on_text(eof, buffer)
}

/// Answer user's `--before`, `--after` options,
/// capture words that match the options from given long text.
pub struct Captor {
    before_regex: Regex,
    after_regex: Regex,
}

impl Captor {
    pub fn new<T: AsRef<str>>(before: &[T], after: &[T]) -> Captor {
        Captor {
            before_regex: Regex::new(
                format!(r"(?:{}|\s|\A)([a-zA-Z0-9_-]+)",
                        Captor::escape(before).join("|")
                ).as_str()
            ).unwrap(),

            after_regex: Regex::new(
                format!(r"([a-zA-Z0-9_-]+)(?:{}|\s|\z)",
                        Captor::escape(after).join("|")
                ).as_str()
            ).unwrap(),
        }
    }

    /// We needs to escape
    /// outside character classes characters that in user input
    /// for constructing regular expression.
    fn escape<T: AsRef<str>>(ori: &[T]) -> Vec<String> {
        ori.into_iter()
            .map(|locator| regex::escape(locator.as_ref()))
            .collect()
    }

    /// Default captor has common locating symbols 
    /// for extracting identifiers from sourcecode.
    pub fn default() -> Captor {
        Captor::new(&["(", ","], &[",", ")", "="])
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

        let before_matches: Vec<String> = self.before_regex.captures_iter(text)
            .into_iter()
            .map(|cap| cap[1].to_string())
            .collect();

        let after_matches: Vec<String> = self.after_regex.captures_iter(text)
            .into_iter()
            .map(|cap| cap[1].to_string())
            .collect();

        let mut union: Vec<String> = before_matches.into_iter()
            .filter(|word| after_matches.contains(word))
            .collect();

        union.sort();
        union.dedup();
        union
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
    use super::Captor;

    /// [This answer](https://stackoverflow.com/a/400316/11397457)
    /// gives me the special character list for testing.
    #[test]
    fn escape_special_chars_in_user_input() {
        let locators = [".", "^", "$", "*", "+",
            "?", "(", ")", "[", "{", r"\", "|"];
        let actual = Captor::escape(&locators);
        let expect = [r"\.", r"\^", r"\$", r"\*", r"\+",
            r"\?", r"\(", r"\)", r"\[", r"\{", r"\\", r"\|"];
        assert_eq!(actual, expect);
    }

    #[test]
    fn return_empty_vec_when_no_match() {
        let text = "@can@not@be@matched";
        let actual = Captor::default().capture_words(text);
        assert!(actual.is_empty())
    }

    #[test]
    fn default_captor_works() {
        let text = "w0 (w1 w2,w3 w4= w5) w6";
        let actual = Captor::default().capture_words(text);
        let expect: Vec<String> = vec!["w0", "w1", "w2", "w3", "w4", "w5", "w6"].into_iter()
            .map(|str| str.to_string()).collect();
        assert_eq!(actual, expect);
    }

    #[test]
    fn custom_captor_works() {
        let text = "@now#can$be&matched";
        let before: Vec<String> = "@#$&".chars().map(|c| c.to_string()).collect();
        let after = before.clone();

        let actual = Captor::new(&before, &after).capture_words(text);
        // notice that the result is sorted.
        let expect: Vec<String> = vec!["be", "can", "matched", "now"].into_iter()
            .map(|str| str.to_string()).collect();
        assert_eq!(actual, expect);
    }

    #[test]
    fn duplicating_matches_are_removed() {
        let text = "a b c a b c";
        let actual = Captor::default().capture_words(text);
        let expect: Vec<String> = vec!["a", "b", "c"].into_iter()
            .map(|str| str.to_string()).collect();
        assert_eq!(actual, expect);
    }
}
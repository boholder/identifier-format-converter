use lazy_static::lazy_static;
use regex::Regex;

use crate::NamingCase;

/// Determine which format the identifier belongs to.
/// Alias of [NamingCase::new()] and [from()](crate::naming_case::from()).
///
/// # Examples
///
/// Note that this method artificially restricts
/// only the `alphabetic + (optionally) numeric` format to be a valid word,
/// and words are compose into various formats with symbol `"-"`, `"_"` or without symbol.
///
/// ```
/// use naming_lib::{NamingCase::*, which_case};
///
/// assert_eq!(SingleWord("foo".to_string()), which_case("foo"));
/// assert_eq!(SingleWord("foo123".to_string()), which_case("foo123"));
/// assert_eq!(ScreamingSnake("FOO_BAR".to_string()), which_case("FOO_BAR"));
/// assert_eq!(Snake("foo_bar".to_string()), which_case("foo_bar"));
/// assert_eq!(Camel("fooBar".to_string()), which_case("fooBar"));
/// assert_eq!(Pascal("FooBar".to_string()), which_case("FooBar"));
/// ```
///
/// Therefore, the following strings will be recognized as invalid format.
///
/// ```
/// use naming_lib::{NamingCase::Invalid, which_case};
///
/// assert_eq!(Invalid("非英语".to_string()), which_case("非英语"));
/// assert_eq!(Invalid("foo@bar".to_string()), which_case("foo@bar"));
/// assert_eq!(Invalid("@foobar".to_string()), which_case("@foobar"));
/// assert_eq!(Invalid("foobar@".to_string()), which_case("foobar@"));
/// ```
pub fn which_case(identifier: &str) -> NamingCase {
    // Any better idea to refactor this method?
    if is_single_word(identifier) {
        return NamingCase::SingleWord(identifier.to_string());
    } else if is_screaming_snake(identifier) {
        return NamingCase::ScreamingSnake(identifier.to_string());
    } else if is_snake(identifier) {
        return NamingCase::Snake(identifier.to_string());
    } else if is_kebab(identifier) {
        return NamingCase::Kebab(identifier.to_string());
    } else if is_camel(identifier) {
        return NamingCase::Camel(identifier.to_string());
    } else if is_pascal(identifier) {
        return NamingCase::Pascal(identifier.to_string());
    } else {
        NamingCase::Invalid(identifier.to_string())
    }
}

/// Matches `r"^(?:\[a-z]+|\[A-Z]+|\[A-Z]\[a-z]+)\d*$"`.
///
/// # Examples
///
/// ```
/// use naming_lib::is_single_word;
///
/// assert!(is_single_word(&"aaa"));
/// assert!(is_single_word(&"aaa123"));
/// assert!(is_single_word(&"Aaa"));
/// assert!(is_single_word(&"AAA"));
///
/// // two camel cases
/// assert!(!is_single_word(&"aAA"));
/// assert!(!is_single_word(&"aAa"));
/// ```
pub fn is_single_word(word: &str) -> bool {
    lazy_static! {
            static ref SINGLE_WORD_REGEX:Regex=Regex::new(r"^(?:[a-z]+|[A-Z]+|[A-Z][a-z]+)\d*$").unwrap();
        }
    SINGLE_WORD_REGEX.is_match(word)
}

/// Matches `r"^\[A-Z]+\d*(_\[A-Z]+\d*)*$"`.
///
/// # Examples
///
/// ```
/// use naming_lib::is_screaming_snake;
///
/// assert!(is_screaming_snake(&"FOO"));
/// assert!(is_screaming_snake(&"FOO_BAR"));
/// assert!(is_screaming_snake(&"FOO123_BAR456"));
/// ```
pub fn is_screaming_snake(identifier: &str) -> bool {
    lazy_static! {
        static ref SCREAMING_SNAKE_REGEX: Regex = Regex::new(r"^[A-Z]+\d*(_[A-Z]+\d*)*$").unwrap();
    }
    SCREAMING_SNAKE_REGEX.is_match(identifier)
}

/// Matches `r"^\[a-z]+\d*(_\[a-z]+\d*)*$"`.
///
/// # Examples
///
/// ```
/// use naming_lib::is_snake;
///
/// assert!(is_snake(&"foo"));
/// assert!(is_snake(&"foo_bar"));
/// assert!(is_snake(&"foo123_bar456"));
/// ```
pub fn is_snake(identifier: &str) -> bool {
    lazy_static! {
        static ref SNAKE_REGEX: Regex = Regex::new(r"^[a-z]+\d*(_[a-z]+\d*)*$").unwrap();
    }
    SNAKE_REGEX.is_match(identifier)
}

/// Matches `r"^\[a-z]+\d*(-\[a-z]+\d*)*$"`.
///
/// # Examples
///
/// ```
/// use naming_lib::is_kebab;
///
/// assert!(is_kebab(&"foo"));
/// assert!(is_kebab(&"foo-bar"));
/// assert!(is_kebab(&"foo123-bar456"));
/// ```
pub fn is_kebab(identifier: &str) -> bool {
    lazy_static! {
        static ref KEBAB_REGEX: Regex = Regex::new(r"^[a-z]+\d*(-[a-z]+\d*)*$").unwrap();
    }
    KEBAB_REGEX.is_match(identifier)
}

/// Matches `r"^\[a-z]+\d*(\[A-Z]\[a-z]*\d*)*$"`.
///
/// # Examples
///
/// ```
/// use naming_lib::is_camel;
///
/// assert!(is_camel(&"foo"));
/// assert!(is_camel(&"fooBar"));
/// assert!(is_camel(&"foo123Bar456"));
/// ```
pub fn is_camel(identifier: &str) -> bool {
    lazy_static! {
        static ref CAMEL_REGEX: Regex = Regex::new(r"^[a-z]+\d*([A-Z][a-z]*\d*)*$").unwrap();
    }
    CAMEL_REGEX.is_match(identifier)
}

/// Matches `r"^(\[A-Z]\[a-z]*\d*)+$"`.
///
/// # Examples
///
/// ```
/// use naming_lib::is_pascal;
///
/// assert!(is_pascal(&"Foo"));
/// assert!(is_pascal(&"FooBar"));
/// assert!(is_pascal(&"Foo123Bar456"));
/// ```
pub fn is_pascal(identifier: &str) -> bool {
    lazy_static! {
        static ref PASCAL_REGEX: Regex = Regex::new(r"^([A-Z][a-z]*\d*)+$").unwrap();
    }
    PASCAL_REGEX.is_match(identifier)
}
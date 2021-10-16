use lazy_static::lazy_static;
use regex::Regex;

use crate::NamingCase;

/// Determine which format the identifier belongs to.
///
/// Note that this method artificially restricts
/// only the `alphabetic + (optionally) numeric` format to be a valid word,
/// and words are compose into various formats with symbol `"-"`, `"_"` or without symbol.
///
/// # Examples
///
/// ```
/// use naming_lib::{NamingCase, which_case};
///
/// assert_eq!(which_case("foo"), NamingCase::SingleWord("foo".to_string()));
/// assert_eq!(which_case("foo123"), NamingCase::SingleWord("foo123".to_string()));
/// assert_eq!(which_case("FOO_BAR"), NamingCase::ScreamingSnake("FOO_BAR".to_string()));
/// assert_eq!(which_case("foo_bar"), NamingCase::Snake("foo_bar".to_string()));
/// assert_eq!(which_case("fooBar"), NamingCase::Camel("fooBar".to_string()));
/// assert_eq!(which_case("FooBar"), NamingCase::Pascal("FooBar".to_string()));
///
/// assert_eq!(which_case("中文"), NamingCase::Invalid("中文".to_string()));
/// assert_eq!(which_case("foo@bar"), NamingCase::Invalid("foo@bar".to_string()));
/// assert_eq!(which_case("@foobar"), NamingCase::Invalid("@foobar".to_string()));
/// assert_eq!(which_case("foobar@"), NamingCase::Invalid("foobar@".to_string()));
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

/// matches r"^(?:\[a-z]+|\[A-Z]+|\[A-Z]\[a-z]+)\d*$"
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
/// assert!(!is_single_word(&"aAA"));
/// assert!(!is_single_word(&"aAa"));
/// ```
pub fn is_single_word(word: &str) -> bool {
    lazy_static! {
            static ref SINGLE_WORD_REGEX:Regex=Regex::new(r"^(?:[a-z]+|[A-Z]+|[A-Z][a-z]+)\d*$").unwrap();
        }
    SINGLE_WORD_REGEX.is_match(word)
}

/// matches r"^\[A-Z]+\d*(_\[A-Z]+\d*)*$",
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

/// matches r"^\[a-z]+\d*(_\[a-z]+\d*)*$",
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

/// matches r"^\[a-z]+\d*(-\[a-z]+\d*)*$",
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

/// matches r"^\[a-z]+\d*(\[A-Z]\[a-z]*\d*)*$",
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

/// matches r"^(\[A-Z]\[a-z]*\d*)+$",
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
use std::fmt::{Display, Formatter, Result};

use lazy_static::lazy_static;
use regex::Regex;

/// Indicates which format the string belongs to.
pub enum NamingFormat {
    /// A single word will be recognized as multiple formats,
    /// so it belongs to a separate category.
    SingleWord(String),
    ScreamingSnake(String),
    Snake(String),
    Kebab(String),
    Camel(String),
    Pascal(String),
    /// Can't be recognized as known format.
    Invalid(String),
}

impl Display for NamingFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            NamingFormat::SingleWord(s) => { write!(f, "{}", s) }
            NamingFormat::ScreamingSnake(s) => { write!(f, "{}", s) }
            NamingFormat::Snake(s) => { write!(f, "{}", s) }
            NamingFormat::Kebab(s) => { write!(f, "{}", s) }
            NamingFormat::Camel(s) => { write!(f, "{}", s) }
            NamingFormat::Pascal(s) => { write!(f, "{}", s) }
            NamingFormat::Invalid(s) => { write!(f, "{}", s) }
        }
    }
}

/// Determine which format the identifier belongs to.
pub fn which_format(identifier: &str) -> NamingFormat {
    // Any better idea to refactor this method?
    if is_single_word(identifier) {
        return NamingFormat::SingleWord(identifier.to_string());
    } else if is_screaming_snake(identifier) {
        return NamingFormat::ScreamingSnake(identifier.to_string());
    } else if is_snake(identifier) {
        return NamingFormat::Snake(identifier.to_string());
    } else if is_kebab(identifier) {
        return NamingFormat::Kebab(identifier.to_string());
    } else if is_camel(identifier) {
        return NamingFormat::Camel(identifier.to_string());
    } else if is_pascal(identifier) {
        return NamingFormat::Pascal(identifier.to_string());
    } else {
        NamingFormat::Invalid(identifier.to_string())
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
/// assert!(is_pascal(&"FooBar"));
/// assert!(is_pascal(&"Foo123Bar456"));
/// ```
pub fn is_pascal(identifier: &str) -> bool {
    lazy_static! {
        static ref PASCAL_REGEX: Regex = Regex::new(r"^([A-Z][a-z]*\d*)+$").unwrap();
    }
    PASCAL_REGEX.is_match(identifier)
}

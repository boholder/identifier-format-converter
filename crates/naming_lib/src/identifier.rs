use lazy_static::lazy_static;
use regex::Regex;

/// Indicates which format the string belongs to.
enum NamingFormat {
    /// Can't be recognized,
    /// because the string is not composed of words in r"^\[a-zA-Z]+\d*" regex format.
    Invalid,
    /// A single word will be recognized as multiple formats,
    /// so it belongs to a separate category.
    SingleWord,
    /// matches r"^\[A-Z]+\d*(_\[A-Z]+\d*)*$",
    /// example: FOO_BAR, FOO123_BAR456
    ScreamingSnake,
    /// matches r"^\[a-z]+\d*(_\[a-z]+\d*)*$",
    /// example: foo_bar, foo123_bar456
    Snake,
    /// matches r"^\[a-z]+\d*(-\[a-z]+\d*)*$",
    /// example: foo-bar, foo123-bar456
    Kebab,
    /// matches r"^\[a-z]+\d*(\[A-Z]\[a-z]*\d*)*$",
    /// example: fooBar, foo123Bar456
    Camel,
    /// matches r"^(\[A-Z]\[a-z]*\d*)+$",
    /// example: FooBar, Foo123Bar456
    Pascal,
}

// pub fn which_format(identifier: &str) -> NamingFormat {
//
// }

pub fn is_screaming_snake(identifier: &str) -> bool {
    lazy_static! {
        static ref SCREAMING_SNAKE_REGEX: Regex = Regex::new(r"^[A-Z]+\d*(_[A-Z]+\d*)*$").unwrap();
    }
    // example: FOO123_BAR456
    SCREAMING_SNAKE_REGEX.is_match(identifier)
}

pub fn is_snake(identifier: &str) -> bool {
    lazy_static! {
        static ref SNAKE_REGEX: Regex = Regex::new(r"^[a-z]+\d*(_[a-z]+\d*)*$").unwrap();
    }
    // example: foo123_bar456
    SNAKE_REGEX.is_match(identifier)
}

pub fn is_kebab(identifier: &str) -> bool {
    lazy_static! {
        static ref KEBAB_REGEX: Regex = Regex::new(r"^[a-z]+\d*(-[a-z]+\d*)*$").unwrap();
    }
    // example: foo123-bar456
    KEBAB_REGEX.is_match(identifier)
}

pub fn is_camel(identifier: &str) -> bool {
    lazy_static! {
        static ref CAMEL_REGEX: Regex = Regex::new(r"^[a-z]+\d*([A-Z][a-z]*\d*)*$").unwrap();
    }
    // example: foo123Bar456
    CAMEL_REGEX.is_match(identifier)
}

pub fn is_pascal(identifier: &str) -> bool {
    lazy_static! {
        static ref PASCAL_REGEX: Regex = Regex::new(r"^([A-Z][a-z]*\d*)+$").unwrap();
    }
    // example: Foo123Bar456
    PASCAL_REGEX.is_match(identifier)
}

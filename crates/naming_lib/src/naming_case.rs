use std::fmt::{Display, Formatter, Result as FmtResult};

use lazy_static::lazy_static;
use regex::Regex;

use crate::detector;

/// Indicates which format the string belongs to.
///
/// It holds the given [String] value when created,
/// which can be got by calling [NamingCase::to_string()].
///
/// It also can be converted to [String] in another format,
/// as long as it's not the [NamingCase::Invalid] enum.
#[derive(PartialEq, Debug)]
pub enum NamingCase {
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

impl Display for NamingCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            NamingCase::SingleWord(s) => { write!(f, "{}", s) }
            NamingCase::ScreamingSnake(s) => { write!(f, "{}", s) }
            NamingCase::Snake(s) => { write!(f, "{}", s) }
            NamingCase::Kebab(s) => { write!(f, "{}", s) }
            NamingCase::Camel(s) => { write!(f, "{}", s) }
            NamingCase::Pascal(s) => { write!(f, "{}", s) }
            NamingCase::Invalid(s) => { write!(f, "{}", s) }
        }
    }
}

#[allow(dead_code)]
impl NamingCase {
    /// Create a [NamingCase] value from an identifier.
    ///
    /// Alias of [which_case()].
    fn new(identifier: &str) -> NamingCase {
        detector::which_case(identifier)
    }

    fn to_screaming_snake(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        Ok(words.into_iter()
            .map(|word| word.to_ascii_uppercase())
            .collect::<Vec<String>>()
            .join("_"))
    }

    fn to_snake(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        Ok(words.into_iter()
            .map(|word| word.to_lowercase())
            .collect::<Vec<String>>()
            .join("_"))
    }

    fn to_kebab(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        Ok(words.into_iter()
            .map(|word| word.to_lowercase())
            .collect::<Vec<String>>()
            .join("-"))
    }

    fn to_camel(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        let mut iter = words.into_iter();
        let first_word = iter.next().unwrap();
        Ok(first_word + &compose_words_to_pascal(iter.collect()))
    }

    fn to_pascal(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        Ok(compose_words_to_pascal(words))
    }
}

/// Create a [NamingCase] value from an identifier.
///
/// Alias of [which_case()] and [NamingCase::new()].
pub fn from(identifier: &str) -> NamingCase {
    detector::which_case(identifier)
}

/// Return a [NamingCase::Pascal] value for a hungarian notation identifier,
/// remove the first word which representing the variable type.
///
/// Or return a [Naming::Invalid] value for others.
///
/// # Examples
///
/// ```
/// use naming_lib::{from_hungarian_notation,NamingCase};
///
/// let valid = from_hungarian_notation("iPageSize");
/// assert_eq!(valid, NamingCase::Pascal("PageSize".to_string()));
/// assert_eq!(valid.to_string(), "PageSize");
///
/// // A hungarian notation identifier will be recognized as camel case.
/// // Even though this is a valid Pascal case, it is still treated as invalid.
/// let invalid = from_hungarian_notation("NotACamelCase");
/// assert_eq!(invalid, NamingCase::Invalid("NotACamelCase".to_string()));
/// ```
pub fn from_hungarian_notation(identifier: &str) -> NamingCase {
    let real_case = detector::which_case(identifier);
    if real_case != NamingCase::Camel(identifier.to_string()) {
        return NamingCase::Invalid(identifier.to_string());
    }

    let mut iter = extract_words_from(&real_case).unwrap().into_iter();
    // discard first word
    iter.next();
    // return remains as a pascal case.
    NamingCase::new(&iter.collect::<Vec<String>>().join(""))
}

lazy_static! {
    static ref LOWER_CASE_REGEX:Regex=Regex::new(r"^[a-z]+\d*").unwrap();
    static ref FIRST_UPPER_CASE_REGEX:Regex=Regex::new(r"[A-Z][a-z]*\d*").unwrap();
}

fn extract_words_from(case: &NamingCase) -> Result<Vec<String>, &'static str> {
    return match case {
        NamingCase::SingleWord(ori) => { Ok(vec![ori.to_string()]) }
        NamingCase::ScreamingSnake(ori) => {
            Ok(ori.split('_').map(|word| word.to_string()).collect())
        }
        NamingCase::Snake(ori) => {
            Ok(ori.split('_').map(|word| word.to_string()).collect())
        }
        NamingCase::Kebab(ori) => {
            Ok(ori.split('-').map(|word| word.to_string()).collect())
        }
        NamingCase::Camel(ori) => {
            let mut words = Vec::new();

            let first_word = LOWER_CASE_REGEX.captures(&ori).unwrap().get(0)
                .expect("Can't capture first word in camel case string.")
                .as_str().to_string();

            let mut other_words = extract_words_from_pascal(
                &(ori.strip_prefix(&first_word)).unwrap());

            words.push(first_word);
            words.append(&mut other_words);

            Ok(words)
        }
        NamingCase::Pascal(ori) => { Ok(extract_words_from_pascal(&ori)) }
        NamingCase::Invalid(_) => { Err("Can't extract words from this type.") }
    };
}

fn extract_words_from_pascal(s: &str) -> Vec<String> {
    FIRST_UPPER_CASE_REGEX.find_iter(s)
        .map(|mat| mat.as_str().to_string())
        .collect()
}

fn compose_words_to_pascal(words: Vec<String>) -> String {
    words.into_iter()
        .map(|word| to_first_uppercase(word))
        .collect::<Vec<String>>()
        .join("")
}

fn to_first_uppercase(s: String) -> String {
    let (first, other) = s.split_at(1);
    first.to_ascii_uppercase() + &other.to_ascii_lowercase()
}

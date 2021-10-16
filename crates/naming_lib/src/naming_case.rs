use std::fmt::{Display, Formatter, Result as FmtResult};

use lazy_static::lazy_static;
use regex::Regex;

use crate::detector;

/// Indicates which format the string belongs to,
/// and acts as an intermediary between format conversions.
///
/// ## Create Instances
///
/// There are three ways to create an instance.
/// These three are aliases of each other.
///
/// ```
/// use naming_lib::{NamingCase, which_case, from};
///
/// let first = NamingCase::new("identifier");
/// let second = which_case("identifier");
/// let third = from("identifier");
/// ```
///
/// ### Notice
///
/// Of course you can generate instances of a specific enum type directly,
/// I can't stop you
/// (in fact [there is a solution](https://stackoverflow.com/a/28090996/11397457),
/// but it makes things more complex),
/// but I **don't recommend using this approach**.
///
/// I can't do an input valid check when you use this approach,
/// type-related methods on these instances **may cause unexpected panic**.
///
/// I currently use this myself to:
///
/// 1. write document test cases
/// (have to use this to "clearly express the state of the test values").
///
/// 2. generated instances of [Invalid](NamingCase::Invalid) enum type
/// (it's safe, because conversion methods cannot be called on this enum type,
/// there are no other type-related methods available now).
///
/// ```
/// use naming_lib::NamingCase;
///
/// let direct_instance = NamingCase::Invalid("text".to_string());
/// ```
///
/// ## Get Origin String From An Instance
///
/// A [NamingCase] instance holds the given string value when created,
/// which can be got by calling [to_string()](std::string::ToString).
///
/// ```
/// use naming_lib::from;
///
/// assert_eq!("example",from("example").to_string())
/// ```
///
/// ## Convert An Instance To Other Naming Case String
///
/// A [NamingCase] instance also can be converted to a string in another naming format,
/// as long as it's not the [Invalid](NamingCase::Invalid) enum.
///
/// ```
/// use naming_lib::from;
///
/// assert_eq!("camel_case", from("camelCase").to_snake().unwrap());
/// ```
///
/// ### Notice
///
/// For ease of use,
/// instead of implementing the conversion methods
/// with [Invalid](NamingCase::Invalid) excluded,
/// I have chosen that all conversion methods
/// will return the [Result](core::result) type.
///
/// Calling any conversion method on an [Invalid](NamingCase::Invalid) enum
/// will return an [Err](core::result::Result::Err).
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
    /// Can't be recognized as a known format.
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

impl NamingCase {
    /// Create a [NamingCase] value from an identifier.
    ///
    /// Alias of [which_case()](crate::detector::which_case()) and [from()](crate::naming_case::from()).
    pub fn new(identifier: &str) -> NamingCase {
        detector::which_case(identifier)
    }

    /// Convert the included string to screaming snake case.
    ///
    /// # Examples
    ///
    /// ```
    /// use naming_lib::{from};
    ///
    /// assert_eq!("SCREAMING", from("Screaming").to_screaming_snake().unwrap());
    /// assert_eq!("CAMEL_CASE", from("camelCase").to_screaming_snake().unwrap());
    /// assert_eq!("SNAKE_CASE", from("snake_case").to_screaming_snake().unwrap());
    /// ```
    /// # Errors
    ///
    /// Perform this on [Invalid](NamingCase::Invalid) enum
    /// will get an [Err](core::result::Result::Err).
    pub fn to_screaming_snake(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        Ok(words.into_iter()
            .map(|word| word.to_ascii_uppercase())
            .collect::<Vec<String>>()
            .join("_"))
    }

    /// Convert the included string to snake case.
    ///
    /// # Examples
    ///
    /// ```
    /// use naming_lib::{from};
    ///
    /// assert_eq!("snake", from("Snake").to_snake().unwrap());
    /// assert_eq!("kebab_case", from("kebab-case").to_snake().unwrap());
    /// assert_eq!("camel_case", from("camelCase").to_snake().unwrap());
    /// ```
    /// # Errors
    ///
    /// Perform this on [Invalid](NamingCase::Invalid) enum
    /// will get an [Err](core::result::Result::Err).
    pub fn to_snake(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        Ok(words.into_iter()
            .map(|word| word.to_ascii_lowercase())
            .collect::<Vec<String>>()
            .join("_"))
    }

    /// Convert the included string to kebab case.
    ///
    /// # Examples
    ///
    /// ```
    /// use naming_lib::{from};
    ///
    /// assert_eq!("kebab", from("Kebab").to_kebab().unwrap());
    /// assert_eq!("camel-case", from("camelCase").to_kebab().unwrap());
    /// assert_eq!("snake-case", from("snake_case").to_kebab().unwrap());
    /// ```
    /// # Errors
    ///
    /// Perform this on [Invalid](NamingCase::Invalid) enum
    /// will get an [Err](core::result::Result::Err).
    pub fn to_kebab(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        Ok(words.into_iter()
            .map(|word| word.to_ascii_lowercase())
            .collect::<Vec<String>>()
            .join("-"))
    }

    /// Convert the included string to camel case.
    ///
    /// # Examples
    ///
    /// ```
    /// use naming_lib::{from};
    ///
    /// assert_eq!("camel", from("Camel").to_camel().unwrap());
    /// assert_eq!("pascalCase", from("PascalCase").to_camel().unwrap());
    /// assert_eq!("snakeCase", from("snake_case").to_camel().unwrap());
    /// ```
    /// # Errors
    ///
    /// Perform this on [Invalid](NamingCase::Invalid) enum
    /// will get an [Err](core::result::Result::Err).
    ///
    pub fn to_camel(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        let mut iter = words.into_iter();
        let first_word = iter.next().unwrap();
        Ok(first_word.to_ascii_lowercase() + &compose_words_to_pascal(iter.collect()))
    }

    /// Convert the included string to pascal case.
    ///
    /// # Examples
    ///
    /// ```
    /// use naming_lib::{from};
    ///
    /// assert_eq!("Pascal", from("Pascal").to_pascal().unwrap());
    /// assert_eq!("CamelCase", from("camelCase").to_pascal().unwrap());
    /// assert_eq!("SnakeCase", from("snake_case").to_pascal().unwrap());
    /// ```
    /// # Errors
    ///
    /// Perform this on [Invalid](NamingCase::Invalid) enum
    /// will get an [Err](core::result::Result::Err).
    pub fn to_pascal(&self) -> Result<String, &'static str> {
        let words = extract_words_from(self)?;
        Ok(compose_words_to_pascal(words))
    }
}

/// Create a [NamingCase] value from an identifier.
///
/// Alias of [which_case()](crate::detector::which_case()) and [NamingCase::new()].
pub fn from(identifier: &str) -> NamingCase {
    detector::which_case(identifier)
}

/// Return a [Pascal](NamingCase::Pascal) enum for a hungarian notation identifier,
/// remove the first word which representing the variable type.
///
/// Or return a [Invalid](NamingCase::Invalid) enum for other inputs.
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
/// // A hungarian notation identifier will be recognized as a camel case.
/// // Even though this is a valid pascal case, it will still be treated as invalid.
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
    NamingCase::Pascal(iter.collect::<Vec<String>>().join(""))
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

            let first_word = LOWER_CASE_REGEX.captures(&ori).unwrap()
                .get(0).unwrap().as_str().to_string();

            let other_words = ori.strip_prefix(&first_word).unwrap();
            let mut other_words = extract_words_from_pascal(&other_words);

            words.push(first_word.to_ascii_lowercase());
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

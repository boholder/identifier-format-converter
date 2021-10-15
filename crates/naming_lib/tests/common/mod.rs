use std::ops::Add;

use rand::Rng;

use naming_lib as lib;

pub fn is_not_valid_single_word(word: &str) -> bool {
    !lib::is_single_word(word)
}

pub fn build_random_format_str(word: String) -> String {
    let mut rng = rand::thread_rng();
    return match rng.gen_range(0..5) {
        0 => build_screaming_snake_str(word),
        1 => build_snake_str(word),
        2 => build_kebab_str(word),
        3 => build_camel_str(word),
        _ => build_pascal_str(word),
    };
}

pub fn build_screaming_snake_str(word: String) -> String {
    build_underline_str_from(word.to_ascii_uppercase())
}

pub fn build_snake_str(word: String) -> String {
    build_underline_str_from(word.to_ascii_lowercase())
}

pub fn build_kebab_str(word: String) -> String {
    build_dash_str_from(word.to_ascii_lowercase())
}

pub fn build_camel_str(word: String) -> String {
    let head = word.clone();
    head.to_ascii_lowercase() + &build_no_separator_str_from(to_first_uppercase(word))
}

pub fn build_pascal_str(word: String) -> String {
    build_no_separator_str_from(to_first_uppercase(word))
}

pub fn to_first_uppercase(s: String) -> String {
    let (first, other) = s.split_at(1);
    first.to_ascii_uppercase() + &other.to_ascii_lowercase()
}

pub fn build_underline_str_from(word: String) -> String {
    join_random_repeated_word_with_separator(word, "_")
}

pub fn build_dash_str_from(word: String) -> String {
    join_random_repeated_word_with_separator(word, "-")
}

pub fn build_no_separator_str_from(word: String) -> String {
    join_random_repeated_word_with_separator(word, "")
}

pub fn join_random_repeated_word_with_separator(word: String, sep: &str) -> String {
    let mut rng = rand::thread_rng();
    let word = word.add(sep);
    word.repeat(rng.gen_range(1..6)).strip_suffix(sep).unwrap().to_string()
}
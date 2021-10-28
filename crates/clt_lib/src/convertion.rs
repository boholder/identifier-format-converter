use naming_lib::{self as naming, NamingCase};

/// Answer user's `--filter` option,
/// ignore captured words that user not indicates in `--filter` option,
/// and convert words to NamingCase instances.
pub struct Filter<'a> {
    options: Vec<&'a str>,
}

impl<'a> Filter<'a> {
    pub fn new(options: &'a Vec<&str>) -> Result<Filter<'a>, &'static str> {
        if Filter::has_hungarian_camel_conflict(&options) {
            return Err("In option \"--filter\", at most one of the two, \
            hungarian notation (h) and camel case (c) can appear.");
        }
        Ok(Filter { options: options.to_vec() })
    }

    fn has_hungarian_camel_conflict(options: &Vec<&str>) -> bool {
        options.contains(&"h") && options.contains(&"c")
    }

    /// Not only transform String to NamingCase,
    /// but also apply given filter on result vector.
    pub fn to_naming_cases_from(&self, words: Vec<String>) -> Vec<NamingCase> {
        let words = self.filter_words_with_options(words);

        // if user wants to treat camel case words as hungarian notation format.
        let required_hungarian = self.options.contains(&"h");
        words.iter()
            .map(|word| {
                if required_hungarian && naming::is_camel(word) {
                    naming::from_hungarian_notation(word)
                } else {
                    naming::which_case(word)
                }
            }).collect()
    }

    const PREDICATES: [(&'static str, fn(&str) -> bool); 6] =
        [("S", naming::is_screaming_snake),
            ("s", naming::is_snake),
            ("k", naming::is_kebab),
            ("c", naming::is_camel),
            ("h", naming::is_camel),
            ("p", naming::is_pascal)];

    fn filter_words_with_options(&self, words: Vec<String>) -> Vec<String> {
        let predicates: Vec<fn(&str) -> bool> = Filter::PREDICATES.iter()
            .filter(|(opt, _)| self.options.contains(opt))
            .map(|(_, f)| *f).collect();

        words.into_iter()
            .filter(|word| {
                // check if word's format belongs to one of predicates
                predicates.iter()
                    .map(|f| f(word))
                    .reduce(|a, b| a || b)
                    .unwrap()
            }).collect()
    }
}

// pub struct Convertor<'a> {
//     options: Vec<&'a str>,
// }
//
// impl<'a> Convertor<'a> {
//     pub fn new(options: &'a Vec<&str>) -> Convertor<'a> {
//         Convertor { options: options.to_vec() }
//     }
//
//     const PIPES: [(&'static str, fn(NamingCase) -> String); 5] =
//         [("S", |case| case.to_screaming_snake().unwrap()),
//             ("s", |case| case.to_snake().unwrap()),
//             ("k", |case| case.to_kebab().unwrap()),
//             ("c", |case| case.to_camel().unwrap()),
//             ("p", |case| case.to_pascal().unwrap())];
//
//     pub fn to_formats_from(&self, cases: Vec<NamingCase>) -> Vec<Vec<String>> {
//         let pipes: Vec<fn(&str) -> bool> = Convertor::PIPES.iter()
//             .filter(|(opt, _)| self.options.contains(opt))
//             .map(|(_, f)| *f).collect();
//
//         cases.into_iter()
//             .map(|case| {}).collect()
//     }
// }

#[cfg(test)]
mod filter_tests {
    use naming_lib::NamingCase;

    use super::Filter;

    #[test]
    fn can_find_hungarian_camel_conflict() {
        assert!(Filter::has_hungarian_camel_conflict(&vec!["c", "h"]));
    }

    #[test]
    fn can_filter_words_with_option() {
        let options = vec!["S", "s", "k", "c", "p"];
        let filter = Filter::new(&options).unwrap();
        let words: Vec<String> = vec!["SCREAMING_SNAKE", "snake_case",
                                      "kebab-case", "camelCase", "PascalCase",
                                      "-invalid_"].into_iter()
            .map(|s| s.to_string()).collect();

        let mut expect = words.clone();
        // remove the invalid word at tail
        expect.pop();
        let actual = Filter::filter_words_with_options(&filter, words);
        assert_eq!(actual, expect);
    }

    #[test]
    fn can_convert_words_as_hungarian_notation() {
        let options = vec!["h"];
        let words: Vec<String> = vec!["intPageSize".to_string()];

        let actual = Filter::new(&options).unwrap()
            .to_naming_cases_from(words);
        assert_eq!(actual, vec![NamingCase::Pascal("PageSize".to_string())]);
    }

    #[test]
    fn can_convert_words_to_naming_cases() {
        let options = vec!["S", "s", "k", "c", "p"];
        let words: Vec<String> = vec!["SCREAMING_SNAKE", "snake_case",
                                      "kebab-case", "camelCase", "PascalCase",
                                      "-invalid_"].into_iter()
            .map(|s| s.to_string()).collect();

        let actual = Filter::new(&options).unwrap()
            .to_naming_cases_from(words);

        let expect = vec![
            NamingCase::ScreamingSnake("SCREAMING_SNAKE".to_string()),
            NamingCase::Snake("snake_case".to_string()),
            NamingCase::Kebab("kebab-case".to_string()),
            NamingCase::Camel("camelCase".to_string()),
            NamingCase::Pascal("PascalCase".to_string())];

        assert_eq!(actual, expect);
    }
}
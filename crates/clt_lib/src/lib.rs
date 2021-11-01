#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

pub use conversion::*;
pub use extraction::*;
use naming_lib::NamingCase;

mod extraction;
mod conversion;

lazy_static! {
    // default option of `--filter` and `--output`, i.e. allow all formats.
    static ref DEFAULT_OPTIONS: Vec<String> = to_string_vec(vec!["S", "s", "k", "c", "p"]);

    // used for converting NamingCase to String type in iteration.
    static ref DIRECT_MAPPERS: HashMap<&'static str, fn(&NamingCase) -> String> = {
        let mut map: HashMap<&'static str, fn(&NamingCase) -> String> = HashMap::new();
        map.insert("S", |case| case.to_screaming_snake().unwrap());
        map.insert("s", |case| case.to_snake().unwrap());
        map.insert("k", |case| case.to_kebab().unwrap());
        map.insert("c", |case| case.to_camel().unwrap());
        map.insert("p", |case| case.to_pascal().unwrap());
        map
    };

    // same as above.
    static ref JSON_MAPPERS:HashMap<&'static str, fn(&NamingCase) -> String> = {
        fn compose(key:&str,value:String) -> String {
              "\"".to_string() + key + "\":\"" + &value + "\""
        }

        let mut map: HashMap<&'static str, fn(&NamingCase) -> String> = HashMap::new();
        map.insert("S", |case| compose("screaming_snake",case.to_screaming_snake().unwrap()));
        map.insert("s", |case| compose("snake",case.to_snake().unwrap()));
        map.insert("k", |case| compose("kebab",case.to_kebab().unwrap()));
        map.insert("c", |case| compose("camel",case.to_camel().unwrap()));
        map.insert("p", |case| compose("pascal",case.to_pascal().unwrap()));
        map
    };
}

pub fn to_string_vec(ori: Vec<&str>) -> Vec<String> {
    ori.iter()
        .map(|str| str.to_string())
        .collect()
}
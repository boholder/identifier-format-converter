#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;

pub use conversion::*;
pub use extraction::*;
use naming_lib::NamingCase;

mod conversion;
mod extraction;

/// Intermediate type for converting
/// NamingCase type to String in different output format.
struct Formatter {
    pub inner: fn(&NamingCase) -> String,
}

impl Clone for Formatter {
    fn clone(&self) -> Self {
        Formatter { inner: self.inner }
    }
}

impl Copy for Formatter {}

lazy_static! {
    // default option of `--filter` and `--output`, i.e. allow all formats.
    static ref DEFAULT_OPTIONS: Vec<String> = to_string_vec(vec!["S", "s", "k", "c", "p"]);

    // used for converting NamingCase to String type in iteration.
    static ref DIRECT_MAPPERS: HashMap<&'static str, Formatter> = {
        let mut map: HashMap<&'static str, Formatter> = HashMap::new();
        map.insert("S", Formatter{inner:|case| case.to_screaming_snake().unwrap()});
        map.insert("s", Formatter{inner:|case| case.to_snake().unwrap()});
        map.insert("k", Formatter{inner:|case| case.to_kebab().unwrap()});
        map.insert("c", Formatter{inner:|case| case.to_camel().unwrap()});
        map.insert("p", Formatter{inner:|case| case.to_pascal().unwrap()});
        map
    };

    // same as above.
    static ref JSON_MAPPERS:HashMap<&'static str, Formatter> = {
        fn compose(key:&str,value:String) -> String {
              "\"".to_string() + key + "\":\"" + &value + "\""
        }

        let mut map: HashMap<&'static str, Formatter> = HashMap::new();
        map.insert("S", Formatter{
            inner:|case| compose("screaming_snake",case.to_screaming_snake().unwrap())
        });
        map.insert("s", Formatter{inner:|case| compose("snake",case.to_snake().unwrap())});
        map.insert("k", Formatter{inner:|case| compose("kebab",case.to_kebab().unwrap())});
        map.insert("c", Formatter{inner:|case| compose("camel",case.to_camel().unwrap())});
        map.insert("p", Formatter{inner:|case| compose("pascal",case.to_pascal().unwrap())});
        map
    };
}

pub fn to_string_vec(ori: Vec<&str>) -> Vec<String> {
    ori.iter().map(|str| str.to_string()).collect()
}

use lazy_static::*;
use regex::Regex;
use std::collections::HashMap;

use crate::common::str_to_numbers;

pub fn get_rule_set(input: &str) -> HashMap<Vec<u8>, u8> {
    lazy_static! {
        static ref PARSING_EXPR: Regex =
            Regex::new(r"^(?P<input>[#.]{5}) => (?P<output>[#.])").unwrap();
    }

    let mut map = HashMap::new();

    for line in input.lines() {
        let captures = PARSING_EXPR
            .captures(line)
            .unwrap_or_else(|| panic!(format!("Invalid line \"{}\"", line)));

        map.insert(
            str_to_numbers(&captures["input"]),
            str_to_numbers(&captures["output"])[0],
        );
    }

    map
}

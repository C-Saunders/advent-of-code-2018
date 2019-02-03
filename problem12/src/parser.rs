use lazy_static::*;
use regex::Regex;

#[derive(Debug)]
struct Rule {
    input: String,
    output: String,
}

impl Rule {
    fn new(input_line: &str) -> Self {
        lazy_static! {
            static ref PARSING_EXPR: Regex =
                Regex::new(r"^(?P<input>[#.]{5}) => (?P<output>[#.])").unwrap();
        }

        let captures = PARSING_EXPR
            .captures(input_line)
            .unwrap_or_else(|| panic!(format!("Invalid line \"{}\"", input_line)));

        Rule {
            input: captures["input"].trim().to_string(),
            output: captures["output"].trim().to_string(),
        }
    }
}

pub struct RuleSet

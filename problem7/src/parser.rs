use lazy_static::*;
use regex::Regex;
use std::str::FromStr;

#[derive(PartialEq, Eq, Debug)]
struct StepParseError();

#[derive(PartialEq, Eq, Debug)]
struct Edge {
    from: char,
    to: char,
}

impl FromStr for Edge {
    type Err = String;

    fn from_str(input_line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            // Step L must be finished before step M can begin.
            // Step B must be finished before step S can begin.
            static ref PARSING_EXPR: Regex =
                Regex::new(r"^Step (?P<from>[A-Z]) must be finished before step (?P<to>[A-Z]) can begin.$").unwrap();
        }

        let captures = PARSING_EXPR
            .captures(input_line)
            .ok_or(format!("Invalid line \"{}\"", input_line).to_owned())?;

        Ok(Edge {
            from: captures["from"].parse::<char>().unwrap(),
            to: captures["to"].parse::<char>().unwrap(),
        })
    }
}

#[cfg(test)]
mod test_edge_from_str {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(
            Ok(Edge { from: 'A', to: 'B' }),
            Edge::from_str("Step A must be finished before step B can begin.")
        )
    }

    #[test]
    fn bad_input() {
        assert!(Edge::from_str("Step A must be before step B.").is_err())
    }
}

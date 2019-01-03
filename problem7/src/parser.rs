use lazy_static::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;
use std::str::FromStr;

type StepName = char;

type PriorSteps = HashSet<StepName>;

#[derive(PartialEq, Eq, Debug)]
struct StepParseError();

#[derive(PartialEq, Eq, Debug)]
struct Edge {
    current_step: StepName,
    depends_on: StepName,
}

impl Edge {
    pub fn current_step(&self) -> StepName {
        self.current_step
    }

    pub fn depends_on(&self) -> StepName {
        self.depends_on
    }
}

impl FromStr for Edge {
    type Err = String; // TODO: figure out how to make this return a fancy custom error

    fn from_str(input_line: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref PARSING_EXPR: Regex =
                Regex::new(r"^Step (?P<depends_on>[A-Z]) must be finished before step (?P<current_step>[A-Z]) can begin.$").unwrap();
        }

        let captures = PARSING_EXPR
            .captures(input_line)
            .ok_or_else(|| format!("Invalid line \"{}\"", input_line).to_owned())?;

        Ok(Edge {
            depends_on: captures["depends_on"].parse::<StepName>().unwrap(),
            current_step: captures["current_step"].parse::<StepName>().unwrap(),
        })
    }
}

pub struct Graph(HashMap<StepName, PriorSteps>);

impl Graph {
    pub fn new(input: &str) -> Self {
        let mut graph: Graph = Graph(HashMap::with_capacity(input.len()));
        for line in input.lines() {
            let edge = Edge::from_str(line).unwrap();

            // make sure we create a root node
            graph
                .0
                .entry(edge.depends_on())
                .or_insert_with(HashSet::new);

            let current = graph
                .0
                .entry(edge.current_step())
                .or_insert_with(HashSet::new);
            current.insert(edge.depends_on());
        }

        graph
    }

    pub fn find_in_order(&self) -> String {
        let mut completed_steps: Vec<StepName> = vec![];
        let total_steps = self.0.keys().len();

        while completed_steps.len() < total_steps {
            let mut completable_steps: Vec<&StepName> = self
                .0
                .iter()
                .filter(|(step_name, prior_steps)| {
                    !completed_steps.contains(step_name)
                        && prior_steps.iter().all(|s| completed_steps.contains(s))
                })
                .map(|(step_name, _)| step_name)
                .collect();

            completable_steps.sort_unstable();

            completed_steps.push(**completable_steps.first().unwrap());
        }

        String::from_iter(completed_steps)
    }
}

#[cfg(test)]
mod test_edge_from_str {
    use super::Edge;
    use std::str::FromStr;

    #[test]
    fn test() {
        assert_eq!(
            Ok(Edge {
                depends_on: 'A',
                current_step: 'B'
            }),
            Edge::from_str("Step A must be finished before step B can begin.")
        )
    }

    #[test]
    fn bad_input() {
        assert!(Edge::from_str("Step A must be before step B.").is_err())
    }
}

#[cfg(test)]
mod test_graph_ordered {
    use super::Graph;

    #[test]
    fn single_node() {
        assert_eq!(
            Graph::new("Step A must be finished before step B can begin.").find_in_order(),
            "AB",
        )
    }

    #[test]
    fn one_branch() {
        assert_eq!(
            Graph::new("Step A must be finished before step B can begin.\nStep B must be finished before step C can begin.\nStep C must be finished before step D can begin.").find_in_order(),
            "ABCD",
        )
    }

    #[test]
    fn two_branches() {
        assert_eq!(
            Graph::new("Step A must be finished before step B can begin.\nStep B must be finished before step C can begin.\nStep A must be finished before step D can begin.").find_in_order(),
            "ABCD",
        )
    }
}

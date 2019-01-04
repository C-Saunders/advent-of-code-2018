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

type TimeRemaining = u32;

struct StepsInProgress(HashMap<StepName, TimeRemaining>);

impl StepsInProgress {
    fn new() -> Self {
        StepsInProgress(HashMap::with_capacity(5))
    }

    fn complete_steps(&mut self, time_jump: TimeRemaining) -> Vec<StepName> {
        for remaining_time in self.0.values_mut() {
            *remaining_time -= time_jump
        }

        let completed: Vec<StepName> = self
            .0
            .iter()
            .filter(|(_, time_remaining)| **time_remaining == 0)
            .map(|(step_name, _)| *step_name)
            .collect();

        for step in completed.iter() {
            self.0.remove(step);
        }

        completed
    }

    fn smallest_time_remaining(&self) -> u32 {
        *self
            .0
            .values()
            .fold(None, |min, curr| match min {
                None => Some(curr),
                Some(existing) => Some(if curr < existing { curr } else { existing }),
            })
            .unwrap_or(&0)
    }

    fn step_in_progress(&self, step_name: StepName) -> bool {
        self.0.keys().any(|key| key == &step_name)
    }

    fn replace_step(
        &mut self,
        old_step: StepName,
        new_step: StepName,
        new_step_duration: TimeRemaining,
    ) {
        self.0.remove(&old_step);
        self.start_step(new_step, new_step_duration);
    }

    fn start_step(&mut self, new_step: StepName, new_step_duration: TimeRemaining) {
        self.0.insert(new_step, new_step_duration);
    }

    fn num_available_workers(&self) -> usize {
        5 - self.0.keys().len()
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

    pub fn find_in_order_with_durations(&self) -> (String, u32) {
        let mut completed_steps: Vec<StepName> = vec![];
        let total_steps = self.0.keys().len();
        let mut in_progress = StepsInProgress::new();
        let mut elapsed_time = 0;

        while completed_steps.len() < total_steps {
            let time_jump = in_progress.smallest_time_remaining();
            elapsed_time += time_jump;

            let mut just_completed = in_progress.complete_steps(time_jump);
            just_completed.sort_unstable();
            completed_steps.append(&mut just_completed.clone());

            let mut available_steps: Vec<&StepName> = self
                .0
                .iter()
                .filter(|(step_name, prior_steps)| {
                    !completed_steps.contains(step_name)
                        && !in_progress.step_in_progress(**step_name)
                        && prior_steps.iter().all(|s| completed_steps.contains(s))
                })
                .map(|(step_name, _)| step_name)
                .collect();

            available_steps.sort_unstable();

            if available_steps.is_empty() {
                continue;
            }

            let mut available_steps_iter = available_steps.iter();

            // Fill open slots
            for _ in 1..=in_progress.num_available_workers() {
                if let Some(available) = available_steps_iter.next() {
                    in_progress.start_step(**available, get_step_duration(**available));
                } else {
                    break;
                }
            }

            // Replace completed steps
            for (completed, available) in just_completed.iter().zip(available_steps_iter) {
                in_progress.replace_step(*completed, **available, get_step_duration(**available));
            }
        }

        (String::from_iter(completed_steps), elapsed_time)
    }
}

fn get_step_duration(name: StepName) -> TimeRemaining {
    "ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .char_indices()
        .find(|(_, ch)| ch == &name)
        .unwrap()
        .0 as u32
        + 61
}

#[cfg(test)]
mod test_get_step_duration {
    use super::get_step_duration;

    #[test]
    fn test() {
        assert_eq!(get_step_duration('A'), 1 + 60);
        assert_eq!(get_step_duration('M'), 13 + 60);
        assert_eq!(get_step_duration('Z'), 26 + 60);
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

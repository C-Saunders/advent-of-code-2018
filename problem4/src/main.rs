use std::collections::HashMap;
use std::fs;
use std::result::Result;
#[macro_use]
extern crate lazy_static;
use crate::parser::*;

mod parser;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    run(&input).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug)]
struct SleepPeriod {
    start_minute: usize,
    end_minute: usize,
}

#[derive(PartialEq)]
enum State {
    Asleep,
    Awake,
}

fn run(input: &str) -> Result<(), String> {
    let mut guard_patterns: HashMap<usize, Vec<SleepPeriod>> = HashMap::new();
    let mut lines: Vec<&str> = input.lines().collect::<Vec<&str>>();
    lines.sort();

    let mut current_guard: Option<usize> = None;
    let mut guard_state: State = State::Awake;
    let mut fell_asleep_min: Option<usize> = None;

    for line in lines {
        let (data, line_type) = get_line_data(line)?;
        match line_type {
            LineType::NewGuard => {
                if guard_state == State::Asleep {
                    return Err("Can't end the shift asleep".to_owned())
                }
                current_guard = Some(data);
                guard_state = State::Awake;
            }
            LineType::FallAsleep => {
                if guard_state == State::Asleep {
                    return Err("Can't fall asleep when already asleep".to_owned())
                }
                guard_state = State::Asleep;
                fell_asleep_min = Some(data);
            }
            LineType::WakeUp => {
                if guard_state == State::Awake {
                    return Err("Can't wake up if already awake".to_owned())
                }
                guard_state = State::Awake;

                let new_sleep_period = SleepPeriod {
                    start_minute: fell_asleep_min.expect("Woke up without sleeping first"),
                    end_minute: data,
                };

                let current_guard_id =
                    current_guard.expect("Completed a sleep period without a guard being set");

                let curr = guard_patterns.entry(current_guard_id).or_insert(vec![]);
                curr.push(new_sleep_period);
            }
        }
    }

    println!("{:?}", guard_patterns);

    Ok(())
}

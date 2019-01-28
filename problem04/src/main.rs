#![feature(map_get_key_value)]

use std::collections::HashMap;
use std::fs;
use std::result::Result;
#[macro_use]
extern crate lazy_static;
use crate::parser::*;

mod parser;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let guard_patterns = get_parsed_patterns(&input)?;

    part1(&guard_patterns).map_err(|e| e.to_string())?;
    part2(&guard_patterns).map_err(|e| e.to_string())?;
    Ok(())
}

#[derive(Debug)]
struct SleepPeriod {
    start_minute: usize,
    end_minute: usize,
}

impl SleepPeriod {
    fn new(start: usize, end: usize) -> SleepPeriod {
        if start > end {
            panic!("End must be after start");
        }

        SleepPeriod {
            start_minute: start,
            end_minute: end,
        }
    }

    fn duration(&self) -> usize {
        self.end_minute - self.start_minute
    }
}

#[derive(PartialEq)]
enum State {
    Asleep,
    Awake,
}

fn part1(guard_patterns: &HashMap<usize, Vec<SleepPeriod>>) -> Result<(), String> {
    let highest_sleep_time_guard = get_highest_sleep_time_guard(&guard_patterns);
    let most_common_minute = get_most_common_minute(guard_patterns.get(&highest_sleep_time_guard).unwrap()).0;

    println!("Part1 (should be 131469):\nguard = {}, most common min = {}, product = {}", highest_sleep_time_guard, most_common_minute, highest_sleep_time_guard * most_common_minute);

    Ok(())
}

fn get_parsed_patterns(input: &str) -> Result<HashMap<usize, Vec<SleepPeriod>>, String> {
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

                let new_sleep_period = SleepPeriod::new(
                    fell_asleep_min.expect("Woke up without sleeping first"),
                    data,
                );

                let current_guard_id =
                    current_guard.expect("Completed a sleep period without a guard being set");

                let curr = guard_patterns.entry(current_guard_id).or_insert(vec![]);
                curr.push(new_sleep_period);
            }
        }
    }

    Ok(guard_patterns)
}

fn get_highest_sleep_time_guard(guard_sleep_periods: &HashMap<usize, Vec<SleepPeriod>>) -> usize {
    let raw_highest = guard_sleep_periods.get_key_value(guard_sleep_periods.keys().nth(0).unwrap()).unwrap();
    let mut highest = (raw_highest.0, get_total_sleep(&raw_highest.1));

    for (id, sleep_periods) in guard_sleep_periods.iter() {
        let current_total_sleep = get_total_sleep(&sleep_periods);
        if current_total_sleep > highest.1 {
            highest = (id, current_total_sleep);
        }
    }

    *highest.0
}

fn get_total_sleep(periods: &Vec<SleepPeriod>) -> usize {
    periods.iter().fold(0, |acc, current| acc + current.duration())
}

fn get_most_common_minute(periods: &Vec<SleepPeriod>) -> (usize, usize) {
    let minute_occurrences = get_occurence_counts(&periods);
    let random_key = *minute_occurrences.keys().nth(0).unwrap();
    let random_key_value = *minute_occurrences.get(&random_key).unwrap();

    let mut most_common = (random_key, random_key_value);

    for (min, count) in minute_occurrences {
        if count > most_common.1 {
            most_common = (min, count);
        }
    }

    most_common
}

fn get_occurence_counts(periods: &Vec<SleepPeriod>) -> HashMap<usize, usize> {
    let mut minute_occurrences: HashMap<usize, usize> = HashMap::new();

    for period in periods.iter() {
        for min in period.start_minute..period.end_minute {
            let counter = minute_occurrences.entry(min).or_insert(0);
            *counter += 1;
        }
    }

    minute_occurrences
}

#[derive(Debug)]
struct MostFrequentSleepingMinute {
    id: usize,
    minute: usize,
    count: usize,
}

fn part2(guard_patterns: &HashMap<usize, Vec<SleepPeriod>>) -> Result<(), String> {
    let mut most_frequent = MostFrequentSleepingMinute {
        id: 0,
        minute: 0,
        count: 0,
    };

    for (curr_id, sleep_periods) in guard_patterns.iter() {
        let (min, curr_count) = get_most_common_minute(&sleep_periods);
        if curr_count > most_frequent.count {
            most_frequent = MostFrequentSleepingMinute {
                id: *curr_id,
                minute: min,
                count: curr_count,
            }
        }
    }

    println!("{:?}", most_frequent);
    // MostFrequentSleepingMinute { id: 1901, minute: 51, count: 19 }
    // 1901 * 51 = 96,951
    
    Ok(())
}

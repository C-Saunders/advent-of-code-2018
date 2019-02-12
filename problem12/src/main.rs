extern crate problem12;
use std::collections::HashMap;
use problem12::{rule_set, plants};
use std::fs;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let rules = rule_set::get_rule_set(&input);

    let initial_state = "#.##.#.##..#.#...##...#......##..#..###..##..#.#.....##..###...#.#..#...######...#####..##....#..###";

    assert_eq!(2166, evaluate_generations(&initial_state, &rules, 20));
    
    // Turned out to be a pattern: 21(0{N-1})61, where N = the number of zeros
    // Similar patterns emerged for 1E3+, etc.
    // dbg!(evaluate_generations(&initial_state, &rules, 50000000000));

    Ok(())
}

fn evaluate_generations(
    initial_state: &str,
    rules: &HashMap<Vec<u8>, u8>,
    num_generations: u64,
) -> i32 {
    let mut plants = plants::PlantSet::from_string(initial_state);

    for _ in 0..num_generations {
        plants.get_next_generation(rules);
    }

    plants.get_sum_of_indexes()
}

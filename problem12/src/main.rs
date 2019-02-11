extern crate problem12;
use std::collections::HashMap;
use problem12::{rule_set, plants};
use std::fs;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let rules = rule_set::get_rule_set(&input);

    let initial_state = "#.##.#.##..#.#...##...#......##..#..###..##..#.#.....##..###...#.#..#...######...#####..##....#..###";
    // let initial_state = "#.##";

    evaluate_generations(&initial_state, &rules, 2);

    Ok(())
}

fn evaluate_generations(
    initial_state: &str,
    rules: &HashMap<Vec<u8>, u8>,
    num_generations: u32,
) {
    let mut new_gen = plants::PlantSet::from_string(initial_state);

    for _ in 0..num_generations {
        new_gen = new_gen.get_next_generation(rules);
        println!("{:?}", new_gen);
    }
}

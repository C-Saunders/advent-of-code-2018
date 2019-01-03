use std::fs;
extern crate lazy_static;

mod parser;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    part1(&input);
    Ok(())
}

fn part1(input: &str) {
    println!("{}", parser::Graph::new(input).find_in_order());
}

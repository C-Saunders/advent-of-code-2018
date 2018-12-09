use std::fs;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/data.txt")?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut current_line_map: HashMap<char, i32> = HashMap::new();

    let mut has_double_letter_count = 0;
    let mut has_triple_letter_count = 0;

    for line in input.lines() {
        current_line_map.clear();

        for item in line.chars() {
            if let Some(val) = current_line_map.get_mut(&item) {
                *val = *val + 1;
            } else {
                current_line_map.insert(item, 1);
            }
        }

        if current_line_map.values().any(|i| *i == 2) {
            has_double_letter_count = has_double_letter_count + 1;
        }

        if current_line_map.values().any(|i| *i == 3) {
            has_triple_letter_count = has_triple_letter_count + 1;
        }
    }

    println!("Doubles * Triples = {}", has_double_letter_count * has_triple_letter_count);
    Ok(())
}

fn part2(_input: &str) -> Result<()> {
    println!("Not implemented");
    Ok(())
}
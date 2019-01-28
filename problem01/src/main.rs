use std::fs;
use std::io::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/data.txt")?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut current_value = 0;

    for line in input.lines() {
        current_value = current_value + line.parse::<i32>().unwrap();
    }
    println!("Part 1 = {}", current_value);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    let mut current_value = 0;
    let mut seen_values: HashSet<i32> = HashSet::new();

    seen_values.insert(current_value);

    'outer: loop {
        for line in input.lines() {
            current_value = current_value + line.parse::<i32>().unwrap();

            if seen_values.contains(&current_value) {
                println!("Part 2 = {}", current_value);
                break 'outer;
            }
            
            seen_values.insert(current_value);
        }
    }
    Ok(())
}

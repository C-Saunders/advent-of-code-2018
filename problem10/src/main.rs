use std::fs;
use crate::parser::PointData;

mod parser;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    
    for line in input.lines() {
        println!("{:?}", PointData::new(line));
    }

    Ok(())
}

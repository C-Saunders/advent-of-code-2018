use std::fs;
extern crate lazy_static;

mod parser;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    Ok(())
}

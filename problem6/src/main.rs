use crate::helpers::get_node_list;
use crate::part1::*;
use std::fs;

mod helpers;
mod part1;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let node_list = get_node_list(&input)?;
    part1(&node_list);
    Ok(())
}

use crate::helpers::get_node_list;
use crate::part1::*;
use crate::part2::*;
use std::fs;

mod helpers;
mod part1;
mod part2;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let node_list = get_node_list(&input)?;
    part1(&node_list);
    part2(&node_list);
    Ok(())
}

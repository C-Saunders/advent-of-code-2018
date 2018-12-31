use crate::helpers::*;
use std::collections::{HashMap, HashSet};

pub fn part1(node_list: &NodeList) -> () {
    let total_area: Area = Area::from_node_list(&node_list);
    println!("Input node total area = {:?}", total_area);
    let grid = calculate_distances(&node_list, &total_area);
    let largest_area = find_largest_non_infinite_area(&grid);
    assert_eq!(4233, largest_area);
    println!("Part 1 largest area = {}", largest_area);
}

fn find_largest_non_infinite_area(grid: &Box<Grid>) -> i32 {
    let mut infinite_area_nodes: HashSet<InputNode> = HashSet::new();
    let mut area_totals: HashMap<Point, i32> = HashMap::new();

    for grid_point in grid.iter() {
        let sole_closest = grid_point.get_sole_closest_node();
        if let Some(closest) = sole_closest {
            if grid_point.is_exterior() {
                infinite_area_nodes.insert(closest);
            }

            if !infinite_area_nodes.contains(&closest) {
                let node_total = area_totals.entry(closest).or_insert(0);
                *node_total = *node_total + 1;
            }
        }
    }

    println!("Infinite area nodes = {:?}", infinite_area_nodes);
    println!("Area totals = {:?}", area_totals);

    *area_totals
        .iter()
        .fold(None, |max, curr| match max {
            None => Some(curr.1),
            Some(existing) => Some(if curr.1 > existing { curr.1 } else { existing }),
        })
        .unwrap()
}

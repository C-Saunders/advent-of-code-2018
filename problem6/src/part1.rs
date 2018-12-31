use crate::helpers::*;
use std::collections::{HashMap, HashSet};

pub fn part1(node_list: &NodeList) -> () {
    let total_area: Area = find_total_area_dimensions(&node_list);
    println!("{:?}", total_area);
    let grid = calculate_distances(&node_list, &total_area);
    let largest_area = find_largest_non_infinite_area(&grid);
    assert_eq!(4233, largest_area);
    println!("largest area = {}", largest_area);
}

fn find_total_area_dimensions(node_list: &NodeList) -> Area {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for current_point in node_list.iter() {
        if current_point.x < min_x {
            min_x = current_point.x
        }
        if current_point.y < min_y {
            min_y = current_point.y
        }
        if current_point.x > max_x {
            max_x = current_point.x
        }
        if current_point.y > max_y {
            max_y = current_point.y
        }
    }

    Area::new(min_x, min_y, max_x, max_y)
}

fn calculate_distances(node_list: &NodeList, total_area: &Area) -> Box<Grid> {
    let mut grid: Grid = vec![];
    for curr_x in total_area.min_x()..=total_area.max_x() {
        for curr_y in total_area.min_y()..=total_area.max_y() {
            let curr_point = Point::new(curr_x, curr_y);

            let mut curr_grid_point =
                GridPoint::new(is_exterior_point(&curr_point, &total_area), HashMap::new());

            for node in node_list.iter() {
                curr_grid_point.insert_distance(*node, manhattan_distance(&curr_point, &node));
            }

            grid.push(curr_grid_point);
        }
    }

    Box::new(grid)
}

fn is_exterior_point(point: &Point, area: &Area) -> bool {
    point.x == area.min_x()
        || point.x == area.max_x()
        || point.y == area.min_y()
        || point.y == area.max_y()
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

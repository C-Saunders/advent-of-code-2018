use crate::helpers::*;

pub fn part2(node_list: &NodeList) -> () {
    const MAX_DISTANCE: i32 = 10000;
    let total_area = calculate_max_possible_area(node_list, MAX_DISTANCE);
    println!("Part 2 search total area = {:?}", total_area);
    let grid = calculate_distances(&node_list, &total_area);
    println!("Part 2 area = {}", find_area(&grid, MAX_DISTANCE));
}

fn calculate_max_possible_area(node_list: &NodeList, max_distance: i32) -> Area {
    let bounding_area = Area::from_node_list(node_list);

    let min_x =
        ((bounding_area.max_x() - max_distance) as f32 / node_list.len() as f32).floor() as i32;
    let min_y =
        ((bounding_area.max_y() - max_distance) as f32 / node_list.len() as f32).floor() as i32;
    let max_x =
        ((bounding_area.min_x() + max_distance) as f32 / node_list.len() as f32).floor() as i32;
    let max_y =
        ((bounding_area.min_y() + max_distance) as f32 / node_list.len() as f32).floor() as i32;

    Area::new(min_x, min_y, max_x, max_y)
}

fn find_area(grid: &Box<Grid>, max_distance: i32) -> i32 {
   grid.iter().filter(|grid_point| grid_point.total_distance() < max_distance).count() as i32
}
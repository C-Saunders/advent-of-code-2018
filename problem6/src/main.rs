use std::collections::{HashMap, HashSet};
use std::fs;
use std::num::ParseIntError;
use std::result::Result;
use std::str::FromStr;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let node_list = get_node_list(&input)?;
    let total_area: Area = find_total_area_dimensions(&node_list);
    println!("{:?}", total_area);
    let grid = calculate_distances(&node_list, &total_area);
    let largest_area = find_largest_non_infinite_area(&grid, &total_area);
    println!("largest area = {}", largest_area);
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

type InputNode = Point;

type NodeList = Vec<InputNode>;

impl Point {
    fn new(x_val: i32, y_val: i32) -> Self {
        Point { x: x_val, y: y_val }
    }
}

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let coords: Vec<&str> = s.split(", ").collect();

        let x_fromstr = coords[0].parse::<i32>()?;
        let y_fromstr = coords[1].parse::<i32>()?;

        Ok(Point {
            x: x_fromstr,
            y: y_fromstr,
        })
    }
}

#[derive(Debug)]
struct Area {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

#[derive(Debug)]
struct GridPoint {
    point: Point,
    is_exterior: bool,
    distances: HashMap<InputNode, i32>,
}

type Grid = Vec<GridPoint>;

fn get_node_list(input: &str) -> Result<NodeList, String> {
    let mut node_list: NodeList = vec![];
    for line in input.lines() {
        node_list.push(InputNode::from_str(line).map_err(|e| e.to_string())?);
    }

    Ok(node_list)
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

    Area {
        min_x,
        min_y,
        max_x,
        max_y,
    }
}

fn calculate_distances(node_list: &NodeList, total_area: &Area) -> Box<Grid> {
    let mut grid: Grid = vec![];
    for curr_x in total_area.min_x..=total_area.max_x {
        for curr_y in total_area.min_y..=total_area.max_y {
            let curr_point = Point::new(curr_x, curr_y);

            let mut curr_grid_point = GridPoint {
                point: curr_point,
                is_exterior: is_exterior_point(&curr_point, &total_area),
                distances: HashMap::new(),
            };

            let curr_distances = &mut curr_grid_point.distances;

            for node in node_list.iter() {
                curr_distances.insert(*node, manhattan_distance(&curr_point, &node));
            }

            grid.push(curr_grid_point);
        }
    }

    Box::new(grid)
}

fn is_exterior_point(point: &Point, area: &Area) -> bool {
    point.x == area.min_x || point.x == area.max_x || point.y == area.min_y || point.y == area.max_y
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn find_largest_non_infinite_area(grid: &Box<Grid>, area: &Area) -> i32 {
    let mut infinite_area_nodes: HashSet<InputNode> = HashSet::new();
    let mut area_totals: HashMap<Point, i32> = HashMap::new();

    for grid_point in grid.iter() {
        let sole_closest = calculate_sole_closest(grid_point);
        if  sole_closest.is_some() {
            let closest = sole_closest.unwrap();

            if is_exterior_point(&grid_point.point, area) {
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

    *area_totals.iter().fold(None, |max, curr| match max {
        None => Some(curr.1),
        Some(existing) => Some(if curr.1 > existing { curr.1 } else { existing })
    }).unwrap()
}

fn calculate_sole_closest(grid_point: &GridPoint) -> Option<InputNode> {
    let mut sorted = grid_point.distances.iter().collect::<Vec<(&InputNode, &i32)>>();
    sorted.sort_by(|a, b| a.1.cmp(&b.1));

    let shortest_distance: i32 = *sorted[0].1;

    let closest = sorted.into_iter()
        .take_while(|(_, distance)| **distance == shortest_distance)
        .collect::<Vec<(&InputNode, &i32)>>();

    match closest.len() {
        0 => panic!("Unable to find closest node"),
        1 => Some(*closest[0].0),
        _ => None,
    }
}

#[cfg(test)]
mod test_manhattan_distance {
    use super::{manhattan_distance, Point};

    #[test]
    fn zero_distance() {
        assert_eq!(manhattan_distance(&Point::new(0, 0), &Point::new(0, 0)), 0);

        assert_eq!(
            manhattan_distance(&Point::new(10, 30), &Point::new(10, 30)),
            0
        );
    }

    #[test]
    fn nonzero_distance() {
        assert_eq!(manhattan_distance(&Point::new(1, 0), &Point::new(0, 0)), 1);

        assert_eq!(
            manhattan_distance(&Point::new(1, 30), &Point::new(10, 40)),
            19
        );

        assert_eq!(
            manhattan_distance(&Point::new(10, 40), &Point::new(1, 30)),
            19
        );
    }
}

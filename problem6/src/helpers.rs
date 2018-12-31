use std::collections::HashMap;
use std::num::ParseIntError;
use std::result::Result;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub type InputNode = Point;

pub type NodeList = Vec<InputNode>;

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
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
pub struct Area {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Area {
    pub fn min_x(&self) -> i32 {
        self.min_x
    }
    pub fn min_y(&self) -> i32 {
        self.min_y
    }
    pub fn max_x(&self) -> i32 {
        self.max_x
    }
    pub fn max_y(&self) -> i32 {
        self.max_y
    }

    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        Area {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}

#[derive(Debug)]
pub struct GridPoint {
    is_exterior: bool,
    distances: HashMap<InputNode, i32>,
    sole_closest_node: Option<InputNode>,
}

impl GridPoint {
    pub fn new(is_exterior: bool, distances: HashMap<InputNode, i32>) -> Self {
        GridPoint {
            is_exterior,
            distances,
            sole_closest_node: None,
        }
    }

    pub fn is_exterior(&self) -> bool {
        self.is_exterior
    }

    pub fn insert_distance(&mut self, node: InputNode, distance: i32) -> Option<i32> {
        self.distances.insert(node, distance)
    }

    pub fn get_sole_closest_node(&self) -> Option<InputNode> {
        let mut sorted = self.distances.iter().collect::<Vec<(&InputNode, &i32)>>();
        sorted.sort_by(|a, b| a.1.cmp(&b.1));

        let shortest_distance: i32 = *sorted[0].1;

        let closest = sorted
            .into_iter()
            .take_while(|(_, distance)| **distance == shortest_distance)
            .collect::<Vec<(&InputNode, &i32)>>();

        match closest.len() {
            0 => panic!("Unable to find closest node"),
            1 => Some(*closest[0].0),
            _ => None,
        }
    }
}

pub type Grid = Vec<GridPoint>;

pub fn get_node_list(input: &str) -> Result<NodeList, String> {
    let mut node_list: NodeList = vec![];
    for line in input.lines() {
        node_list.push(InputNode::from_str(line).map_err(|e| e.to_string())?);
    }

    Ok(node_list)
}

pub fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
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

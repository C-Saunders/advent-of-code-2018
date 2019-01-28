use lazy_static::*;
use regex::Regex;
use std::cmp;

#[derive(PartialEq, Eq, Debug)]
pub struct PointData {
    pub x_pos: i32,
    pub y_pos: i32,
    pub x_vel: i32,
    pub y_vel: i32,
}

impl PointData {
    pub fn new(input_line: &str) -> Self {
        lazy_static! {
            static ref PARSING_EXPR: Regex =
                Regex::new(r"^position=<(?P<x_pos>\s*-?\d+), (?P<y_pos>\s*-?\d+)> velocity=<(?P<x_vel>\s*-?\d+), (?P<y_vel>\s*-?\d+)>$").unwrap();
        }

        let captures = PARSING_EXPR
            .captures(input_line)
            .unwrap_or_else(|| panic!(format!("Invalid line \"{}\"", input_line)));

        PointData {
            x_pos: captures["x_pos"].trim().parse::<i32>().unwrap(),
            y_pos: captures["y_pos"].trim().parse::<i32>().unwrap(),
            x_vel: captures["x_vel"].trim().parse::<i32>().unwrap(),
            y_vel: captures["y_vel"].trim().parse::<i32>().unwrap(),
        }
    }
}

pub struct PointSet {
    data: Vec<PointData>,
    time: u32,
}

impl PointSet {
    pub fn new(input: &str) -> Self {
        PointSet {
            data: input.lines().map(|line| PointData::new(line)).collect(),
            time: 0,
        }
    }

    pub fn move_points(&mut self) {
        self.time += 1;

        for point in self.data.iter_mut() {
            point.x_pos += point.x_vel;
            point.y_pos += point.y_vel;
        }
    }

    pub fn move_points_backwards(&mut self) {
        self.time -= 1;

        for point in self.data.iter_mut() {
            point.x_pos -= point.x_vel;
            point.y_pos -= point.y_vel;
        }
    }

    pub fn bounding_box(&self) -> BoundingBox {
        let mut min_x = None;
        let mut min_y = None;
        let mut max_x = None;
        let mut max_y = None;

        for current_point in self.data.iter() {
            min_x = Some(min_x.map_or(current_point.x_pos, |x| cmp::min(current_point.x_pos, x)));

            min_y = Some(min_y.map_or(current_point.y_pos, |y| cmp::min(current_point.y_pos, y)));

            max_x = Some(max_x.map_or(current_point.x_pos, |x| cmp::max(current_point.x_pos, x)));

            max_y = Some(max_y.map_or(current_point.y_pos, |y| cmp::max(current_point.y_pos, y)));
        }

        BoundingBox::new(
            min_x.unwrap(),
            min_y.unwrap(),
            max_x.unwrap(),
            max_y.unwrap(),
        )
    }

    pub fn print_set(&self) {
        let bb = self.bounding_box();
        let mut output = format!("At time {}\n", self.time);

        for y in bb.min_y..=bb.max_y {
            for x in bb.min_x..=bb.max_x {
                if self.data.iter().any(|point| point.x_pos == x && point.y_pos == y) {
                    output.push_str("#");
                } else {
                    output.push_str(" ");
                }
            }
            output.push_str("\n");
        }

        println!("{}", output);
    }
}

#[derive(Debug)]
pub struct BoundingBox {
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
}

impl BoundingBox {
    fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Self {
        BoundingBox {
            min_x, min_y, max_x, max_y
        }
    }

    pub fn area(&self) -> i64 {
        (self.max_x - self.min_x) as i64 * (self.max_y - self.min_y) as i64
    }
}

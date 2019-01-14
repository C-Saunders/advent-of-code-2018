use lazy_static::*;
use regex::Regex;

#[derive(PartialEq, Eq, Debug)]
pub struct PointData {
    x_pos: i32,
    y_pos: i32,
    x_vel: i32,
    y_vel: i32,
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

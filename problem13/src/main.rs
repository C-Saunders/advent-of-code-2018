use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let (parsed_grid, carts) = parse(&input);

    dbg!(parsed_grid);

    Ok(())
}

fn parse(input: &str) -> (HashMap<Point, TrackType>, Vec<Cart>) {
    let mut grid: HashMap<Point, TrackType> = HashMap::new();
    let mut carts: Vec<Cart> = vec![];

    let raw_grid: RawGrid = input.lines().map(|line| line.chars().collect()).collect();

    for (y_val, line) in raw_grid.iter().enumerate() {
        for (x_val, current) in line.iter().enumerate() {
            if current.is_whitespace() {
                continue;
            }

            let curr_point = Point::new(x_val, y_val);

            if let Some(cart_orientation) = Orientation::from_char(&current) {
                carts.push(Cart::new(curr_point.clone(), current));
            }

            if let Some(track_type) = TrackType::from_char(&current) {
                grid.insert(curr_point, track_type);
            } else {
                grid.insert(
                    curr_point,
                    TrackType::infer_type(&current, &curr_point, &raw_grid)
                        .expect("Failed to inter track type"),
                );
            }
        }
    }

    (grid, carts)
}

type RawGrid = Vec<Vec<char>>;

struct Cart {
    location: Point,
    orientation: Orientation,
    next_turn_direction: TurnDirection,
}

impl Cart {
    fn new(location: Point, symbol: &char) -> Self {
        Cart {
            location,
            orientation: Orientation::from_char(symbol).expect("Unknown cart symbol"),
            next_turn_direction: TurnDirection::LEFT,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

enum Orientation {
    Up,
    Down,
    Left,
    Right,
}

impl Orientation {
    fn from_char(ch: &char) -> Option<Orientation> {
        match ch {
            '^' => Some(Orientation::Up),
            'v' => Some(Orientation::Down),
            '<' => Some(Orientation::Left),
            '>' => Some(Orientation::Right),
            _ => None,
        }
    }
}

enum TurnDirection {
    LEFT,
    STRAIGHT,
    RIGHT,
}

#[derive(Debug)]
enum TrackType {
    Vertical,
    Horizontal,
    Intersection,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl TrackType {
    fn from_char(ch: &char) -> Option<TrackType> {
        match ch {
            '|' => Some(TrackType::Vertical),
            '-' => Some(TrackType::Horizontal),
            '+' => Some(TrackType::Intersection),
            _ => None,
        }
    }

    fn infer_type(current_char: &char, current_point: &Point, grid: &RawGrid) -> Option<TrackType> {
        // get first useful symbol left, right, up, and down
        let left = get_left(&current_point, &grid);
        let right = get_right(&current_point, &grid);
        let up = get_up(&current_point, &grid);
        let down = get_down(&current_point, &grid);

        // there are no instances of corners next to each other on the same track
        // -> we'll only look for "straight" segments
        // based on the data, we don't need to worry aout
        // -> a cart starting on a corner
        // -> a cart starting at an intersection

        match current_char {
            '/' => {
                if right.map_or(false, |v| v == '-') || down.map_or(false, |v| v == '|') {
                    Some(TrackType::TopLeft)
                } else if left.map_or(false, |v| v == '-') || up.map_or(false, |v| v == '|') {
                    Some(TrackType::BottomRight)
                } else {
                    None
                }
            }
            '\\' => {
                if left.map_or(false, |v| v == '-') || down.map_or(false, |v| v == '|') {
                    Some(TrackType::TopRight)
                } else if right.map_or(false, |v| v == '-') || up.map_or(false, |v| v == '|') {
                    Some(TrackType::BottomLeft)
                } else {
                    None
                }
            }
            '>' | '<' => Some(TrackType::Horizontal),
            '^' | 'v' => Some(TrackType::Vertical),
            _ => None,
        }
    }
}

fn get_left(point: &Point, grid: &RawGrid) -> Option<char> {
    if point.x == 0 {
        return None;
    }

    for current_x in (0..point.x).rev() {
        if grid[point.y][current_x] != '+' {
            return Some(grid[point.y][current_x]);
        }
    }

    None
}

fn get_right(point: &Point, grid: &RawGrid) -> Option<char> {
    if point.x == grid[0].len() {
        return None;
    }

    for current_x in (point.x + 1)..grid[0].len() {
        if grid[point.y][current_x] != '+' {
            return Some(grid[point.y][current_x]);
        }
    }

    None
}

fn get_up(point: &Point, grid: &RawGrid) -> Option<char> {
    if point.y == 0 {
        return None;
    }

    for current_y in (0..point.y).rev() {
        if grid[current_y][point.x] != '+' {
            return Some(grid[current_y][point.x]);
        }
    }

    None
}

fn get_down(point: &Point, grid: &RawGrid) -> Option<char> {
    if point.y == grid.len() {
        return None;
    }

    for current_y in (point.y + 1)..grid.len() {
        if grid[current_y][point.x] != '+' {
            return Some(grid[current_y][point.x]);
        }
    }

    None
}

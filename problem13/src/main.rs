#![feature(drain_filter)]
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fs;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;
    let (mut carts, parsed_grid) = parse(&input);

    get_crash(&mut carts, &parsed_grid);

    Ok(())
}

fn parse(input: &str) -> (Vec<Cart>, HashMap<Point, TrackType>) {
    let mut grid: HashMap<Point, TrackType> = HashMap::new();
    let mut carts: Vec<Cart> = vec![];

    let raw_grid: RawGrid = input.lines().map(|line| line.chars().collect()).collect();

    for (y_val, line) in raw_grid.iter().enumerate() {
        for (x_val, current) in line.iter().enumerate() {
            if current.is_whitespace() {
                continue;
            }

            let curr_point = Point::new(x_val, y_val);

            if Orientation::from_char(&current).is_some() {
                carts.push(Cart::new(curr_point.clone(), current));
            }

            if let Some(track_type) = TrackType::from_char(&current) {
                grid.insert(curr_point, track_type);
            } else {
                grid.insert(
                    curr_point,
                    TrackType::infer_type(&current, &curr_point, &raw_grid)
                        .expect("Failed to infer track type"),
                );
            }
        }
    }

    (carts, grid)
}

fn get_crash(carts: &mut Vec<Cart>, tracks: &HashMap<Point, TrackType>) {
    let mut last_len = carts.len();
    loop {
        do_tick(carts, tracks);

        if last_len != carts.len() {
            println!("{}", carts.len());
            last_len = carts.len();
        }

        if carts.len() <= 1 {
            dbg!(carts);
            break;
        }
    }
}

fn do_tick(carts: &mut Vec<Cart>, tracks: &HashMap<Point, TrackType>) {
    carts.sort();

    let mut crashed_point: Option<Point> = None;
    let mut occupied_points: HashSet<Point> = HashSet::new();
    for cart in carts.iter() {
        occupied_points.insert(cart.location.clone());
    }

    carts.drain_filter(|cart| {
        occupied_points.remove(&cart.location);

        if crashed_point.is_some() && crashed_point.unwrap() == cart.location {
            crashed_point = None;
            return true;
        }

        match cart.orientation {
            Orientation::Up => {
                cart.location.y -= 1;

                match tracks.get(&cart.location).expect("Cart off the track") {
                    TrackType::TopLeft => cart.orientation = Orientation::Right,
                    TrackType::TopRight => cart.orientation = Orientation::Left,
                    TrackType::Intersection => {
                        cart.orientation = match cart.next_turn_direction {
                            TurnDirection::Left => Orientation::Left,
                            TurnDirection::Straight => Orientation::Up,
                            TurnDirection::Right => Orientation::Right,
                        };
                        cart.next_turn_direction = cart.get_new_next_turn_direction();
                    }
                    _ => {}
                }
            }
            Orientation::Down => {
                cart.location.y += 1;

                match tracks.get(&cart.location).expect("Cart off the track") {
                    TrackType::BottomLeft => cart.orientation = Orientation::Right,
                    TrackType::BottomRight => cart.orientation = Orientation::Left,
                    TrackType::Intersection => {
                        cart.orientation = match cart.next_turn_direction {
                            TurnDirection::Left => Orientation::Right,
                            TurnDirection::Straight => Orientation::Down,
                            TurnDirection::Right => Orientation::Left,
                        };
                        cart.next_turn_direction = cart.get_new_next_turn_direction();
                    }
                    _ => {}
                }
            }
            Orientation::Left => {
                cart.location.x -= 1;

                match tracks.get(&cart.location).expect("Cart off the track") {
                    TrackType::TopLeft => cart.orientation = Orientation::Down,
                    TrackType::BottomLeft => cart.orientation = Orientation::Up,
                    TrackType::Intersection => {
                        cart.orientation = match cart.next_turn_direction {
                            TurnDirection::Left => Orientation::Down,
                            TurnDirection::Straight => Orientation::Left,
                            TurnDirection::Right => Orientation::Up,
                        };
                        cart.next_turn_direction = cart.get_new_next_turn_direction();
                    }
                    _ => {}
                }
            }
            Orientation::Right => {
                cart.location.x += 1;

                match tracks.get(&cart.location).expect("Cart off the track") {
                    TrackType::TopRight => cart.orientation = Orientation::Down,
                    TrackType::BottomRight => cart.orientation = Orientation::Up,
                    TrackType::Intersection => {
                        cart.orientation = match cart.next_turn_direction {
                            TurnDirection::Left => Orientation::Up,
                            TurnDirection::Straight => Orientation::Right,
                            TurnDirection::Right => Orientation::Down,
                        };
                        cart.next_turn_direction = cart.get_new_next_turn_direction();
                    }
                    _ => {}
                }
            }
        }

        if occupied_points.contains(&cart.location) {
            // crash
            crashed_point = Some(cart.location);
            return true;
        }

        occupied_points.insert(cart.location.clone());
        return false;
    });

    // need to filter again in case we got a crash in a later iteration
    carts.drain_filter(|cart| crashed_point.is_some() && crashed_point.unwrap() == cart.location);
}

type RawGrid = Vec<Vec<char>>;

#[derive(Debug, Eq)]
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
            next_turn_direction: TurnDirection::Left,
        }
    }

    fn get_new_next_turn_direction(&self) -> TurnDirection {
        match self.next_turn_direction {
            TurnDirection::Left => TurnDirection::Straight,
            TurnDirection::Straight => TurnDirection::Right,
            TurnDirection::Right => TurnDirection::Left,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Ord for Cart {
    fn cmp(&self, other: &Cart) -> Ordering {
        let self_loc = self.location;
        let other_loc = other.location;

        if self_loc.y == other_loc.y {
            self_loc.x.cmp(&other_loc.x)
        } else {
            self_loc.y.cmp(&other_loc.y)
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Cart) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cart {
    fn eq(&self, other: &Cart) -> bool {
        self.location == other.location
            && self.orientation == other.orientation
            && self.next_turn_direction == other.next_turn_direction
    }
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

#[derive(PartialEq, Eq, Debug)]
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

#[derive(PartialEq, Eq, Debug)]
enum TurnDirection {
    Left,
    Straight,
    Right,
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

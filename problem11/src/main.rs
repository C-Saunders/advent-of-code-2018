use std::collections::HashMap;

fn main() {
    let grid = get_populated_grid();

    let mut highest = 0;
    let mut highest_point = (0, 0);

    for x in 0..=297 {
        for y in 0..=297 {
            let window_value = get_window_value((x, y), &grid);
            if window_value > highest {
                highest = window_value;
                highest_point = (x, y);
            }
        }
    }

    dbg!(highest);
    dbg!(highest_point);
}

// very sloppy types due to laziness
fn get_populated_grid() -> HashMap<(i32, i32), i32> {
    let serial_number = 6878;

    /*
    Find the fuel cell's rack ID, which is its X coordinate plus 10.
    Begin with a power level of the rack ID times the Y coordinate.
    Increase the power level by the value of the grid serial number (your puzzle input).
    Set the power level to itself multiplied by the rack ID.
    Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
    Subtract 5 from the power level.
    */

    let mut grid: HashMap<(i32, i32), i32> = HashMap::with_capacity(300 * 300);

    for x in 0..300 {
        for y in 0..300 {
            let rack_id = x + 10;
            let mut power_level: i32 = rack_id * y;
            power_level += serial_number;
            power_level = rack_id * power_level;
            power_level = get_hundreds_digit(power_level);
            power_level -= 5;

            grid.insert((x, y), power_level);
        }
    }

    grid
}

fn get_hundreds_digit(number: i32) -> i32 {
    let as_str = number.to_string();
    if as_str.len() < 3 {
        return 0
    }

    let index = as_str.len() - 3;
    as_str.get(index..=index).map_or(0, |v| v.parse::<i32>().unwrap())
}

fn get_window_value(point: (i32, i32), grid: &HashMap<(i32, i32), i32>)-> i32 {
    let point_x = point.0;
    let point_y = point.1;

    let mut sum = 0;

    for x in point_x..=point_x+2 {
        for y in point_y..=point_y+2 {
            sum += grid.get(&(x, y)).unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod test_get_hundreds_digit {
    use super::get_hundreds_digit;

    #[test]
    fn under_100() {
        assert_eq!(get_hundreds_digit(6), 0);
    }

    #[test]
    fn three_digit() {
        assert_eq!(get_hundreds_digit(683), 6);
    }

    #[test]
    fn many_digit() {
        assert_eq!(get_hundreds_digit(7813109), 1);
    }
}

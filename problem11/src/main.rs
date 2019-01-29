use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
    let grid = get_populated_grid();

    let mut highest = 0;
    let mut highest_point = (0, 0);

    for x in 0..=297 {
        for y in 0..=297 {
            let window_value = get_window_value((x, y), &grid, 3);
            if window_value > highest {
                highest = window_value;
                highest_point = (x, y);
            }
        }
    }

    dbg!(highest);
    dbg!(highest_point);
}

// need to memoize, I think
// maybe store last results in a HashMap based on upper corner, add just the new items on the border
// clear the hashmap between, if you want
fn part2() {
    let grid = get_populated_grid();
    let mut memo: HashMap<(i32, i32, i32), i32> = HashMap::new(); // (x, y, window size) => value

    let mut highest = 0;
    let mut highest_point = (0, 0, 1); // x, y, window_size

    for window_size in 1..=300 {
        for x in 0..=(300 - window_size) {
            for y in 0..=(300 - window_size) {
                let mut window_value: i32;

                if window_size == 1 {
                    window_value = *grid.get(&(x, y)).unwrap();
                } else {
                    window_value = *memo.get(&(x, y, window_size - 1)).unwrap();

                    for add_x in x..x+window_size {
                        window_value += grid.get(&(add_x, y + window_size - 1)).unwrap();
                    }

                    for add_y in y..y+window_size-1 { // -1 here so we don't double count the corner
                        window_value += grid.get(&(x + window_size - 1, add_y)).unwrap();
                    }
                }

                memo.insert((x, y, window_size), window_value);

                if window_value > highest {
                    highest = window_value;
                    highest_point = (x, y, window_size);
                }
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

fn get_window_value(point: (i32, i32), grid: &HashMap<(i32, i32), i32>, window_size: i32)-> i32 {
    let point_x = point.0;
    let point_y = point.1;

    let mut sum = 0;

    for x in point_x..point_x + window_size {
        for y in point_y..point_y + window_size {
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

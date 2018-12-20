use std::fs;
use std::io::Result;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/data.txt")?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(_input: &str) -> Result<()> {
    Ok(())
}

struct Area {
    id: u32,
    left_edge: u32,
    top_edge: u32,
    width: u32,
    height: u32,
}

// #9 @ 810,143: 27x20
// #10 @ 674,274: 25x13
impl Area {
    fn new(specification: &str) -> Area {
        
    }
}

fn find_total_dimensions(input: &str) -> Result<()> {
    let mut max_right = 0;
    let mut max_bottom = 0;

    for line in input.lines() {
        let parsed = Area::new(&line);
        if parsed.right_edge > max_right {
            max_right = parsed.right_edge;
        }

        if parsed.bottom_edge > max_bottom {
            max_bottom = parsed.bottom_edge;
        }
    }
    Ok(())
}

fn part2(_input: &str) -> Result<()> {
    Ok(())
}


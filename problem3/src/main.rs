use std::fs;
use std::io::Result;
use regex::Regex;
#[macro_use] extern crate lazy_static;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/data.txt")?;
    find_total_dimensions(&input)?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

#[derive(Debug)]
struct ClaimArea {
    id: usize,
    left_edge: usize,
    top_edge: usize,
    width: usize,
    height: usize,
}

impl ClaimArea {
    fn new(specification: &str) -> ClaimArea {
        lazy_static! {
            // #9 @ 810,143: 27x20
            // #10 @ 674,274: 25x13
            static ref PARSING_EXPR: Regex = Regex::new(r"^#(?P<id>\d+) @ (?P<left>\d+),(?P<top>\d+): (?P<width>\d+)x(?P<height>\d+)$").unwrap();
        }

        let caps = PARSING_EXPR.captures(specification).unwrap();

        ClaimArea {
            id: caps["id"].parse::<usize>().unwrap(),
            left_edge: caps["left"].parse::<usize>().unwrap(),
            top_edge: caps["top"].parse::<usize>().unwrap(),
            width: caps["width"].parse::<usize>().unwrap(),
            height: caps["height"].parse::<usize>().unwrap(),
        }
    }

    fn bottom_edge(&self) -> usize {
        self.top_edge + self.height
    }

    fn right_edge(&self) -> usize {
        self.left_edge + self.width
    }
}

#[derive(Clone, Debug)]
enum SquareInchStatus {
    Unclaimed,
    ClaimedOnce,
    ClaimedMultiple,
}

fn find_total_dimensions(input: &str) -> Result<()> {
    let mut max_right = 0;
    let mut max_bottom = 0;

    for line in input.lines() {
        let parsed = ClaimArea::new(&line);
        if parsed.right_edge() > max_right {
            max_right = parsed.right_edge();
        }

        if parsed.bottom_edge() > max_bottom {
            max_bottom = parsed.bottom_edge();
        }
    }

    println!("max right = {}, max bottom = {}", max_right, max_bottom);
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let row_len = 999 + 1;
    let rows = 1000 + 1;

    let mut fabric = vec![vec![SquareInchStatus::Unclaimed; row_len]; rows];
    let mut overlapping_total = 0;

    for line in input.lines() {
        let parsed = ClaimArea::new(&line);

        println!("{:?}", parsed);

        for row_num in parsed.top_edge..parsed.bottom_edge() {
            for col_num in parsed.left_edge..parsed.right_edge() {
                match fabric[row_num][col_num] {
                    SquareInchStatus::Unclaimed => fabric[row_num][col_num] = SquareInchStatus::ClaimedOnce,
                    SquareInchStatus::ClaimedOnce => {
                        fabric[row_num][col_num] = SquareInchStatus::ClaimedMultiple;
                        overlapping_total = overlapping_total + 1;
                    }
                    SquareInchStatus::ClaimedMultiple => {}
                }
            }
        }
    }

    println!("Total overlap = {}", overlapping_total);
    
    Ok(())
}

fn part2(_input: &str) -> Result<()> {
    Ok(())
}


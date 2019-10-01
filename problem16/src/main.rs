use lazy_static::*;
use regex::Regex;
use std::fs;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;

    let mut previous_line_was_blank = false;
    let mut current_example = Example::new();

    for line in input.lines() {
        if line.is_empty() && previous_line_was_blank {
            break;
        }
        if line.is_empty() {
            previous_line_was_blank = true;

            dbg!(&current_example);

            current_example = Example::new();
            continue;
        }

        previous_line_was_blank = false;
        match line.chars().nth(0).unwrap() {
            'B' => current_example.before = line,
            '0'..='9' => current_example.instruction = line,
            'A' => current_example.after = line,
            _ => panic!("Shit"),
        }
    }

    Ok(())
}

#[derive(Default, Debug)]
struct Example<'a> {
    before: &'a str,
    instruction: &'a str,
    after: &'a str,
}

struct Instruction {
    opcode: u32,
    input_a: u32,
    input_b: u32,
    output: u32,
}

impl Instruction {
    fn new(values: Vec<u32>) -> Self {
        Instruction {
            opcode: *values.get(0).unwrap(),
            input_a: *values.get(1).unwrap(),
            input_b: *values.get(2).unwrap(),
            output: *values.get(3).unwrap(),
        }
    }
}

impl<'a> Example<'a> {
    fn new() -> Self {
        Example {
            ..Default::default()
        }
    }

    fn parse_before(&self) -> Vec<u32> {
        Example::parse(self.before)
    }

    fn parse_instruction(&self) -> Instruction {
        Instruction::new(
            self.instruction
                .split(' ')
                .map(|item| item.parse::<u32>().expect("Failed to parse as u32"))
                .collect(),
        )
    }

    fn parse_after(&self) -> Vec<u32> {
        Example::parse(self.after)
    }

    fn parse(value: &str) -> Vec<u32> {
        lazy_static! {
            static ref PARSING_EXPR: Regex =
                Regex::new(r"^(Before|After): \[(?P<content>\d+)\]$").unwrap();
        }

        let captures = PARSING_EXPR
            .captures(value)
            .unwrap_or_else(|| panic!(format!("Invalid line \"{}\"", value)));

        captures["content"]
            .split(", ")
            .map(|item| item.parse::<u32>().expect("Failed to parse as u32"))
            .collect()
    }
}

use std::fs;
use std::io::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let input = fs::read_to_string("input/data.txt")?;
    part1(&input)?;
    part2(&input)?;
    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut current_line_map: HashMap<char, i32> = HashMap::new();

    let mut has_double_letter_count = 0;
    let mut has_triple_letter_count = 0;

    for line in input.lines() {
        current_line_map.clear();

        for item in line.chars() {
            if let Some(val) = current_line_map.get_mut(&item) {
                *val = *val + 1;
            } else {
                current_line_map.insert(item, 1);
            }
        }

        if current_line_map.values().any(|i| *i == 2) {
            has_double_letter_count = has_double_letter_count + 1;
        }

        if current_line_map.values().any(|i| *i == 3) {
            has_triple_letter_count = has_triple_letter_count + 1;
        }
    }

    println!("Doubles * Triples = {}", has_double_letter_count * has_triple_letter_count);
    Ok(())
}

fn part2(input: &str) -> Result<()> {
    'outer: for (index, current_line) in input.lines().enumerate() {
        for comparison_line in input.lines().skip(index + 1) {
            if hamming_distance(current_line, comparison_line) == 1 {
                println!("{} && {} = {}", current_line, comparison_line, common_chars(current_line, comparison_line));
                break 'outer;
            }
        }
    }
    Ok(())
}

fn hamming_distance(s1: &str, s2: &str) -> i32 {
    let mut distance = 0;
    for (s1_char, s2_char) in s1.chars().zip(s2.chars()) {
        if s1_char != s2_char {
            distance = distance + 1;
        }
    }

    distance
}

fn common_chars(s1: &str, s2: &str) -> String {
    let mut common = "".to_string();
    let mut s2_chars = s2.chars();

    for (index, current) in s1.chars().enumerate() {
        if s2.len() > index && current == s2_chars.nth(0).unwrap() {
            common.push(current);
        }
    }

    common
}

#[cfg(test)]
mod hamming_distance_tests {
    use super::hamming_distance;

    #[test]
    fn empty() {
        assert_eq!(hamming_distance("", ""), 0)
    }

    #[test]
    fn equal() {
        assert_eq!(hamming_distance("aaa", "aaa"), 0)
    }

    #[test]
    fn not_equal() {
        assert_eq!(hamming_distance("aaa", "aba"), 1)
    }
}

#[cfg(test)]
mod common_chars_tests {
    use super::common_chars;

    #[test]
    fn empty() {
        assert_eq!(common_chars("", ""), "")
    }

    #[test]
    fn equal() {
        assert_eq!(common_chars("aaa", "aaa"), "aaa")
    }

    #[test]
    fn not_equal() {
        assert_eq!(common_chars("aaa", "aba"), "aa")
    }

    #[test]
    fn more_letters() {
        assert_eq!(common_chars("abcdefghijk", "abzdezghijk"), "abdeghijk")
    }

    #[test]
    fn first_shorter() {
        assert_eq!(common_chars("aa", "aaa"), "aa")
    }

    #[test]
    fn second_shorter() {
        assert_eq!(common_chars("aaa", "aa"), "aa")
    }
}

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

/*
The boxes will have IDs which differ by exactly one character at the same position in both strings. For example, given the following box IDs:

abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz

The IDs abcde and axcye are close, but they differ by two characters (the second and fourth).

However, the IDs fghij and fguij differ by exactly one character, the third (h and u). Those must be the correct boxes.

What letters are common between the two correct box IDs? (In the example above, this is found by removing the differing character from either ID, producing fgij.)
*/
fn part2(_input: &str) -> Result<()> {
    println!("Not implemented");
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

#[cfg(test)]
mod tests {
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

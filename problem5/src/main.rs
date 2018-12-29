use std::fs;
use std::result::Result;

fn main() -> Result<(), String> {
    let input = fs::read_to_string("input/data.txt").map_err(|e| e.to_string())?;

    part1(&input).map_err(|e| e.to_string())?;
    part2(&input).map_err(|e| e.to_string())?;
    Ok(())
}

fn part1(input: &str) -> Result<(), String> {
    let result = perform_reactions(input);
    println!("Result = {}\nLength = {}", result, result.len());
    Ok(())
}

fn part2(input: &str) -> Result<(), String> {
    static ASCII_LOWER: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 
        'f', 'g', 'h', 'i', 'j', 
        'k', 'l', 'm', 'n', 'o',
        'p', 'q', 'r', 's', 't', 
        'u', 'v', 'w', 'x', 'y', 
        'z',
    ];

    let mut shortest_len = input.len();

    for to_remove in ASCII_LOWER.iter() {
        let filtered = input.clone().chars().filter(|ch| !ch.eq_ignore_ascii_case(to_remove)).collect::<String>();
        let reacted_len = perform_reactions(&filtered).len();
        if reacted_len < shortest_len {
            shortest_len = reacted_len;
        }
    }

    println!("{}", shortest_len);

    Ok(())
}

fn perform_reactions(input: &str) -> String {
    let mut new_str = input.to_string();
    loop {
        let start_size = new_str.len();
        new_str = perform_single_reaction_pass(&new_str);
        if new_str.len() == start_size {
            break;
        }
    }
    
    new_str
}

fn perform_single_reaction_pass(input: &str) -> String {
    let mut new_str = "".to_string();

    let mut iter = input.chars().peekable();

    while let Some(ch) = iter.next() {
        match iter.peek() {
            None => {
                new_str.push(ch);
                break;
            },
            Some(next_char) => {
                if ch.eq_ignore_ascii_case(next_char) && casings_are_different(&ch, &next_char) {
                    iter.next();
                } else {
                    new_str.push(ch);
                }
            }
        }
    }

    new_str
}

fn casings_are_different(ch1: &char, ch2: &char) -> bool {
    ch1.is_lowercase() && !ch2.is_lowercase() || !ch1.is_lowercase() && ch2.is_lowercase()
}

#[cfg(test)]
mod test_perform_reactions {
    use super::{perform_single_reaction_pass, perform_reactions};

    #[test]
    fn no_reactions_single_letter() {
        assert_eq!(perform_single_reaction_pass("a"), "a");
        assert_eq!(perform_single_reaction_pass("A"), "A");

        assert_eq!(perform_reactions("a"), "a");
        assert_eq!(perform_reactions("A"), "A");
    }

    #[test]
    fn no_reactions_different_letters() {
        assert_eq!(perform_reactions("abcABC"), "abcABC");
        assert_eq!(perform_reactions("aabbCCDD"), "aabbCCDD");
    }

    #[test]
    fn single_reacting_pair() {
        assert_eq!(perform_reactions("Aa"), "");
        assert_eq!(perform_reactions("aA"), "");
    }

    #[test]
    fn two_reacting_pairs() {
        assert_eq!(perform_reactions("AabB"), "");
        assert_eq!(perform_reactions("AaCbB"), "C");
    }

    #[test]
    fn pair_after_removal() {
        assert_eq!(perform_reactions("ACca"), "");
    }

    #[test]
    fn pair_after_multi_removal() {
        assert_eq!(perform_reactions("ZbACcaBz"), "");
    }
}

use std::collections::HashMap;
use crate::circle::Circle;

mod circle;

fn main() {
    let part1_res = part1(473, 70904);
    assert_eq!(371284, part1_res);
    println!("Part 1 = {}", part1_res);
    let part2_res = part1(473, 70904 * 100);
    println!("Part 2 = {}", part2_res);
}

fn part1(player_count: usize, last_marble_value: usize) -> usize {
    let mut player_scores: HashMap<usize, usize> = HashMap::new();
    let mut placed_marbles = Circle::new(last_marble_value);
    placed_marbles.insert(0);

    for marble_number in 1..=last_marble_value {
        let current_player = marble_number % player_count;

        if marble_number % 23 == 0 {
            let marble_score = marble_number + placed_marbles.remove();

            let current_player_score = player_scores.entry(current_player).or_insert(0);
            *current_player_score += marble_score;
        } else {
            placed_marbles.insert(marble_number);
        }
    }

    *player_scores
        .iter()
        .fold(None, |max, curr| match max {
            None => Some(curr.1),
            Some(existing) => Some(if curr.1 > existing { curr.1 } else { existing }),
        })
        .unwrap()
}

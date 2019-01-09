use std::collections::HashMap;
use std::collections::VecDeque;

fn main() {
    let part1_res = part1(473, 70904);
    assert_eq!(371284, part1_res);
    println!("Part 1 = {}", part1_res);
    // let part2_res = part1(473, 70904 * 100);
    // println!("Part 2 = {}", part2_res);
}

fn part1(player_count: usize, last_marble_value: usize) -> usize {
    let mut player_scores: HashMap<usize, usize> = HashMap::new();
    let mut placed_marbles: VecDeque<usize> = VecDeque::new();
    placed_marbles.push_front(0);

    let mut current_marble_location = 0;

    for marble_number in 1..=last_marble_value {
        let current_player = marble_number % player_count;

        if marble_number % 23 == 0 {
            let raw_position_to_remove: i32 = current_marble_location as i32 - 7;

            let position_to_remove: usize = if raw_position_to_remove.is_negative() {
                (placed_marbles.len() as i32 + raw_position_to_remove) as usize
            } else {
                raw_position_to_remove as usize
            };

            let marble_score = marble_number + placed_marbles.remove(position_to_remove).unwrap();
            current_marble_location = position_to_remove;

            let current_player_score = player_scores.entry(current_player).or_insert(0);
            *current_player_score += marble_score;
        } else {
            let insertion_position = (current_marble_location + 2) % placed_marbles.len();
            placed_marbles.insert(insertion_position, marble_number);
            current_marble_location = insertion_position;
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

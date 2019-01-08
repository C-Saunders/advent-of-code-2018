use std::collections::HashMap;

fn main() {
    part1(473, 70904);
}

fn part1(player_count: usize, last_marble_value: usize) {
    let mut player_scores: HashMap<usize, usize> = HashMap::new();
    let mut placed_marbles: Vec<usize> = vec![0];
    let mut current_marble_location = 0;
    let mut marble_number = 1;

    loop {
        let current_player = marble_number % player_count;

        if marble_number % 23 == 0 {
            let raw_position_to_remove: i32 = (current_marble_location as i32 - 7);
            let position_to_remove: usize = if raw_position_to_remove //negative {
                placed_marbles.len() - raw_position_to_remove
            } else {
                raw_position_to_remove
            }

            let marble_score = marble_number + placed_marbles.remove(position_to_remove);
            current_marble_location = position_to_remove;

            let current_player_score = player_scores.entry(current_player).or_insert(0);
            *current_player_score += marble_score;

            if marble_score == last_marble_value {
                break;
            }
        } else {
            let insertion_position = (current_marble_location + 2) % placed_marbles.len();
            placed_marbles.insert(insertion_position, marble_number);
            current_marble_location = insertion_position;
        }

        println!("{:?}", placed_marbles);

        marble_number += 1;
    }
}

use crate::common::str_to_numbers;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct PlantSet {
    data: VecDeque<u8>,
    leftmost_index: i32,
}

impl PlantSet {
    pub fn from_string(initial_state: &str) -> Self {
        PlantSet {
            data: VecDeque::from(str_to_numbers(initial_state)),
            leftmost_index: 0,
        }
    }

    pub fn get_next_generation(&self, rule_set: &HashMap<Vec<u8>, u8>) -> PlantSet {
        let padded_copy = dbg!(self.get_padded_copy());
        let data = padded_copy.data;
        let window_size = 5;

        let mut output: VecDeque<u8> = VecDeque::from(vec![data[0], data[1]]);

        for start_index in 0..data.len() - (window_size - 1) {
            let window: Vec<u8> = data
                .clone()
                .into_iter()
                .skip(start_index)
                .take(window_size)
                .collect();

            let result = rule_set.get(&window).expect("Unknown pattern");

            output.push_back(*result);
        }

        dbg!(PlantSet {
            data: output,
            leftmost_index: padded_copy.leftmost_index,
        }).get_trimmed_copy()
    }

    fn get_padded_copy(&self) -> PlantSet {
        let padding: VecDeque<u8> = vec![0; 4].into_iter().collect();

        let mut new_state = padding.clone();
        new_state.extend(&self.data.clone());
        new_state.append(&mut padding.clone());

        PlantSet {
            data: new_state,
            leftmost_index: self.leftmost_index - 4,
        }
    }

    fn get_trimmed_copy(&self) -> PlantSet {
        let mut new_data = self.data.clone();

        // right
        loop {
            let popped = new_data.pop_back().expect("Failed to pop back");
            if popped == 1 {
                new_data.push_back(popped);
                break;
            }
        }

        // left
        let mut left_pop_counter = 0;
        loop {
            let popped = new_data.pop_front().expect("Failed to pop front");
            if popped == 1 {
                new_data.push_front(popped);
                break;
            }
            left_pop_counter += 1;
        }

        PlantSet {
            data: new_data,
            leftmost_index: self.leftmost_index + left_pop_counter,
        }
    }
}

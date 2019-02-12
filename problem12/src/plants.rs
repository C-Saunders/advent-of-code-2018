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

    pub fn get_sum_of_indexes(&self) -> i32 {
        self.data.iter().enumerate().fold(0, |acc, (index, value)| {
            acc + ((index as i32 + self.leftmost_index) * (*value as i32))
        })
    }

    pub fn get_next_generation(&mut self, rule_set: &HashMap<Vec<u8>, u8>) {
        self.pad_data();
        let data = &self.data;
        let window_size = 5;

        let mut output: VecDeque<u8> = VecDeque::from(vec![data[0], data[1]]);

        for start_index in 0..data.len() - (window_size - 1) {
            let window: Vec<u8> = data
                .iter()
                .cloned()
                .skip(start_index)
                .take(window_size)
                .collect();

            let result = rule_set.get(&window).expect("Unknown pattern");

            output.push_back(*result);
        }

        self.data = output;
        self.trim_data();
    }

    fn pad_data(&mut self) {
        for _ in 0..4 {
            self.data.push_back(0);
            self.data.push_front(0);
        }

        self.leftmost_index -= 4;
    }

    fn trim_data(&mut self) {
        // right
        loop {
            let popped = self.data.pop_back().expect("Failed to pop back");
            if popped == 1 {
                self.data.push_back(popped);
                break;
            }
        }

        // left
        let mut left_pop_counter = 0;
        loop {
            let popped = self.data.pop_front().expect("Failed to pop front");
            if popped == 1 {
                self.data.push_front(popped);
                break;
            }
            left_pop_counter += 1;
        }

        self.leftmost_index += left_pop_counter;
    }
}

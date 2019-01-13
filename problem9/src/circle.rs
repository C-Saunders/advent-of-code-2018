use std::collections::VecDeque;

#[derive(Debug)]
pub struct Circle {
    data: VecDeque<usize>,
}

impl Circle {
    pub fn new(capacity: usize) -> Self {
        Circle {
            data: VecDeque::with_capacity(capacity),
        }
    }

    fn rotate_left(&mut self, num_places: usize) {
        for _ in 0..num_places {
            if let Some(item) = self.data.pop_front() {
                self.data.push_back(item);
            }
        }
    }

    fn rotate_right(&mut self, num_places: usize) {
        for _ in 0..num_places {
            if let Some(item) = self.data.pop_back() {
                self.data.push_front(item);
            }
        }
    }

    // we only care about inserting two to the right of the current node
    pub fn insert(&mut self, data: usize) {
        self.rotate_left(2);
        self.data.push_front(data);
    }

    // we only care about removing the item seven to the left of current
    pub fn remove(&mut self) -> usize {
        self.rotate_right(7);
        self.data.pop_front().unwrap()
    }
}

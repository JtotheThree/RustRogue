use std::collections::VecDeque;
use std::time::{Duration, Instant};

pub struct Counter {
    last_second_frames: VecDeque<Instant>
}

impl Counter {
    pub fn new() -> Counter {
        Counter {
            last_second_frames: VecDeque::with_capacity(128)
        }
    }

    pub fn tick(&mut self) -> usize {
        let now = Instant::now();
        let a_second_ago = now - Duration::from_secs(1);

        while self.last_second_frames.front().map_or(false, |t| *t < a_second_ago) {
            self.last_second_frames.pop_front();
        }

        self.last_second_frames.push_back(now);
        self.last_second_frames.len()
    }
}
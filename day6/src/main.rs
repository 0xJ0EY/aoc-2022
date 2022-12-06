use std::collections::{HashMap, VecDeque};

static PART1_MARKER_SIZE: usize = 4;
static PART2_MARKER_SIZE: usize = 14;

struct StreamTuner {
    entries: HashMap::<char, usize>,
    latest: VecDeque::<char>,
    index: usize,
    marker_size: usize
}

impl StreamTuner {
    fn new(marker_size: usize) -> StreamTuner {
        StreamTuner {
            entries: HashMap::new(),
            latest: VecDeque::new(),
            index: 0,
            marker_size
        }
    }

    fn add(&mut self, input: char) {
        self.latest.push_back(input);
        self.index += 1;

        *self.entries.entry(input).or_insert(0) += 1;

        if self.latest.len() > self.marker_size {
            let popped = self.latest.pop_front().unwrap();
            
            *self.entries.get_mut(&popped).unwrap() -= 1
        }
    }

    fn is_unique(&self) -> bool {
        if self.latest.len() < self.marker_size { return false; }

        for c in self.latest.iter() {
            let result = match self.entries.get(c) {
                Some(v) => *v <= 1,
                None => false,
            };

            if !result { return false }
        }

        true
    }

    fn index(&self) -> usize {
        self.index
    }

}

fn part1(input: &str) -> usize {
    let mut tuner = StreamTuner::new(PART1_MARKER_SIZE);

    for c in input.chars().into_iter() {
        tuner.add(c);

        if tuner.is_unique() { return tuner.index(); }
    }
    
    0
}

fn part2(input: &str) -> usize {
    let mut tuner = StreamTuner::new(PART2_MARKER_SIZE);

    for c in input.chars().into_iter() {
        tuner.add(c);

        if tuner.is_unique() { return tuner.index(); }
    }
    
    0
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part1: {}", part2(input));
}

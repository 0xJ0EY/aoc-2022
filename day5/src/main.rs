use std::{str::FromStr, collections::VecDeque};

#[derive(Debug)]
struct CrateContainer {
    pub crates: Vec<Vec<char>>,
}

impl FromStr for CrateContainer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('\n').collect();
        let mut crates: Vec<Vec<char>> = Vec::new(); 

        parts.iter().rev().skip(1).for_each(|&line| {
            let line_values: Vec<char> = line.chars().into_iter()
                .skip(1)
                .step_by(4)
                .collect();

            line_values.iter().enumerate().for_each(|(index, value)| {
                if *value == ' ' { return; }

                match crates.get_mut(index) {
                    Some(row) => row.push(*value),
                    None => {
                        let mut row = Vec::new();
                        row.push(*value);

                        crates.insert(index, row);
                    },
                }
            });
        });

        Ok(CrateContainer { crates })
    }
}

impl CrateContainer {
    fn process_single_move(&mut self, input: &str) {
        let input_parts: Vec<&str> = input.split(' ').collect();
        let amount = input_parts[1].parse::<usize>().unwrap();
        let start = input_parts[3].parse::<usize>().unwrap() - 1;
        let end = input_parts[5].parse::<usize>().unwrap() - 1;

        for _ in 0..amount {
            let value = self.crates.get_mut(start).unwrap().pop().unwrap();
            self.crates.get_mut(end).unwrap().push(value);
        }
    }

    fn process_multiple_move(&mut self, input: &str) {
        let input_parts: Vec<&str> = input.split(' ').collect();
        let amount = input_parts[1].parse::<usize>().unwrap();
        let start = input_parts[3].parse::<usize>().unwrap() - 1;
        let end = input_parts[5].parse::<usize>().unwrap() - 1;

        let mut temporary: VecDeque<char> = VecDeque::new();

        for _ in 0..amount {
            let value = self.crates.get_mut(start).unwrap().pop().unwrap();
            temporary.push_front(value);
        }

        for _ in 0..amount {
            let value = temporary.pop_front().unwrap();
            self.crates.get_mut(end).unwrap().push(value);
        }
    }

    fn get_top_level_crates(&self) -> String {
        let mut output = String::with_capacity(self.crates.len());

        self.crates.iter().for_each(|row| {
            output.push(*row.last().unwrap());
        });

        output
    }

}

fn part1(input: &str) -> String {
    let (crates, moves) = input.split_once("\n\n").unwrap();
    let mut container = CrateContainer::from_str(crates).unwrap();

    moves.split('\n').into_iter().for_each(|input| container.process_single_move(input));

    container.get_top_level_crates()
}

fn part2(input: &str) -> String {
    let (crates, moves) = input.split_once("\n\n").unwrap();
    let mut container = CrateContainer::from_str(crates).unwrap();

    moves.split('\n').into_iter().for_each(|input| container.process_multiple_move(input));

    container.get_top_level_crates()
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

use std::{str::FromStr, num::ParseIntError, cmp};

struct Section {
    pub start: usize,
    pub end: usize
}

impl FromStr for Section {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').unwrap();

        let start_fromstr   = start.parse::<usize>()?;
        let end_fromstr     = end.parse::<usize>()?;

        Ok(Section { start: start_fromstr, end: end_fromstr })
    }
}

impl Section {
    fn contains(&self, other: &Section) -> bool {
        let start   = self.start <= other.start && self.end >= other.end;
        let end     = self.start >= other.start && self.end <= other.end;

        start || end
    }

    fn overlap(&self, other: &Section) -> bool {
        let start   = cmp::max(self.start, other.start) as i64;
        let end     = cmp::min(self.end, other.end) as i64;

        end - start >= 0
    }
}

fn part1(input: &str) -> usize {
    return input
        .split_terminator('\n')
        .filter(|line| {
            let (start, end) = line.split_once(',').unwrap();

            let lhs = Section::from_str(start).unwrap();
            let rhs = Section::from_str(end).unwrap();

            lhs.contains(&rhs)
        })
        .count()
}

fn part2(input: &str) -> usize {
    return input.split_terminator('\n')
        .filter(|line| {
            let (start, end) = line.split_once(',').unwrap();

            let lhs = Section::from_str(start).unwrap();
            let rhs = Section::from_str(end).unwrap();

            lhs.overlap(&rhs)
        })
        .count();
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

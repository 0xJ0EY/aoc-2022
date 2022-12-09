use std::{str::FromStr, collections::HashSet};

#[derive(Debug, Clone, Copy)]
enum Motion {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32)
}

impl FromStr for Motion {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (identifier, value) = s.split_once(" ").unwrap();

        let value = value.parse::<i32>().unwrap();
        
        Ok(match identifier {
            "U" => Motion::Up(value),
            "D" => Motion::Down(value),
            "L" => Motion::Left(value),
            "R" => Motion::Right(value),
            _ => unreachable!()
        })
    }
}

impl Motion {
    fn parse(s: &str) -> Vec<Motion> {
        s.split_terminator('\n')
            .map(|line| Motion::from_str(line).unwrap())
            .collect()
    }

    fn consume(&self) -> Option<Motion> {
        match self {
            Motion::Up(x)       => if *x > 1 { Some(Motion::Up(*x - 1)) } else { None },
            Motion::Down(x)     => if *x > 1 { Some(Motion::Down(*x - 1)) } else { None },
            Motion::Left(x)     => if *x > 1 { Some(Motion::Left(*x - 1)) } else { None },
            Motion::Right(x)    => if *x > 1 { Some(Motion::Right(*x - 1)) } else { None },
        }
    }
}

struct Bridge {
    parts: Vec<(i32, i32)>,
    visited_locations: HashSet<(i32, i32)>,
}

impl Bridge {
    fn new(length: usize) -> Bridge {
        let mut visited_locations = HashSet::new();
        visited_locations.insert((0, 0));

        Bridge {
            parts: (0..length).into_iter().map(|_| (0, 0)).collect(),
            visited_locations,
        }
    }

    fn apply_motion(&mut self, motion: &Motion) {
        let mut action = Some(motion.clone());

        while let Some(direction) = action {
            let (hx, hy) = self.parts.first_mut().unwrap();

            match direction {
                Motion::Up(_)       => *hy += 1,
                Motion::Down(_)     => *hy -= 1,
                Motion::Left(_)     => *hx -= 1,
                Motion::Right(_)    => *hx += 1,
            }

            for index in 1..self.parts.len() {
                let (hx, hy) = self.parts.get(index - 1).unwrap();
                let (tx, ty) = self.parts.get(index).unwrap();

                let (dx, dy) = ((*hx - *tx), (*hy - *ty));
                
                if dx.abs() > 1 || dy.abs() > 1 {
                    let rotation = (dy as f32).atan2(dx as f32).to_degrees();
                    let step = ((rotation / 45.0).round() * 45.0) as i32;

                    let (x, y) = match step {
                        0 => (1, 0),
                        45 => (1, 1),
                        90 => (0, 1),
                        135 => (-1, 1),
                        -45 => (1, -1),
                        -90 => (0, -1),
                        -135 => (-1, -1),
                        180 | -180 => (-1, 0),
                        _   => unreachable!()
                    };

                    let new_pos = (*tx + x, *ty + y);

                    if index == self.parts.len() - 1 { self.visited_locations.insert(new_pos); }

                    let old_pos = self.parts.get_mut(index).expect("cannot get mutable of snake part");
                    old_pos.0 = new_pos.0;
                    old_pos.1 = new_pos.1;
                }
            }

            action = direction.consume();
        }
    }

    fn apply_motions(&mut self, motion: &Vec<Motion>) {
        motion.iter().for_each(|x| self.apply_motion(x))
    }
}

fn part1(motions: &Vec<Motion>) -> usize {
    let mut bridge = Bridge::new(2);

    bridge.apply_motions(&motions);

    bridge.visited_locations.len()
}

fn part2(motions: &Vec<Motion>) -> usize {
    let mut bridge = Bridge::new(10);

    bridge.apply_motions(&motions);

    bridge.visited_locations.len()
}

fn main() {    
    let input = include_str!("input.txt");

    let motions = Motion::parse(input);

    println!("part1: {}", part1(&motions));
    println!("part2: {}", part2(&motions));
}

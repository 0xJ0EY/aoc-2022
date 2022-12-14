use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    walls: HashSet<(i32, i32)>,
    idle_sand: HashSet<(i32, i32)>,
    highest_index: i32,
}

impl Grid {
    fn parse(s: &str) -> Grid {
        let mut walls = HashSet::<(i32, i32)>::new();
        let idle_sand = HashSet::<(i32, i32)>::new();

        let lines = s.split_terminator('\n').map(|x| {
            x.split(" -> ").map(|y| {
                let (x, y) = y.split_once(',').unwrap();

                (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
            }).collect::<Vec<_>>()
        }).collect::<Vec<_>>();

        for line in lines {
            for coords in line.windows(2) {
                let ((x, y), (nx, ny)) = (coords[0], coords[1]);

                walls.insert((x, y));

                let (mut dx, mut dy) = (x - nx, y - ny);

                while dx != 0 {
                    walls.insert((x - dx, y));

                    dx += -dx.signum()
                }
                
                while dy != 0 {
                    walls.insert((x, y - dy));
                    dy += -dy.signum()
                }
            }
        }

        let highest_index = walls.iter().map(|(_, y)| y).max().unwrap() + 2;
        Grid { walls, idle_sand, highest_index }
    }

    fn check_collision(&self, x: i32, y: i32) -> bool {
        self.walls.contains(&(x, y)) || self.idle_sand.contains(&(x, y))
    }

    fn check_collision_with_floor(&self, x: i32, y: i32) -> bool {
        if y >= self.highest_index { return true }

        self.walls.contains(&(x, y)) || self.idle_sand.contains(&(x, y))
    }

    fn spawn_sand(&mut self, is_blocked: fn(&Grid, i32, i32) -> bool) -> bool {
        let start = (500, 0);
        let (mut x, mut y) = start;

        loop {
            let initial = (x, y);

            for (dx, dy) in [(0, 1), (-1, 1), (1, 1)] {
                if !(is_blocked)(&self, x + dx, y + dy) {
                    x += dx;
                    y += dy;

                    break;
                }
            }

            if start == (x, y) {
                return false;
            }

            if y > self.highest_index {
                return false;
            }

            if initial == (x, y) {
                self.idle_sand.insert((x, y));
                return true;
            }
        }
    }

}

fn part1(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    let mut count = 0;

    while grid.spawn_sand(Grid::check_collision) {
        count += 1;
    }

    count
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::parse(input);

    // Start at 1, since the last iteration of spawn_sand will also place a piece of sand
    let mut count = 1;

    while grid.spawn_sand(Grid::check_collision_with_floor) {
        count += 1;
    }

    count
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

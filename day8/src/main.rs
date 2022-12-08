use std::{str::FromStr, cmp::max};

struct Grid {
    width: usize,
    height: usize,
    trees: Vec<u32>
}

impl Grid {
    pub fn to_coordinate(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.height)
    }

    pub fn entry(&self, x: i32, y: i32) -> Option<&u32> {
        if x < 0 || y < 0 { return None; }
        if x >= (self.width as i32) || y >= (self.height as i32) { return None; }

        let index = x + (y * self.width as i32);

        self.trees.get(index as usize)
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.split_terminator('\n').collect();
        let width   = lines.first().unwrap().len();
        let height  = lines.len();

        let trees = lines.iter().fold(Vec::new(), |mut acc: Vec<u32>, line| {
            line.chars().into_iter().map(|x| x as u32 - '0' as u32).for_each(|x| acc.push(x));
            acc
        });

        Ok(Grid { width, height, trees })
    }
}
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

fn solve(grid: &Grid) -> (usize, usize) {
    grid.trees.iter().enumerate().map(|(index, tree_height)| {
        let mut scenic_score = 1;
        let mut any_visible = false;

        let (x, y) = grid.to_coordinate(index);

        DIRECTIONS.into_iter().for_each(|(dx, dy)| {
            let (mut tx, mut ty) = (x as i32 + dx, y as i32 + dy);

            let mut visible = true;
            let mut view_distance = 0;

            while let Some(entry) = grid.entry(tx, ty) {
                view_distance += 1;

                if entry >= tree_height {
                    visible = false;
                    break;
                }

                (tx, ty) = ((tx + dx), (ty + dy))
            }

            if visible == true {
                any_visible = true;
            }

            scenic_score *= view_distance;
        });

        (any_visible, scenic_score)
    }).fold((0, 0), |(mut visible, mut max_scenic_score), (is_visible, scenic_score)| {
        if is_visible { visible += 1 }

        max_scenic_score = max(max_scenic_score, scenic_score);

        (visible, max_scenic_score)
    })
}

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::from_str(input).unwrap();

    let (part1, part2) = solve(&grid);

    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

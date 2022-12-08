use std::str::FromStr;

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

fn part1(grid: &Grid) -> usize {
    let result = grid.trees.iter().enumerate().filter(|(index, tree_height)| {
        let (x, y) = grid.to_coordinate(*index);
        if x == 0 || y == 0 { return true; }

        DIRECTIONS.into_iter().any(|(dx, dy)| {
            let (mut tx, mut ty) = (x as i32 + dx, y as i32 + dy);

            while let Some(entry) = grid.entry(tx, ty) {    
                if entry >= *tree_height {
                    return false
                }
    
                (tx, ty) = ((tx + dx), (ty + dy))
            }

            true
        })
    }).count();

    result
}

fn part2(grid: &Grid) -> usize {
    let result = grid.trees.iter().enumerate().map(|(index, tree_height)| {
        let (x, y) = grid.to_coordinate(index);
        if x == 0 || y == 0 { return 0 }

        let scenic_score = DIRECTIONS.into_iter().map(|(dx, dy)| {
            let (mut tx, mut ty) = (x as i32 + dx, y as i32 + dy);

            let mut view_distance = 0;

            while let Some(entry) = grid.entry(tx, ty) {    
                view_distance += 1;

                if entry >= tree_height {
                    return view_distance
                }
    
                (tx, ty) = ((tx + dx), (ty + dy))
            }

            view_distance
        }).fold(1, |acc, val| acc * val);

        scenic_score
    }).max();

    result.unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::from_str(input).unwrap();

    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

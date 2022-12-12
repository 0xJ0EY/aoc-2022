use std::collections::{HashSet, VecDeque};

#[derive(Debug)]
struct Grid {
    start: (usize, usize),
    end: (usize, usize),
    height: usize,
    width: usize,
    values: Vec<u8>
}

#[derive(Debug)]
struct Node {
    step: usize,
    x: usize,
    y: usize,
    value: u8,
}

impl Grid {
    fn parse(s: &str) -> Grid {
        let lines = s.split_terminator('\n').collect::<Vec<_>>();
        let width = lines[0].len();
        let height = lines.len();

        let values: Vec<char> = lines.iter()
            .flat_map(|x| x.chars())
            .collect();

        let start = values.iter().position(|x| *x == 'S').expect("No start found in grid");
        let end = values.iter().position(|x| *x == 'E').expect("No end found in grid");

        let start = (start % width, start / width);
        let end = (end % width, end / width);
        
        let values = values.into_iter().map(|x| {
            let mut char = x;

            if char == 'S' { char = 'a' }
            if char == 'E' { char = 'z' }

            char as u8 - b'a'
        }).collect();

        Grid {
            start,
            end,
            height,
            width,
            values,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<Node> {
        if x < 0 || y < 0 { return None }
        let (x, y) = (x as usize, y as usize);
        if x >= self.width as usize || y >= self.height { return None }

        let index = x + y * self.width;
        Some(Node { step: 0, x, y, value: self.values[index] })
    }

    fn get_by_index(&self, index: usize) -> Option<Node> {
        let x = index % self.width;
        let y = index / self.width;

        Some(Node { step: 0, x, y, value: self.values[index] })
    }

    fn start_node(&self) -> Option<Node> {
        let (x, y) = self.start;

        self.get(x as isize, y as isize)
    }

    fn get_adjacent_nodes(&self, node: &Node) -> Vec<Node> {
        let mut nodes = Vec::new();

        let (x, y) = (node.x as isize, node.y as isize);

        if let Some(up)     = self.get(x, y - 1) { nodes.push(up) }
        if let Some(down)   = self.get(x, y + 1) { nodes.push(down) }
        if let Some(left)   = self.get(x - 1, y) { nodes.push(left) }
        if let Some(right)  = self.get(x + 1, y) { nodes.push(right) }

        nodes
    }


    fn draw(&self, visited: &HashSet<(usize, usize)>) -> () {

        for y in 0..self.height as isize {
            for x in 0..self.width as isize {
                let node = self.get(x, y).unwrap();

                if visited.contains(&(node.x, node.y)) {
                    print!("@")    
                } else {
                    print!("{}", (node.value + b'a') as char)
                }   
            }

            println!("")
        }

        std::process::Command::new("clear").status().unwrap();
    } 
    
}

#[derive(Debug)]
struct BFS {
    queue: VecDeque<Node>,
    visited: HashSet<(usize, usize)>
}

impl BFS {
    fn new() -> BFS {
        BFS {
            queue: VecDeque::new(),
            visited: HashSet::new(),
        }
    }

    fn find_end(&mut self, grid: &Grid) -> Option<Node> {
        let root = grid.start_node().unwrap();
        self.queue.push_back(root);

        while let Some(v) = self.queue.pop_front() {
            self.visited.insert((v.x, v.y));

            if (v.x, v.y) == grid.end { return Some(v); }

            let adjacent_nodes: Vec<Node> = grid.get_adjacent_nodes(&v).into_iter().filter(|x| {
                let current = &v;
                let neighbor = x;

                current.value + 1 >= neighbor.value
            }).collect();

            for mut edge in adjacent_nodes {
                if self.visited.contains(&(edge.x, edge.y)) { continue }
                self.visited.insert((edge.x, edge.y));

                edge.step = v.step + 1;

                self.queue.push_back(edge);
            }
        }

        None
    }
}

fn part1(grid: &Grid) -> usize {
    let mut bfs = BFS::new();
    let end_node = bfs.find_end(grid).unwrap();
    end_node.step
}

fn part2(grid: &Grid) -> usize {
    let mut bfs = BFS::new();
    let end_node = bfs.find_end(grid).unwrap();
    end_node.step
}

fn main() {
    let input = include_str!("input.txt");
    let grid = Grid::parse(input);

    println!("part1: {}", part1(&grid));
    println!("part2: {}", part2(&grid));
}

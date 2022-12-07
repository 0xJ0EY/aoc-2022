#[derive(Clone, Copy, Debug)]
enum Input {
    Rock, Paper, Scissors
}

impl Input {
    fn points(self) -> usize {
        match self {
            Input::Rock => 1,
            Input::Paper => 2,
            Input::Scissors => 3,
        }
    }

    fn from_lhs(input: &str) -> Option<Self> {
        match input {
            "A" => Some(Input::Rock),
            "B" => Some(Input::Paper),
            "C" => Some(Input::Scissors),
            _ => None
        }
    }

    fn from_rhs(input: &str) -> Option<Self> {
        match input {
            "X" => Some(Input::Rock),
            "Y" => Some(Input::Paper),
            "Z" => Some(Input::Scissors),
            _ => None
        }
    }

    fn from_rhs_to_desired_end(input: &str, lhs: &Self) -> Option<Self> {
        // X => Lose
        // Y => Draw
        // Z => Win
        match (input, lhs) {
            ("X", Input::Paper)     => Some(Input::Rock),
            ("X", Input::Rock)      => Some(Input::Scissors),
            ("X", Input::Scissors)  => Some(Input::Paper),
            ("Y", _)                => Some(*lhs),
            ("Z", Input::Rock)      => Some(Input::Paper),
            ("Z", Input::Paper)     => Some(Input::Scissors),
            ("Z", Input::Scissors)  => Some(Input::Rock),
            (_, _) => None
        }
    }

    fn calculate_move_points(self, other: &Self) -> usize {
        let move_points = match (self, other) {
            (Input::Rock, Input::Paper)     => 0,
            (Input::Rock, Input::Scissors)  => 6,
            (Input::Paper, Input::Rock)     => 6,
            (Input::Paper, Input::Scissors) => 0,
            (Input::Scissors, Input::Rock)  => 0,
            (Input::Scissors, Input::Paper) => 6,
            _ => 3
        };

        move_points + self.points()
    }

}

fn part1(input: &str) -> usize {
    input.split('\n').map(|line| {
        let parts: Vec<&str> = line.split(' ').collect();

        if parts.first().is_none() || parts.get(1).is_none() {
            return 0
        }

        let lhs = Input::from_lhs(parts.first().unwrap()).unwrap();
        let rhs = Input::from_rhs(parts.get(1).unwrap()).unwrap();

        rhs.calculate_move_points(&lhs)
    }).sum()
}

fn part2(input: &str) -> usize {
    input.split('\n').map(|line| {
        let parts: Vec<&str> = line.split(' ').collect();

        if parts.first().is_none() || parts.get(1).is_none() {
            return 0
        }

        let lhs = Input::from_lhs(parts.first().unwrap()).unwrap();
        let rhs = Input::from_rhs_to_desired_end(parts.get(1).unwrap(), &lhs).unwrap();
        
        rhs.calculate_move_points(&lhs)
    }).sum()
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

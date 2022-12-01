
fn part1(input: &str) -> usize {
    let mut calories: Vec<usize> = Vec::with_capacity(512);
    let parts = input.split("\n");

    let mut current_calories: usize = 0;

    for part in parts {
        if part.len() == 0 {
            calories.push(current_calories);
            current_calories = 0;
            continue;
        }

        current_calories += part.parse::<usize>().unwrap()        
    }
    
    *calories.iter().max().unwrap()
}

fn part2(input: &str) -> usize {
    let mut calories: Vec<usize> = Vec::with_capacity(512);
    let parts = input.split("\n");

    let mut current_calories: usize = 0;

    for part in parts {
        if part.len() == 0 {
            calories.push(current_calories);
            current_calories = 0;
            continue;
        }

        current_calories += part.parse::<usize>().unwrap()        
    }
    
    calories.sort_by(|a, b| b.cmp(a));

    calories.iter().take(3).sum()
}

fn part2_shorter(input: &str) -> usize {
    let mut values: Vec<usize> = input.split("\n\n").map(|x| 
        x.split("\n")
            .map(|y| y.parse::<usize>().unwrap_or_default())
            .sum()
    ).collect();

    values.sort_by(|a, b| b.cmp(a));

    values.iter().take(3).sum()
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
    println!("part2: {}", part2_shorter(input));
}

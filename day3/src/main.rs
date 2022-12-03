use std::collections::HashSet;

fn priority(item: char) -> usize {
    match item {
        'a' ..= 'z' => (item as usize - 'a' as usize) + 1,
        'A' ..= 'Z' => (item as usize - 'A' as usize) + 27,
        _ => unreachable!()
    }
}

fn part1(input: &str) -> usize {
    input.split('\n').map(|line| {
        let (left, right) = line.split_at(line.len() / 2);

        let intersect = left.chars().find(|&item| right.contains(item));

        if intersect.is_none() { return 0; }
        
        priority(intersect.unwrap())
    }).sum()
}

fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut sum = 0;

    for i in (0..lines.len()).step_by(3) {
        let (s1, s2, s3) = (
            *lines.get(i + 0).unwrap_or(&""),
            *lines.get(i + 1).unwrap_or(&""),
            *lines.get(i + 2).unwrap_or(&"")
        );

        let h1: HashSet<char> = s1.chars().into_iter().collect();
        let h2: HashSet<char> = s2.chars().into_iter().collect();

        for c in s3.chars() {
            if h1.contains(&c) && h2.contains(&c) {
                sum += priority(c);
                break;
            }
        }
    }

    sum
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

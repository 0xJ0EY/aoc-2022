use std::collections::HashSet;

fn ascii_to_result(item: &char) -> usize {
    if !item.is_ascii() { return 0; }

    if item.is_ascii_uppercase() {
        let index = *item as usize - 'A' as usize;
        return 27 + index;
    } else {
        let index = *item as usize - 'a' as usize;
        return 1 + index;
    }
}

fn part1(input: &str) -> usize {
    input.split('\n').map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        let mut map = HashSet::with_capacity(left.len());
        
        for c in left.chars() { map.insert(c); }

        for c in right.chars() {
            if map.contains(&c) {
                return Some(c.clone())
            }
        }

        None
    }).map(|item| {
        if item.is_none() { return 0; }
        ascii_to_result(&item.unwrap())
    }).sum()
}

fn part2(input: &str) -> usize {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut sum = 0;

    for i in (0..lines.len()).step_by(3) {
        let bundle = [
            lines.get(i + 1).unwrap_or(&""),
            lines.get(i + 0).unwrap_or(&""),
            lines.get(i + 2).unwrap_or(&"")
        ];

        let mut map = HashSet::with_capacity(bundle[0].len());
        for c in bundle[0].chars() { map.insert(c); }

        let mut filtered_map = HashSet::new();
        for c in bundle[1].chars() {
            if map.contains(&c) { filtered_map.insert(c); }
        }

        for c in bundle[2].chars() {
            if filtered_map.contains(&c) {
                sum += ascii_to_result(&c);
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

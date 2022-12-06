fn is_unique_set(bytes: &[u8]) -> bool {
    let mut bitset = 0u64;

    for &byte in bytes {
        bitset |= 1 << (byte & 0b00111111);
    }

    bitset.count_ones() as usize == bytes.len()
}

fn solve(input: &[u8], window_size: usize) -> Option<usize> {
    let start = input.windows(window_size).position(|x| is_unique_set(x))?;

    Some(start + window_size)
}

fn part1(input: &[u8]) -> Option<usize> {
    solve(input, 4)
}

fn part2(input: &[u8]) -> Option<usize> {
    solve(input, 14)
}

fn main() {
    let input = include_bytes!("input.txt");

    println!("part1: {}", part1(input).unwrap());
    println!("part2: {}", part2(input).unwrap());
}

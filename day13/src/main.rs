use std::{collections::VecDeque, cmp::Ordering};

#[derive(Debug, Clone, Eq)]
enum Entry {
    List(Vec<Entry>),
    Value(usize)
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}


impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Entry::List(l), Entry::List(r)) => l.cmp(r),
            (Entry::List(_), Entry::Value(r)) => self.cmp(&Entry::List(vec![Entry::Value(*r)])),
            (Entry::Value(l), Entry::List(_)) => Entry::List(vec![Entry::Value(*l)]).cmp(other),
            (Entry::Value(l), Entry::Value(r)) => l.cmp(r),
        }
    }
}

impl Entry {
    fn parse_entry(s: &str) -> Entry {
        let mut tokens = s.chars().collect::<VecDeque<char>>();

        Entry::parse_list(&mut tokens)
    }

    fn parse_number(tokens: &mut VecDeque<char>) -> Entry {
        let mut number = String::new();

        while let Some(current_token) = tokens.front() {

            if current_token.is_numeric() {
                number.push(*current_token);
                tokens.pop_front();
            } else {
                break;
            }
        }

        Entry::Value(number.parse::<usize>().unwrap())
    }

    fn parse_list(tokens: &mut VecDeque<char>) -> Entry {
        let mut items = Vec::new();

        tokens.pop_front();

        while let Some(current_token) = tokens.front() {
            if current_token.is_numeric() {
                items.push(Entry::parse_number(tokens));
            } else
            if *current_token == '[' {
                items.push(Entry::parse_list(tokens));
            } else 
            if *current_token == ',' {
                tokens.pop_front();
            } else
            if  *current_token == ']' {
                tokens.pop_front();
                break;
            }
        }

        Entry::List(items)
    }

    fn parse(s: &str) -> Vec<(Entry, Entry)> {
        let mut entries = Vec::<(Entry, Entry)>::new();
        let pairs = s.split("\n\n").collect::<Vec<_>>();

        pairs.iter().for_each(|pair| {
            let lines = pair.split('\n').take(2).collect::<Vec<&str>>();

            entries.push((
                Entry::parse_entry(lines[0]),
                Entry::parse_entry(lines[1])
            ))
        });

        entries
    }
}

fn part1(input: &str) -> usize {
    let entries = Entry::parse(input);

    entries.iter().enumerate().map(|(index, (left, right))| {
        let in_order = left < right;
        let multiplier = if in_order { 1 } else { 0 };

        (index + 1) * multiplier
    }).sum()
}

fn part2(input: &str) -> usize {
    let mut entries = Entry::parse(input).into_iter().fold(vec![], |acc, (l, r)| {
        let mut acc = acc;
        acc.push(l);
        acc.push(r);
        acc
    });

    let d1 = Entry::List(vec![Entry::List(vec![Entry::Value(2)])]);
    let d2 = Entry::List(vec![Entry::List(vec![Entry::Value(6)])]);

    entries.push(d1.clone());
    entries.push(d2.clone());
    
    entries.sort();

    let key_part1 = entries.iter().position(|x| x == &d1).unwrap() + 1;
    let key_part2 = entries.iter().position(|x| x == &d2).unwrap() + 1;

    key_part1 * key_part2
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

struct MonkeyTest {
    value: usize,
    if_true: usize,
    if_false: usize
}

enum Value {
    Previous,
    Number(usize)
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        let value = val.parse::<usize>();

        match value {
            Ok(x) => Value::Number(x),
            Err(_) => Value::Previous,
        }
    }
}

struct Monkey {
    inspected: usize,
    items: Vec<usize>,
    rhs: Value,
    operation: fn(&Monkey, usize) -> usize,
    test: MonkeyTest
}

impl Monkey {
    fn add(&self, item: usize) -> usize {
        let rhs = match self.rhs {
            Value::Previous => item,
            Value::Number(x) => x,
        };

        item + rhs
    }

    fn multiply(&self, item: usize) -> usize {
        let rhs = match self.rhs {
            Value::Previous => item,
            Value::Number(x) => x,
        };

        item * rhs
    }

    fn inspect_part1(&mut self) -> Vec<(usize, usize)> {
        let mut actions = Vec::<(usize, usize)>::new();

        while let Some(worry_level) = self.items.pop() {
            let worry_level = (self.operation)(&self, worry_level) / 3;

            if worry_level % self.test.value == 0 {
                actions.push((self.test.if_true, worry_level));
            } else {
                actions.push((self.test.if_false, worry_level));
            }

            self.inspected += 1;
        }

        actions
    }

    fn inspect_part2(&mut self, divisor: usize) -> Vec<(usize, usize)> {
        let mut actions = Vec::<(usize, usize)>::new();

        while let Some(worry_level) = self.items.pop() {
            let worry_level = (self.operation)(&self, worry_level) % divisor;

            if worry_level % self.test.value == 0 {
                actions.push((self.test.if_true, worry_level));
            } else {
                actions.push((self.test.if_false, worry_level));
            }

            self.inspected += 1;
        }

        actions
    }
}

struct MonkeyCollective {
    divisor: usize,
    monkeys: Vec<Monkey>
}

impl MonkeyCollective {
    fn parse(s: &str) -> MonkeyCollective {
        let monkeys = s.split_terminator("\n\n").map(|entry| {
            let lines = entry.split_terminator('\n').collect::<Vec<&str>>();

            let items = lines.get(1).unwrap()["  Starting items: ".len()..]
                .split(", ")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            let test = lines.get(3).unwrap()["  Test: divisible by ".len()..].parse().unwrap();
            let if_true = lines.get(4).unwrap()["    If true: throw to monkey ".len()..].parse().unwrap();
            let if_false = lines.get(5).unwrap()["    If false: throw to monkey ".len()..].parse().unwrap();

            let operation_line = lines.get(2).unwrap();
            let operator_index = operation_line.chars().position(|x| x == '+' || x == '*').unwrap();

            let is_operation_add = &operation_line[operator_index..operator_index+1] == "+";
            let rhs = Value::from(&operation_line[operator_index + 2..]);

            Monkey {
                inspected: 0,
                items,
                rhs,
                operation: if is_operation_add { Monkey::add } else { Monkey::multiply },
                test: MonkeyTest { value: test, if_true, if_false }
            }
        }).collect();

        let divisor = MonkeyCollective::calculate_divisor(&monkeys);
        
        MonkeyCollective { divisor, monkeys }
    }

    fn round1(&mut self) {
        for index in 0..self.monkeys.len() {
            for (target, worry_level) in self.monkeys[index].inspect_part1() {
                self.monkeys[target].items.push(worry_level);
            }
        }
    }

    fn round2(&mut self) {
        for index in 0..self.monkeys.len() {
            for (target, worry_level) in self.monkeys[index].inspect_part2(self.divisor) {
                self.monkeys[target].items.push(worry_level);
            }
        }
    }

    fn calculate_divisor(monkeys: &Vec<Monkey>) -> usize {
        monkeys.iter().fold(1, |acc, monkey| acc * monkey.test.value)
    }

    fn calculate_monkey_business(&self) -> usize {
        let mut m: Vec<&Monkey> = self.monkeys.iter().collect();

        m.sort_by(|a, b| b.inspected.cmp(&a.inspected));

        let first = m[0].inspected;
        let second = m[1].inspected;

        first * second
    }
}

fn part1(input: &str) -> usize {
    let mut monkey_collective = MonkeyCollective::parse(input);

    for _ in 0..20 {
        monkey_collective.round1();
    }

    monkey_collective.calculate_monkey_business()
}

fn part2(input: &str) -> usize {
    let mut monkey_collective = MonkeyCollective::parse(input);

    for _ in 0..10000 {
        monkey_collective.round2();
    }

    monkey_collective.calculate_monkey_business()
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

#[derive(Debug)]
struct MonkeyTest {
    predicate: MonkeyOperation,    
    if_true: usize,
    if_false: usize
}

impl MonkeyTest {
    fn from_input_text(lines: &Vec<&str>) -> MonkeyTest {
        let test_parts = &lines.get(3).unwrap()["  Test: ".len()..].split(" ").collect::<Vec<_>>();
        let (operator, value) = (*test_parts.first().unwrap(), *test_parts.last().unwrap());
        let value = Literal::parse(value);

        let predicate = match operator {
            "divisible" => MonkeyOperation::Divisible(value),
            _ => unreachable!("Other operators not supported")
        };

        let if_true = lines.get(4).unwrap()["    If true: throw to monkey ".len()..].parse().unwrap();
        let if_false = lines.get(5).unwrap()["    If false: throw to monkey ".len()..].parse().unwrap();

        MonkeyTest { predicate, if_true, if_false }
    }
}

#[derive(Debug)]
enum Literal {
    Number(usize),
    Variable(String),
}

impl Literal {
    fn parse(s: &str) -> Literal {
        match s.trim().parse() {
            Ok(value) => Literal::Number(value),
            Err(_) => Literal::Variable(String::from(s))
        }
    }

    fn get_value(&self, other: &usize) -> usize {
        match self {
            Literal::Number(value) => *value,
            Literal::Variable(_) => *other,
        }
    }
}

#[derive(Debug)]
enum MonkeyOperation {
    Times(Literal),
    Add(Literal),
    Divisible(Literal)
}

impl MonkeyOperation {
    fn from_operation_line(s: &str) -> MonkeyOperation {
        let (operator, value) = s["  Operation: new = old ".len()..]
            .split_once(" ")
            .unwrap();

        let value = Literal::parse(value);

        match operator {
            "+" => MonkeyOperation::Add(value),
            "*" => MonkeyOperation::Times(value),
            _ => todo!()
        }
    }
}

#[derive(Debug)]
struct Monkey {
    inspected_items: usize,
    items: Vec<usize>,
    operation: MonkeyOperation,
    test: MonkeyTest
}

impl Monkey {
    fn parse(s: &str) -> Monkey {
        let lines: Vec<&str> = s.split_terminator('\n').collect();

        let items_range: Vec<usize> = lines.get(1)
            .unwrap()["  Starting items:".len()..]
            .split(',')
            .map(|x| x.trim().parse::<usize>().unwrap())
            .collect();

        let operation = MonkeyOperation::from_operation_line(lines.get(2).unwrap());

        let test = MonkeyTest::from_input_text(&lines);

        Monkey { inspected_items: 0, items: items_range, operation, test }
    }

    fn round_actions(&mut self) -> Vec<(usize, usize)> {
        let mut actions = Vec::<(usize, usize)>::new();

        while let Some(worry_level) = self.items.pop() {
            
            let worry_level = match &self.operation {
                MonkeyOperation::Times(lit) => (worry_level * lit.get_value(&worry_level)) / 3,
                MonkeyOperation::Add(lit) => (worry_level + lit.get_value(&worry_level)) / 3,
                _ => todo!()
            };

            let is_divisible = match &self.test.predicate {                
                MonkeyOperation::Divisible(by) => {
                    if let Literal::Number(x) = by {
                        worry_level % x == 0
                    } else {
                        todo!("Variable support not implemented")
                    }
                },
                _ => todo!()
            };

            actions.push((worry_level, if is_divisible { self.test.if_true } else { self.test.if_false }));

            self.inspected_items += 1;
        }
        
        actions
    }

    fn round_actions2(&mut self, divisor: usize) -> Vec<(usize, usize)> {
        let mut actions = Vec::<(usize, usize)>::new();

        while let Some(worry_level) = self.items.pop() {
            
            let worry_level = match &self.operation {
                MonkeyOperation::Times(lit) => worry_level * lit.get_value(&worry_level),
                MonkeyOperation::Add(lit) => worry_level + lit.get_value(&worry_level),
                _ => todo!()
            };
            
            let worry_level = worry_level % divisor;

            let is_divisible = match &self.test.predicate {                
                MonkeyOperation::Divisible(by) => {
                    if let Literal::Number(x) = by {
                        worry_level % x == 0
                    } else {
                        todo!("Variable support not implemented")
                    }
                },
                _ => todo!()
            };

            actions.push((worry_level, if is_divisible { self.test.if_true } else { self.test.if_false }));

            self.inspected_items += 1;
        }
        
        actions
    }
}

#[derive(Debug)]
struct MonkeyCollective {
    monkeys: Vec<Monkey>
}

impl MonkeyCollective {
    fn parse(s: &str) -> MonkeyCollective {
        let monkeys = s.split_terminator("\n\n").map(Monkey::parse).collect();

        MonkeyCollective { monkeys }
    }

    fn calculate_divisor(&self) -> usize {
        let mut modulo = 1;

        self.monkeys.iter().for_each(|monkey| {
            if let MonkeyOperation::Divisible(literal) = &monkey.test.predicate {
                if let Literal::Number(value) = &literal {
                    modulo *= value;
                }
            }
        });

        modulo
    }

    fn process_round(&mut self, divisor: usize) {
        for index in 0..self.monkeys.len() {
            let actions = self.monkeys.get_mut(index).unwrap().round_actions();

            actions.iter().for_each(|(item, monkey_index)| {
                let monkey = self.monkeys.get_mut(*monkey_index).unwrap();
                monkey.items.push(*item);
            });
        }
    }

    fn process_round2(&mut self, divisor: usize) {
        for index in 0..self.monkeys.len() {
            let actions = self.monkeys.get_mut(index).unwrap().round_actions2(divisor);

            actions.iter().for_each(|(item, monkey_index)| {
                let monkey = self.monkeys.get_mut(*monkey_index).unwrap();
                monkey.items.push(*item);
            });
        }
    }

    fn calculate_monkey_business(&self) -> usize {
        let mut m: Vec<&Monkey> = self.monkeys.iter().collect();

        m.sort_by(|a, b| b.inspected_items.cmp(&a.inspected_items));

        let first   = m.first().unwrap().inspected_items;
        let second  = m.get(1).unwrap().inspected_items;

        first * second
    } 

}

fn part1(input: &str) -> usize {
    let mut monkey_collective = MonkeyCollective::parse(input);

    for _ in 0..20 {
        monkey_collective.process_round(3);
    }

    monkey_collective.calculate_monkey_business()
}

fn part2(input: &str) -> usize {
    let mut monkey_collective = MonkeyCollective::parse(input);

    let divisor = monkey_collective.calculate_divisor();

    for _ in 0..10000 {
        monkey_collective.process_round2(divisor);
    }

    monkey_collective.calculate_monkey_business()
}

fn main() {
    let input = include_str!("input.txt");

    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

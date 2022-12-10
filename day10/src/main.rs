use std::vec;

#[derive(Debug)]
enum Cycle {
    Noop,
    Addx(isize)
}

impl Cycle {
    fn collection_from_str(s: &str) -> Vec<Cycle> {
        s.split_terminator('\n').map(|line| {
            let parts = line.split(" ").take(2).collect::<Vec<&str>>();

            match *parts.first().expect("Missing operation") {
                "noop" => return Cycle::Noop,
                "addx" => {
                    let value = *parts.get(1).expect("Missing value");
                    let value = value.parse().unwrap();

                    Cycle::Addx(value)
                }
                _ => unreachable!()
            }
        }).collect::<Vec<_>>()
    }

    fn ticks(&self) -> usize {
        match self {
            Cycle::Noop => 1,
            Cycle::Addx(_) => 2,
        }
    }
}

#[derive(Debug)]
struct CPU {
    value: isize,
    current_tick: usize,
}

impl CPU {
    fn new() -> CPU {
        CPU {
            value: 1,
            current_tick: 0,
        }
    }

    fn draw_pixel(&self) -> () {
        let lower = (self.value % 40) - 1;
        let higher = (self.value % 40) + 1;

        let tick = (self.current_tick % 40) as isize;

        if tick >= lower && tick <= higher {
            print!("#");
        } else {
            print!(".");
        }

        if tick % 40 == 39 {
            println!("");
        }
    }

    fn measure_signal_strength(&self) -> Option<isize> {
        let is_20 = self.current_tick == 20;
        let is_40_after_20 = (self.current_tick as isize - 20) % 40 == 0;

        if is_20 || is_40_after_20 {
            Some(self.current_tick as isize * self.value)
        } else {
            None
        }
    }

    fn process_tick(&mut self, cycle: &Cycle) -> Vec<isize> {
        let mut sum = vec![];

        for _ in 0..cycle.ticks() {
            self.draw_pixel();

            self.current_tick += 1;
            if let Some(value) = self.measure_signal_strength() {
                sum.push(value);
            }
        }

        if let Cycle::Addx(value) = cycle {
            self.value += value;
        }

        sum
    }

    fn process_ticks(&mut self, ticks: &Vec<Cycle>) -> Vec<isize> {
        let mut measurements = Vec::<isize>::new();

        ticks.iter().for_each(|cycle| {
            measurements.append(&mut self.process_tick(cycle));
        });

        measurements
    }
}

fn solve(input: &str) -> () {
    let cycles = Cycle::collection_from_str(input);
    let mut cpu = CPU::new();

    let signal_strength: isize = cpu.process_ticks(&cycles).iter().sum();
    println!("signal strength: {}", signal_strength);
}

fn main() {
    let input = include_str!("input.txt");
    solve(input);
}

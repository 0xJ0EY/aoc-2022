use std::collections::HashSet;

#[derive(Debug)]
struct Sensor {
    loc: (i64, i64),
    radius: i64,
}

impl Sensor {
    fn parse(s: &str) -> Vec<Sensor> {
        s.split_terminator('\n').map(|line| {
            let (sensor_str, beacon_str) = line.split_once(": closest beacon is at ").unwrap();
            let sensor_str = &sensor_str["Sensor at ".len()..];

            let sensor_loc = {
                let (x_str, y_str) = sensor_str.split_once(", ").unwrap();
                let x = (&x_str[2..]).parse::<i64>().unwrap();
                let y = (&y_str[2..]).parse::<i64>().unwrap();

                (x, y)
            };

            let beacon_loc = {
                let (x_str, y_str) = beacon_str.split_once(", ").unwrap();
                let x = (&x_str[2..]).parse::<i64>().unwrap();
                let y = (&y_str[2..]).parse::<i64>().unwrap();

                (x, y)
            };

            let radius = (sensor_loc.0 - beacon_loc.0).abs() + (sensor_loc.1 - beacon_loc.1).abs();

            Sensor {
                loc: sensor_loc,
                radius,
            }
        }).collect::<Vec<Sensor>>()
    }

    fn covers_grid(&self, x1: i64, y1: i64, x2: i64, y2: i64) -> bool {
        let (sx, sy) = self.loc;

        // Check for every corner of the grid the manhattan distance to the sensor
        // If the corner that is furthest away from the sensor is still lower then the radius of the sensor
        // We know that the sensor completely covers the grid
        let max = [(x1, y1), (x1, y2), (x2, y1), (x2, y2)].iter()
            .map(|(x, y)| (x - sx).abs() + (y - sy).abs())
            .max().unwrap();

        max <= self.radius
    }
}

fn part1(s: &str) -> usize {
    let sensors = Sensor::parse(s);
    let target_row = 2000000;

    let in_range = sensors.iter()
        .filter(|sensor| {
            let (_, y) = sensor.loc;
            let radius = sensor.radius;

            let lower = y + radius;
            let upper = y - radius;

            target_row >= upper && target_row <= lower
        })
        .collect::<Vec<&Sensor>>();

    let squares = in_range.iter().map(|sensor| {
        let (x, y) = sensor.loc;
        
        let distance        = (target_row - y).abs();
        let current_radius  = sensor.radius - distance;

        let squares = (current_radius * 2).max(0);
        let start = x - (squares / 2);

        let mut used_squares = Vec::<i64>::new();

        for index in 0..squares {
            used_squares.push(start + index);
        }

        used_squares
    }).fold(HashSet::<i64>::new(), |mut acc, x| {
        x.into_iter().for_each(|square| {
            acc.insert(square);
        });

        acc
    });

    squares.len()
}

fn part2(s: &str) -> i64 {
    fn quadrants(x1: i64, y1: i64, x2: i64, y2: i64) -> Vec<(i64, i64, i64, i64)> {
        fn middle(x: i64, y: i64) -> i64 { (x + y) / 2 }

        let top_left    = (x1, y1, middle(x1, x2), middle(y1, y2));
        let top_right   = (middle(x1, x2) + 1, y1, x2, middle(y1, y2));
        let bot_left    = (x1, middle(y1, y2) + 1, middle(x1, x2), y2); 
        let bot_right   = (middle(x1, x2) + 1, middle(y1, y2) + 1, x2, y2);

        vec![top_left, bot_left, top_right, bot_right]
    }

    let sensors = Sensor::parse(s);
    let (search_width, search_height) = (4000000, 4000000);

    let mut partitions: Vec<(i64, i64, i64, i64)> = vec![(0, 0, search_width, search_height)];

    while let Some((x1, y1, x2, y2)) = partitions.pop() {

        // Check if the current partition is covered by any sensor, if it is, we skip it
        if sensors.iter().any(|sensor| sensor.covers_grid(x1, y1, x2, y2)) {
            continue;
        }

        // We have the first one that is the size of 1 square (without any sensors that covers the grid)
        if x1 == x2 && y1 == y2 {
            return 4_000_000 * x1 + y2;
        }

        // Since we didn't return on the previous statement, we still have some space to partition
        for partition in quadrants(x1, y1, x2, y2) {
            partitions.push(partition);
        }
    }

    panic!("No distress beacon found");
}

fn main() {
    let input = include_str!("input.txt");
    println!("part1: {}", part1(input));
    println!("part2: {}", part2(input));
}

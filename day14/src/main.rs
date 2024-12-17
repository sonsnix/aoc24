use regex::Regex;
use std::{collections::HashSet, fs};

const MAX_X: i32 = 101;
const MAX_Y: i32 = 103;

// Define a struct for Robots
#[derive(Debug)]
struct Robot {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[derive(Debug)]
struct Map {
    max_x: i32,
    max_y: i32,
    robots: Vec<Robot>,
}

impl Map {
    fn step(&mut self) {
        for robot in &mut self.robots {
            robot.position.0 = (robot.position.0 + robot.velocity.0).rem_euclid(self.max_x);
            robot.position.1 = (robot.position.1 + robot.velocity.1).rem_euclid(self.max_y);
        }
    }

    fn safety_factor(&self) -> usize {
        [
            self.robots
                .iter()
                .filter(|robot| {
                    robot.position.0 < self.max_x / 2 && robot.position.1 < self.max_y / 2
                })
                .count(),
            self.robots
                .iter()
                .filter(|robot| {
                    robot.position.0 > self.max_x / 2 && robot.position.1 < self.max_y / 2
                })
                .count(),
            self.robots
                .iter()
                .filter(|robot| {
                    robot.position.0 < self.max_x / 2 && robot.position.1 > self.max_y / 2
                })
                .count(),
            self.robots
                .iter()
                .filter(|robot| {
                    robot.position.0 > self.max_x / 2 && robot.position.1 > self.max_y / 2
                })
                .count(),
        ]
        .into_iter()
        .fold(1, |acc, count| acc * count) as usize
    }

    fn _print(&self) {
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                if self.robots.iter().any(|robot| robot.position == (x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn spread_out(&self) -> bool {
        self.robots
            .iter()
            .map(|robot| (robot.position.0, robot.position.1))
            .collect::<HashSet<_>>()
            .len()
            == self.robots.len()
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    // Define the regex to match the lines
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    // Parse each line and collect results
    input
        .lines()
        .filter_map(|line| {
            if let Some(captures) = re.captures(line) {
                let position = (captures[1].parse().unwrap(), captures[2].parse().unwrap());
                let velocity = (captures[3].parse().unwrap(), captures[4].parse().unwrap());
                Some(Robot { position, velocity })
            } else {
                None
            }
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the file contents
    let file_contents = fs::read_to_string("input.txt")?;

    // Parse the robots
    let robots = parse_robots(&file_contents);

    let mut map = Map {
        max_x: MAX_X,
        max_y: MAX_Y,
        robots,
    };

    for i in 0..10000 {
        map.step();
        if i == 99 {
            println!("Part 1: {}", map.safety_factor());
        }
        if map.spread_out() {
            println!("Part 2: {}", i + 1);
            // map.print();
            break;
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Read the file contents
        let file_contents = fs::read_to_string("test.txt").unwrap();

        // Parse the robots
        let robots = parse_robots(&file_contents);

        let mut map = Map {
            max_x: 11,
            max_y: 7,
            robots,
        };

        for _ in 0..100 {
            map.step();
        }

        println!("{:?}", map);

        assert_eq!(map.safety_factor(), 12);
    }
}

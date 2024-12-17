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

// Define a struct for the Map
#[derive(Debug)]
struct Map {
    max_x: i32,
    max_y: i32,
    robots: Vec<Robot>,
}

impl Map {
    // Simulate one step of movement for all robots
    fn step(&mut self) {
        for robot in &mut self.robots {
            robot.position.0 = (robot.position.0 + robot.velocity.0).rem_euclid(self.max_x);
            robot.position.1 = (robot.position.1 + robot.velocity.1).rem_euclid(self.max_y);
        }
    }

    // Calculate the safety factor based on robot positions in quadrants
    fn safety_factor(&self) -> usize {
        let quadrant_counts = [
            // Top-left quadrant
            self.robots
                .iter()
                .filter(|robot| robot.position.0 < self.max_x / 2 && robot.position.1 < self.max_y / 2)
                .count(),
            // Top-right quadrant
            self.robots
                .iter()
                .filter(|robot| robot.position.0 > self.max_x / 2 && robot.position.1 < self.max_y / 2)
                .count(),
            // Bottom-left quadrant
            self.robots
                .iter()
                .filter(|robot| robot.position.0 < self.max_x / 2 && robot.position.1 > self.max_y / 2)
                .count(),
            // Bottom-right quadrant
            self.robots
                .iter()
                .filter(|robot| robot.position.0 > self.max_x / 2 && robot.position.1 > self.max_y / 2)
                .count(),
        ];

        quadrant_counts.into_iter().product()
    }

    // Print the map to visualize robot positions
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

    // Check if all robots are in unique positions
    fn spread_out(&self) -> bool {
        let unique_positions: HashSet<_> = self.robots.iter().map(|robot| robot.position).collect();
        unique_positions.len() == self.robots.len()
    }
}

// Parse robots from the input string
fn parse_robots(input: &str) -> Vec<Robot> {
    let re = Regex::new(r"p=(-?\d+),(-?\d+) v=(-?\d+),(-?\d+)").unwrap();

    input
        .lines()
        .filter_map(|line| {
            re.captures(line).map(|captures| Robot {
                position: (
                    captures[1].parse().unwrap(),
                    captures[2].parse().unwrap(),
                ),
                velocity: (
                    captures[3].parse().unwrap(),
                    captures[4].parse().unwrap(),
                ),
            })
        })
        .collect()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read the file contents
    let file_contents = fs::read_to_string("input.txt")?;

    // Parse the robots from the input
    let robots = parse_robots(&file_contents);

    // Initialize the map
    let mut map = Map {
        max_x: MAX_X,
        max_y: MAX_Y,
        robots,
    };

    for i in 1..10000 {
        map.step();

        // Part 1: Calculate safety factor after 100 steps
        if i == 100 {
            println!("Part 1: {}", map.safety_factor());
        }

        // Part 2: Determine when robots spread out
        if map.spread_out() {
            println!("Part 2: {}", i);
            break;
        }
    }

    Ok(())
}

// Unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        // Read the test file contents
        let file_contents = fs::read_to_string("test.txt").unwrap();

        // Parse the robots from the test input
        let robots = parse_robots(&file_contents);

        // Initialize the map
        let mut map = Map {
            max_x: 11,
            max_y: 7,
            robots,
        };

        // Simulate 100 steps
        for _ in 0..100 {
            map.step();
        }

        // Assert that the safety factor matches the expected value
        assert_eq!(map.safety_factor(), 12);
    }
}

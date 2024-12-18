use std::collections::HashSet;
use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Warehouse {
    walls: HashSet<Position>,
    boxes: HashSet<Position>,
    robot: Position,
    width: i32,
    height: i32,
}

impl Warehouse {
    fn new(map: &str) -> Self {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = Position { x: 0, y: 0 };

        let lines: Vec<&str> = map.lines().collect();
        let height = lines.len() as i32;
        let width = lines.first().map(|l| l.len()).unwrap_or(0) as i32;

        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = Position {
                    x: x as i32,
                    y: y as i32,
                };

                match ch {
                    '#' => {
                        walls.insert(pos);
                    }
                    'O' => {
                        boxes.insert(pos);
                    }
                    '@' => {
                        robot = pos;
                    }
                    _ => {}
                }
            }
        }

        Warehouse {
            walls,
            boxes,
            robot,
            width,
            height,
        }
    }

    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let pos = Position { x, y };
                let ch = if pos == self.robot {
                    '@'
                } else if self.walls.contains(&pos) {
                    '#'
                } else if self.boxes.contains(&pos) {
                    'O'
                } else {
                    '.'
                };
                print!("{}", ch);
            }
            println!();
        }
        println!(); // Extra newline for readability
    }

    fn is_valid_move(&self, pos: Position) -> bool {
        !self.walls.contains(&pos)
    }

    fn move_robot(&mut self, direction: char) -> bool {
        let delta = match direction {
            '^' => Position { x: 0, y: -1 },
            'v' => Position { x: 0, y: 1 },
            '<' => Position { x: -1, y: 0 },
            '>' => Position { x: 1, y: 0 },
            _ => return false,
        };

        let new_robot_pos = Position {
            x: self.robot.x + delta.x,
            y: self.robot.y + delta.y,
        };

        // Check if new robot position is blocked by a wall
        if !self.is_valid_move(new_robot_pos) {
            return false;
        }

        // Check if there are boxes to be pushed and track their positions
        let mut boxes_to_push = Vec::new();
        let mut current_pos = new_robot_pos;

        // Collect consecutive boxes
        while self.boxes.contains(&current_pos) {
            boxes_to_push.push(current_pos);
            current_pos = Position {
                x: current_pos.x + delta.x,
                y: current_pos.y + delta.y,
            };
        }

        // Check if pushing is possible
        if !boxes_to_push.is_empty() {
            // Check if the final position after pushing is free
            if !self.walls.contains(&current_pos) && !self.boxes.contains(&current_pos) {
                // Remove boxes from their original positions
                for &box_pos in &boxes_to_push {
                    self.boxes.remove(&box_pos);
                }

                // Add boxes to their new positions
                for &box_pos in &boxes_to_push {
                    let new_box_pos = Position {
                        x: box_pos.x + delta.x,
                        y: box_pos.y + delta.y,
                    };
                    self.boxes.insert(new_box_pos);
                }
            } else {
                // Cannot push boxes
                return false;
            }
        }

        // Move robot
        self.robot = new_robot_pos;
        true
    }

    fn calculate_gps_coordinates(&self) -> i32 {
        self.boxes.iter().map(|pos| 100 * pos.y + pos.x).sum()
    }
}

fn parse_input(input: &str) -> (String, Vec<char>) {
    // Separate map from moves
    let input = input.replace("\r\n", "\n");
    let mut parts = input.split("\n\n");

    let map = parts.next().unwrap().to_string();

    // Collect moves, ignoring whitespace
    let moves: Vec<char> = parts
        .next()
        .unwrap()
        .chars()
        .filter(|&c| c == '^' || c == 'v' || c == '<' || c == '>')
        .collect();

    (map, moves)
}

pub fn solve_warehouse_puzzle(input: &str) -> i32 {
    let (map, moves) = parse_input(input);

    let mut warehouse = Warehouse::new(&map);

    // println!("Initial state:");
    // warehouse.display();

    for (i, mv) in moves.iter().enumerate() {
        warehouse.move_robot(*mv);
        // println!("After move {}: {}", i + 1, mv);
        // warehouse.display();
    }

    warehouse.calculate_gps_coordinates()
}

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

struct Warehouse {
    walls: HashSet<Position>,
    boxes: HashSet<Position>,
    robot: Position,
}

impl Warehouse {
    fn new(map: &str) -> Self {
        let mut walls = HashSet::new();
        let mut boxes = HashSet::new();
        let mut robot = Position { x: 0, y: 0 };

        let lines: Vec<&str> = map.lines().collect();
        
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let pos = Position {
                    x: x as i32 * 2,
                    y: y as i32,
                };

                match ch {
                    '#' => {
                        walls.insert(pos);
                        walls.insert(Position {
                            x: pos.x + 1,
                            y: pos.y,
                        });
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
        }
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

        // Check if there are boxes to be pushed and track their positions
        let mut boxes_to_push = Vec::new();

        // Collect consecutive boxes
        let mut queue = vec![new_robot_pos];
        while let Some(pos) = queue.pop() {
            if self.walls.contains(&pos) {
                return false;
            }

            let neighbor = Position {
                x: pos.x - 1,
                y: pos.y,
            };

            if direction == '<' && self.boxes.contains(&neighbor) {
                boxes_to_push.push(neighbor);
                queue.push(Position {
                    x: neighbor.x - 1,
                    y: neighbor.y,
                });
            } else if direction == '>' && self.boxes.contains(&pos) {
                boxes_to_push.push(pos);
                queue.push(Position {
                    x: pos.x + 2,
                    y: pos.y,
                });
            } else if direction == '^' || direction == 'v' {
                if self.boxes.contains(&pos) {
                    boxes_to_push.push(pos);
                    queue.push(Position {
                        x: pos.x + delta.x,
                        y: pos.y + delta.y,
                    });
                    queue.push(Position {
                        x: pos.x + delta.x + 1,
                        y: pos.y + delta.y,
                    });
                } else if self.boxes.contains(&neighbor) {
                    boxes_to_push.push(neighbor);
                    queue.push(Position {
                        x: pos.x + delta.x,
                        y: pos.y + delta.y,
                    });
                    queue.push(Position {
                        x: neighbor.x + delta.x,
                        y: neighbor.y + delta.y,
                    });
                }
            }
        }

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

    for mv in moves {
        warehouse.move_robot(mv);
    }

    warehouse.calculate_gps_coordinates()
}

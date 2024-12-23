use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
    Press,
}

enum KeypadType {
    Standard,    // The door keypad
    Directional, // The robot control pad
}

#[derive(Debug)]
struct Keypad {
    buttons: HashMap<Position, char>,
}

impl Keypad {
    fn new(keypad_type: KeypadType) -> Self {
        let buttons = match keypad_type {
            KeypadType::Standard => {
                let mut map = HashMap::new();
                // First row
                map.insert(Position { x: 0, y: 2 }, '7');
                map.insert(Position { x: 1, y: 2 }, '8');
                map.insert(Position { x: 2, y: 2 }, '9');
                // Second row
                map.insert(Position { x: 0, y: 1 }, '4');
                map.insert(Position { x: 1, y: 1 }, '5');
                map.insert(Position { x: 2, y: 1 }, '6');
                // Third row
                map.insert(Position { x: 0, y: 0 }, '1');
                map.insert(Position { x: 1, y: 0 }, '2');
                map.insert(Position { x: 2, y: 0 }, '3');
                // Fourth row
                map.insert(Position { x: 0, y: -1 }, '0');
                map.insert(Position { x: 1, y: -1 }, 'A');
                map
            }
            KeypadType::Directional => {
                let mut map = HashMap::new();
                // First row
                map.insert(Position { x: 1, y: 1 }, '^');
                map.insert(Position { x: 2, y: 1 }, 'A');
                // Second row
                map.insert(Position { x: 0, y: 0 }, '<');
                map.insert(Position { x: 1, y: 0 }, 'v');
                map.insert(Position { x: 2, y: 0 }, '>');
                map
            }
        };

        Self { buttons }
    }

    fn pos_after_move(&mut self, pos: Position, direction: Direction) -> Option<Position> {
        let new_pos = match direction {
            Direction::Up => Position {
                x: pos.x,
                y: pos.y + 1,
            },
            Direction::Down => Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Direction::Left => Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Direction::Right => Position {
                x: pos.x + 1,
                y: pos.y,
            },
        };

        // Only move if the new position exists on the keypad
        self.buttons.contains_key(&new_pos).then_some(new_pos)
    }
}

#[derive(Debug)]
struct State {
    robot_keypads: Vec<Position>,
    door_keypad: Position,
}

const MOVES: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let keypad = Keypad::new(KeypadType::Standard);
    let dirpad = Keypad::new(KeypadType::Directional);

    for code in input.lines() {
        let mut state = State {
            my_keypad: Position { x: 2, y: 1 },
            robot_keypads: vec![Position { x: 2, y: 1 }; 2],
            door_keypad: Position { x: 1, y: -1 },
        };



        for c in code.chars() {

        }

        println!("{:?}", state);

        break;
    }
}

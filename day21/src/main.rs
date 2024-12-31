use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt, usize,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Action {
    Up,
    Down,
    Left,
    Right,
    Press,
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}

impl Action {
    fn from(c: char) -> Self {
        match c {
            '^' => Action::Up,
            'v' => Action::Down,
            '<' => Action::Left,
            '>' => Action::Right,
            'A' => Action::Press,
            _ => panic!("Unknown action: {}", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Action::Up => '^',
            Action::Down => 'v',
            Action::Left => '<',
            Action::Right => '>',
            Action::Press => 'A',
        }
    }
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
                map.insert(Position { x: 1, y: -1 }, '0');
                map.insert(Position { x: 2, y: -1 }, 'A');
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

    fn pos_after_move(&self, pos: Position, direction: Action) -> Option<Position> {
        let new_pos = match direction {
            Action::Up => Position {
                x: pos.x,
                y: pos.y + 1,
            },
            Action::Down => Position {
                x: pos.x,
                y: pos.y - 1,
            },
            Action::Left => Position {
                x: pos.x - 1,
                y: pos.y,
            },
            Action::Right => Position {
                x: pos.x + 1,
                y: pos.y,
            },
            Action::Press => pos,
        };

        // Only move if the new position exists on the keypad
        self.buttons.contains_key(&new_pos).then_some(new_pos)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    robot_keypads: Vec<Position>,
    door_keypad: Position,
    remaining_code: Vec<char>,
}

impl State {
    fn new(code: Vec<char>) -> Self {
        Self {
            robot_keypads: vec![Position { x: 2, y: 1 }; 2],
            door_keypad: Position { x: 2, y: -1 },
            remaining_code: code.into_iter().rev().collect(),
        }
    }

    fn step(&self, action: Action, keypad: &Keypad, dirpad: &Keypad) -> Option<State> {
        let mut new_state = self.clone();

        new_state.robot_keypads[0] = dirpad.pos_after_move(self.robot_keypads[0], action)?;

        if action == Action::Press {
            let sub_action = Action::from(*dirpad.buttons.get(&self.robot_keypads[0])?);

            new_state.robot_keypads[1] =
                dirpad.pos_after_move(self.robot_keypads[1], sub_action)?;

            if sub_action == Action::Press {
                let sub_sub_action = Action::from(*dirpad.buttons.get(&self.robot_keypads[1])?);
                new_state.door_keypad = keypad.pos_after_move(self.door_keypad, sub_sub_action)?;

                if sub_sub_action == Action::Press {
                    if let Some(&c) = keypad.buttons.get(&new_state.door_keypad) {
                        if let Some(c_expected) = new_state.remaining_code.pop() {
                            if c != c_expected {
                                return None;
                            }
                        }
                    }
                }
            }
        }

        Some(new_state)
    }
}

const MOVES: [Action; 5] = [Action::Up, Action::Down, Action::Left, Action::Right, Action::Press];

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let keypad = Keypad::new(KeypadType::Standard);
    let dirpad = Keypad::new(KeypadType::Directional);

    let mut total_complexity = 0;

    for code in input.lines() {
        println!("Looking at code: {}", code);

        let state = State::new(code.chars().collect());

        let mut queue = VecDeque::from([(state, vec![])]);

        let mut shortest_path = usize::MAX;
        let mut visited = HashMap::new();

        while let Some((state, path)) = queue.pop_front() {
            if let Some(&steps)= visited.get(&state.clone()) {
                if steps < path.len() {
                    continue;
                }
            }
            visited.insert(state.clone(), path.len());

            if state.remaining_code.is_empty() {
                if path.len() < shortest_path {
                    shortest_path = path.len();
                }
                continue;
            }
            if path.len() > shortest_path {
                continue;
            }

            for action in MOVES {
                if let Some(new_state) = state.step(action, &keypad, &dirpad) {
                    let mut new_path = path.clone();
                    new_path.push(action);
                    queue.push_back((new_state, new_path));
                }
            }
        }

        let complexity = shortest_path * code[..code.len() - 1].parse::<usize>().unwrap();

        println!("complexity: {}", complexity);

        total_complexity += complexity;
    }
    println!("Part 1: {}", total_complexity);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step() {
        let test_tuples = vec![(
            "029A",
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A",
        ),
        (
            "980A",
            "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A",
        ),(
            "179A",
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A",
        )];

        let keypad = Keypad::new(KeypadType::Standard);
        let dirpad = Keypad::new(KeypadType::Directional);

        for (code, actions) in test_tuples {
            let mut state = State::new(code.chars().collect());

            for action in actions.chars().map(|c| Action::from(c)) {
                if let Some(new_state) = state.step(action, &keypad, &dirpad) {
                    state = new_state;
                }
            }
            assert_eq!(state.remaining_code.len(), 0);
        }
    }
}

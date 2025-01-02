use cached::proc_macro::cached;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Position {
    x: i32,
    y: i32,
}

// Keypad layouts
const STANDARD_KEYPAD: &[(Position, char)] = &[  
    (Position { x: 0, y: 2 }, '7'),
    (Position { x: 1, y: 2 }, '8'),
    (Position { x: 2, y: 2 }, '9'),
    (Position { x: 0, y: 1 }, '4'),
    (Position { x: 1, y: 1 }, '5'),
    (Position { x: 2, y: 1 }, '6'),
    (Position { x: 0, y: 0 }, '1'),
    (Position { x: 1, y: 0 }, '2'),
    (Position { x: 2, y: 0 }, '3'),
    (Position { x: 1, y: -1 }, '0'),
    (Position { x: 2, y: -1 }, 'A'),
];

const DIRECTIONAL_KEYPAD: &[(Position, char)] = &[  
    (Position { x: 1, y: 1 }, '^'),
    (Position { x: 2, y: 1 }, 'A'),
    (Position { x: 0, y: 0 }, '<'),
    (Position { x: 1, y: 0 }, 'v'),
    (Position { x: 2, y: 0 }, '>'),
];

#[cached]
fn get_position(directional: bool, target: char) -> Position {
    let layout = if directional {
        DIRECTIONAL_KEYPAD
    } else {
        STANDARD_KEYPAD
    };

    layout
        .iter()
        .find(|(_, button)| *button == target)
        .map(|(pos, _)| *pos)
        .expect("Target not found in keypad layout")
}

#[cached]
fn pos_after_move(directional: bool, current: Position, action: char) -> Option<Position> {
    let new_pos = match action {
        '^' => Position { x: current.x, y: current.y + 1 },
        'v' => Position { x: current.x, y: current.y - 1 },
        '<' => Position { x: current.x - 1, y: current.y },
        '>' => Position { x: current.x + 1, y: current.y },
        _ => panic!("Unknown action: {}", action),
    };

    let layout = if directional {
        DIRECTIONAL_KEYPAD
    } else {
        STANDARD_KEYPAD
    };

    layout.iter().any(|(pos, _)| *pos == new_pos).then_some(new_pos)
}

#[cached]
fn paths_from_to(directional: bool, from: char, to: char) -> Vec<String> {
    let start = get_position(directional, from);
    let end = get_position(directional, to);

    let horizontal_moves = match end.x - start.x {
        delta if delta > 0 => vec!['>'; delta as usize],
        delta if delta < 0 => vec!['<'; -delta as usize],
        _ => vec![],
    };

    let vertical_moves = match end.y - start.y {
        delta if delta > 0 => vec!['^'; delta as usize],
        delta if delta < 0 => vec!['v'; -delta as usize],
        _ => vec![],
    };

    let paths = HashSet::from([
        [horizontal_moves.clone(), vertical_moves.clone()].concat(),
        [vertical_moves, horizontal_moves].concat(),
    ]);

    paths
        .into_iter()
        .filter_map(|path| {
            let mut position = start;
            if path.iter().all(|&action| {
                if let Some(new_pos) = pos_after_move(directional, position, action) {
                    position = new_pos;
                    true
                } else {
                    false
                }
            }) {
                Some(path.into_iter().chain(std::iter::once('A')).collect::<String>())
            } else {
                None
            }
        })
        .collect()
}

#[cached]
fn solve_code(sequence: String, directional: bool, level: usize) -> usize {
    let from_to_pairs = std::iter::once('A').chain(sequence.chars()).zip(sequence.chars());

    let paths: Vec<Vec<String>> = from_to_pairs
        .map(|(from, to)| paths_from_to(directional, from, to))
        .collect();

    if level == 0 {
        paths
            .into_iter()
            .map(|sub_paths| sub_paths.into_iter().map(|path| path.len()).min().unwrap())
            .sum()
    } else {
        paths
            .into_iter()
            .map(|sub_paths| {
                sub_paths
                    .into_iter()
                    .map(|path| solve_code(path, true, level - 1))
                    .min()
                    .unwrap()
            })
            .sum()
    }
}

fn calculate_complexity(input: &str, max_level: usize) -> usize {
    input
        .lines()
        .map(|code| {
            let weight: usize = code[..code.len() - 1].parse().unwrap();
            let shortest_path = solve_code(code.chars().collect(), false, max_level);
            shortest_path * weight
        })
        .sum()
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");

    println!("Part 1: {}", calculate_complexity(&input, 2));
    println!("Part 2: {}", calculate_complexity(&input, 25));
}

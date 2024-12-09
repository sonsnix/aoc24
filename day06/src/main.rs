use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::time::Instant;

/// Directions: Left, Up, Right, Down
const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

enum StepResult {
    Moved(i32, i32),
    Turned(usize),
    Left,
}

#[derive(Clone)]
struct State {
    map: HashMap<(i32, i32), char>,
    pos: (i32, i32),
    dir: usize,
}

/// Executes a single step in the simulation.
fn step(state: &State) -> StepResult {
    let (dx, dy) = DIRECTIONS[state.dir];
    let new_pos = (state.pos.0 + dx, state.pos.1 + dy);

    match state.map.get(&new_pos) {
        Some('.') => StepResult::Moved(new_pos.0, new_pos.1),
        Some('#') => StepResult::Turned((state.dir + 1) % DIRECTIONS.len()),
        None => StepResult::Left,
        _ => unreachable!("Unexpected map character."),
    }
}

/// Reads the input map and initializes the simulation state.
fn read_map(input: String) -> State {
    let mut map: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();

    let pos = map
        .iter()
        .find(|(_, &c)| c == '^')
        .map(|(&(x, y), _)| (x, y))
        .expect("Starting position '^' not found in the map");

    if let Some(c) = map.get_mut(&pos) {
        *c = '.'; // Replace '^' with an open path
    }

    State {
        dir: 1, // Initial direction (Up)
        map,
        pos,
    }
}

/// Solves part 1: Counts unique positions visited before exiting.
fn part1(initial_state: &State) -> HashSet<(i32, i32)> {
    let mut state = initial_state.clone();
    let mut visited = HashSet::new();

    loop {
        visited.insert(state.pos);

        match step(&state) {
            StepResult::Moved(x, y) => state.pos = (x, y),
            StepResult::Turned(dir) => state.dir = dir,
            StepResult::Left => break,
        }
    }

    visited
}

/// Solves part 2: Counts grid cells where loops can form.
fn part2(initial_state: State, visited_pos: &HashSet<(i32, i32)>) -> i32 {
    visited_pos
        .par_iter() // Parallelize over the positions
        .filter_map(|&(x, y)| {
            let test_pos = (x as i32, y as i32);

            // Skip the starting position
            if test_pos == initial_state.pos {
                return None;
            }

            let mut state = initial_state.clone(); // Each thread gets its own copy
            let mut visited = HashSet::new();

            if let Some(c) = state.map.get_mut(&test_pos) {
                *c = '#'; // Temporarily modify the map for this test
            }

            loop {
                // Detect loops by checking if the state has been visited
                if !visited.insert((state.pos, state.dir)) {
                    return Some(1); // Found a loop
                }

                match step(&state) {
                    StepResult::Moved(x, y) => state.pos = (x, y),
                    StepResult::Turned(dir) => state.dir = dir,
                    StepResult::Left => return None,
                }
            }
        })
        .sum() // Sum up all the results
}

/// Entry point of the program.
fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let initial_state = read_map(input);

    let start = Instant::now();
    let visited = part1(&initial_state);
    let duration = start.elapsed();
    println!("Part 1: {}, took {duration:?}", visited.len());

    let start = Instant::now();
    let loop_count = part2(initial_state, &visited);
    let duration = start.elapsed();
    println!("Part 1: {loop_count:?}, took {duration:?}");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn read_test_input(file_name: &str) -> State {
        let input = fs::read_to_string(file_name).expect("Failed to read test input file");
        read_map(input)
    }

    #[test]
    fn test_part1() {
        let initial_state = read_test_input("test.txt");
        let result = part1(&initial_state);

        // Adjust this expected value based on your "test.txt" input
        let expected_result: usize = 41;
        assert_eq!(result.len(), expected_result, "Part 1 failed");
    }

    #[test]
    fn test_part2() {
        let initial_state = read_test_input("test.txt");
        let visited = part1(&initial_state);
        let result = part2(initial_state, &visited);

        // Adjust this expected value based on your "test.txt" input
        let expected_result = 6;
        assert_eq!(result, expected_result, "Part 2 failed");
    }
}

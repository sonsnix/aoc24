use std::collections::HashSet;

// Possible moves in the four cardinal directions (right, down, left, up).
const MOVES: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    // Read the input file and unwrap to handle errors.
    let input = std::fs::read_to_string("input.txt").unwrap();

    // Variables to store the start ('S') and end ('E') positions, and wall locations.
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);
    let mut walls = HashSet::new();

    // Parse the input to locate the start, end, and wall positions.
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'S' => start_pos = (x as i32, y as i32),
                'E' => end_pos = (x as i32, y as i32),
                '#' => {
                    walls.insert((x as i32, y as i32));
                }
                _ => {}
            }
        }
    }

    // Compute the path from start to end without passing through walls.
    let mut path = vec![];
    let mut pos = start_pos;

    while pos != end_pos {
        path.push(pos);

        // Try each possible move to find a valid next position.
        for (dx, dy) in MOVES {
            let new_pos = (pos.0 + dx, pos.1 + dy);

            if !walls.contains(&new_pos) && !path.contains(&new_pos) {
                pos = new_pos;
                break;
            }
        }
    }
    path.push(end_pos);

    // Part 1: Count shortcut opportunities with minimal distance criteria.
    let mut part1_count = 0;
    let min_savings = 100;

    for i in 0..path.len() - min_savings - 2 {
        let start_cheat = path[i];
        for j in i + min_savings + 2..path.len() {
            let end_cheat = path[j];
            if manhattan_distance(start_cheat, end_cheat) <= 2 {
                part1_count += 1;
            }
        }
    }

    println!("Part 1: {}", part1_count);

    // Part 2: Count significant shortcuts with savings and distance criteria.
    let mut part2_count = 0;

    for i in 0..path.len() - min_savings {
        let start_cheat = path[i];
        for j in i + min_savings..path.len() {
            let end_cheat = path[j];
            let distance = manhattan_distance(start_cheat, end_cheat);
            let savings = j as i32 - i as i32 - distance;

            if distance <= 20 && savings >= 100 {
                part2_count += 1;
            }
        }
    }

    println!("Part 2: {}", part2_count);
}

/// Calculates the Manhattan distance between two positions.
#[inline]
fn manhattan_distance(pos1: (i32, i32), pos2: (i32, i32)) -> i32 {
    (pos1.0 - pos2.0).abs() + (pos1.1 - pos2.1).abs()
}

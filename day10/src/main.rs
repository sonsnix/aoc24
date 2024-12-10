use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

// Directions for movement: right, down, left, up
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    // Read input from file and parse into a height map
    let input = fs::read_to_string("input.txt").expect("Failed to read input file");
    let map = parse_map(&input);

    // Calculate results for both parts
    let (part_1, part_2) = calculate_trails(&map);

    println!("Part 1: {}, Part 2: {}", part_1, part_2);
}

/// Parses the input string into a height map
fn parse_map(input: &str) -> HashMap<(i32, i32), u32> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(move |(x, c)| {
                ((x as i32, y as i32), c.to_digit(10).expect("Invalid digit in map"))
            })
        })
        .collect()
}

/// Calculates the number of trail ends and total trails
fn calculate_trails(map: &HashMap<(i32, i32), u32>) -> (usize, usize) {
    let mut trail_end_count = 0;
    let mut trail_count = 0;

    for (&trailhead, _) in map.iter().filter(|(_, &height)| height == 0) {
        let (trail_ends, trails) = explore_trails(map, trailhead);
        trail_end_count += trail_ends.len();
        trail_count += trails.len();
    }

    (trail_end_count, trail_count)
}

/// Explores trails starting from a given trailhead
fn explore_trails(
    map: &HashMap<(i32, i32), u32>,
    start: (i32, i32),
) -> (HashSet<(i32, i32)>, HashSet<Vec<(i32, i32)>>) {
    let mut to_visit = VecDeque::from([vec![start]]);
    let mut visited = HashSet::new();
    let mut trails = HashSet::new();
    let mut trail_end_positions = HashSet::new();

    while let Some(trail) = to_visit.pop_front() {
        // Skip already visited trails
        if !visited.insert(trail.clone()) {
            continue;
        }

        let (x, y) = *trail.last().expect("Trail should not be empty");
        let height = *map.get(&(x, y)).expect("Position missing in map");

        if height == 9 {
            // Mark the end of a trail
            trails.insert(trail.clone());
            trail_end_positions.insert((x, y));
            continue;
        }

        // Add neighboring positions with valid height increments
        for &(dx, dy) in &DIRS {
            let new_pos = (x + dx, y + dy);
            if let Some(&new_height) = map.get(&new_pos) {
                if new_height == height + 1 {
                    let mut new_trail = trail.clone();
                    new_trail.push(new_pos);
                    to_visit.push_back(new_trail);
                }
            }
        }
    }

    (trail_end_positions, trails)
}

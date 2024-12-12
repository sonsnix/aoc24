use std::collections::{HashMap, HashSet};

// Directions for moving in the grid: right, down, left, up
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn main() {
    // Read the input file and parse it into a grid
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let grid: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate() // Attach line indices
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate() // Attach character indices
                .map(move |(x, c)| ((x as i32, y as i32), c)) // Map (x, y) -> character
        })
        .collect();

    let mut fenced = HashSet::new(); // Track visited regions
    let mut costs_part1 = 0; // Cost for Part 1
    let mut costs_part2 = 0; // Cost for Part 2

    // Iterate over each cell in the grid
    for (&(x, y), &plant) in &grid {
        // Skip already fenced regions
        if fenced.contains(&(x, y)) {
            continue;
        }

        // Initialize traversal for the current region
        let mut queue = vec![(x, y)];
        let mut area = 0; // Area of the current region
        let mut fences = HashSet::new(); // Fences required for this region

        // Traverse the region using a queue (BFS-like approach)
        while let Some((x, y)) = queue.pop() {
            // Skip if already visited
            if !fenced.insert((x, y)) {
                continue;
            }

            area += 1; // Increase area size

            // Check all 4 directions
            for (dir, &(dx, dy)) in DIRS.iter().enumerate() {
                let new_x = x + dx;
                let new_y = y + dy;

                // If the neighboring cell belongs to the same plant, add it to the queue
                if grid.get(&(new_x, new_y)) == Some(&plant) {
                    queue.push((new_x, new_y));
                } else {
                    // Otherwise, mark the boundary as a fence
                    fences.insert((x, y, dir));
                }
            }
        }

        // Part 1: Add cost based on the area and number of fences
        costs_part1 += area * fences.len();

        // Part 2: Calculate joined fences to reduce redundant fencing
        let mut joined_fences = 0; // Count of shared fences
        let mut analyzed_fences = HashSet::new(); // Track analyzed fences

        for &(x, y, dir) in &fences {
            // Determine perpendicular directions (left and right of the current fence)
            let perpendicular_directions = [(dir + 1) % 4, (dir + 3) % 4];

            // Skip if the fence has already been analyzed
            if !analyzed_fences.insert((x, y, dir)) {
                continue;
            }

            // Check along the perpendicular directions for shared fences
            for &perpendicular_dir in &perpendicular_directions {
                let mut pos = (x + DIRS[perpendicular_dir].0, y + DIRS[perpendicular_dir].1);

                // Traverse along the direction as long as the plant and fence match
                while grid.get(&pos) == Some(&plant) && fences.contains(&(pos.0, pos.1, dir)) {
                    analyzed_fences.insert((pos.0, pos.1, dir));
                    joined_fences += 1;
                    pos = (
                        pos.0 + DIRS[perpendicular_dir].0,
                        pos.1 + DIRS[perpendicular_dir].1,
                    );
                }
            }
        }

        // Part 2: Add cost accounting for joined fences
        costs_part2 += area * (fences.len() - joined_fences);
    }

    // Print results for both parts
    println!("Part 1: {}\nPart 2: {}", costs_part1, costs_part2);
}

use std::collections::{HashMap, HashSet};
use std::fs;

// Function to parse the input
fn parse_input(input: &str) -> (HashMap<i32, HashSet<i32>>, Vec<Vec<i32>>) {
    let sections: Vec<&str> = input.split("\n\n").collect();

    // Parse the ordering rules
    let mut dependencies = HashMap::new();
    for line in sections[0].lines() {
        let parts: Vec<i32> = line.split('|').map(|s| s.parse().unwrap()).collect();
        dependencies
            .entry(parts[1])
            .or_insert_with(HashSet::new)
            .insert(parts[0]);
    }

    // Parse the updates
    let updates: Vec<Vec<i32>> = sections[1]
        .lines()
        .map(|line| line.split(',').map(|s| s.parse().unwrap()).collect())
        .collect();

    (dependencies, updates)
}

// Function to check if an update is in valid order
fn is_valid_order(update: &[i32], dependencies: &HashMap<i32, HashSet<i32>>) -> bool {
    let mut position = HashMap::new();
    for (i, &page) in update.iter().enumerate() {
        position.insert(page, i);
    }

    for (&page, depends_on) in dependencies.iter() {
        if let Some(&page_pos) = position.get(&page) {
            for &dep in depends_on {
                if let Some(&dep_pos) = position.get(&dep) {
                    if dep_pos > page_pos {
                        return false;
                    }
                }
            }
        }
    }

    true
}

// Function to reorder an incorrectly-ordered update
fn reorder_update(update: &[i32], dependencies: &HashMap<i32, HashSet<i32>>) -> Vec<i32> {
    let mut sorted_update = update.to_vec();
    sorted_update.sort_by(|&a, &b| {
        if dependencies.get(&b).map_or(false, |deps| deps.contains(&a)) {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Greater
        }
    });
    sorted_update
}

fn main() {
    // Read input from file
    let input = fs::read_to_string("input.txt")
        .expect("Failed to read input file")
        .replace("\r\n", "\n");
    let (dependencies, updates) = parse_input(&input);

    let mut valid_middle_sum = 0;
    let mut reordered_middle_sum = 0;

    for update in updates {
        if is_valid_order(&update, &dependencies) {
            // Part 1: Sum of middle page numbers for valid updates
            valid_middle_sum += update[update.len() / 2];
        } else {
            // Part 2: Reorder and sum middle page numbers for invalid updates
            let reordered = reorder_update(&update, &dependencies);
            reordered_middle_sum += reordered[reordered.len() / 2];
        }
    }

    // Print results for both parts
    println!("Part 1: {}", valid_middle_sum);
    println!("Part 2: {}", reordered_middle_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "47|53\n97|13\n\n75,47,61,53,29\n97,61,53,29,13";
        let (dependencies, updates) = parse_input(input);

        assert_eq!(dependencies.len(), 2);
        assert!(dependencies.get(&53).unwrap().contains(&47));
        assert!(dependencies.get(&13).unwrap().contains(&97));

        assert_eq!(updates.len(), 2);
        assert_eq!(updates[0], vec![75, 47, 61, 53, 29]);
        assert_eq!(updates[1], vec![97, 61, 53, 29, 13]);
    }

    #[test]
    fn test_is_valid_order() {
        let input = "47|53\n97|13\n\n75,47,61,53,29";
        let (dependencies, updates) = parse_input(input);

        assert!(is_valid_order(&updates[0], &dependencies));

        let invalid_update = vec![75, 53, 47, 29];
        assert!(!is_valid_order(&invalid_update, &dependencies));
    }

    #[test]
    fn test_reorder_update() {
        let input = "47|53\n97|13\n\n75,47,61,53,29";
        let (dependencies, _) = parse_input(input);

        let update = vec![75, 53, 47, 29];
        let reordered = reorder_update(&update, &dependencies);

        assert_eq!(reordered, vec![75, 47, 53, 29]);
    }

    #[test]
    fn test_full_solution() {
        let input = fs::read_to_string("test.txt").expect("Failed to read test file").replace("\r\n", "\n");
        let (dependencies, updates) = parse_input(&input);

        let mut valid_middle_sum = 0;
        let mut reordered_middle_sum = 0;

        for update in updates {
            if is_valid_order(&update, &dependencies) {
                valid_middle_sum += update[update.len() / 2];
            } else {
                let reordered = reorder_update(&update, &dependencies);
                reordered_middle_sum += reordered[reordered.len() / 2];
            }
        }

        assert_eq!(valid_middle_sum, 143); // Replace with the expected result for Part 1
        assert_eq!(reordered_middle_sum, 123); // Replace with the expected result for Part 2
    }
}

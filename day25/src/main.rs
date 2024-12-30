use std::fs;

struct Pattern {
    columns: Vec<usize>,
    is_lock: bool, // Added to store lock/key status
}

fn main() {
    let input = read_input("input.txt");
    let patterns = parse_patterns(&input);
    let (locks, keys) = separate_patterns(patterns);

    let fitting_count = count_fitting_combinations(&locks, &keys);
    println!("Part 1: {}", fitting_count);
}

fn read_input(filename: &str) -> String {
    fs::read_to_string(filename).unwrap().replace("\r\n", "\n")
}

fn parse_patterns(input: &str) -> Vec<Pattern> {
    input.split("\n\n").map(parse_single_pattern).collect()
}

fn parse_single_pattern(block: &str) -> Pattern {
    let lines: Vec<_> = block.lines().collect();
    let width = lines[0].len();
    let mut columns = vec![0; width];

    // Count '#' characters in each column (excluding first and last rows)
    for row in 1..lines.len() - 1 {
        for (col, count) in columns.iter_mut().enumerate() {
            if lines[row].chars().nth(col).unwrap() == '#' {
                *count += 1;
            }
        }
    }

    // Check if it's a lock by looking at first character of first row
    let is_lock = lines[0].starts_with('#');

    Pattern { columns, is_lock }
}

fn separate_patterns(patterns: Vec<Pattern>) -> (Vec<Pattern>, Vec<Pattern>) {
    patterns.into_iter().partition(|pattern| pattern.is_lock)
}

fn count_fitting_combinations(locks: &[Pattern], keys: &[Pattern]) -> usize {
    locks
        .iter()
        .flat_map(|lock| keys.iter().filter(move |key| patterns_fit(lock, key)))
        .count()
}

fn patterns_fit(lock: &Pattern, key: &Pattern) -> bool {
    lock.columns
        .iter()
        .zip(&key.columns)
        .all(|(l, k)| l + k <= 5)
}

use cached::proc_macro::cached;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let mut sections = input.split("\n\n");

    let patterns: Vec<String> = sections.next().unwrap().split(", ").map(String::from).collect();
    let towels: Vec<String> = sections.next().unwrap().lines().map(String::from).collect();

    let result = towels
        .par_iter()
        .fold(
            || (0u64, 0u64),
            |acc, towel| {
                let num_combinations = count_patterns(towel.clone(), patterns.clone());

                if num_combinations > 0 {
                    (acc.0 + 1, acc.1 + num_combinations)
                } else {
                    acc
                }
            },
        )
        .reduce(|| (0, 0), |acc, x| (acc.0 + x.0, acc.1 + x.1));

    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

#[cached]
fn count_patterns(towel: String, patterns: Vec<String>) -> u64 {
    if towel.is_empty() {
        return 1;
    }

    let mut count = 0;

    for pattern in &patterns {
        if towel.starts_with(pattern) {
            let remainder = towel[pattern.len()..].to_string();
            count += count_patterns(remainder, patterns.clone());
        }
    }

    count
}

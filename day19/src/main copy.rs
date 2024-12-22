use std::collections::{HashSet, VecDeque};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let mut sections = input.split("\n\n");

    let patterns: Vec<&str> = sections.next().unwrap().split(", ").collect();
    let towels: Vec<&str> = sections.next().unwrap().lines().collect();

    let mut total = 0;
    let mut count = 0;

    towels.par_iter().fold(0, |acc, towel| {
        let mut queue = VecDeque::from([towel]);
        // let mut visited = HashSet::new();

        let mut num_combinations = 0;

        while let Some(towel) = queue.pop_front() {
            if towel.is_empty() {
                num_combinations += 1;
                continue;
            }

            // if !visited.insert(towel) {
            //     continue;
            // }

            for pattern in patterns.iter() {
                if towel.starts_with(pattern) {
                    let remainder = &towel[pattern.len()..];
                    queue.push_back(remainder);
                }
            }
        }
        total += num_combinations;

        if num_combinations > 0 {
            count += 1;
        }
    });

    println!("Part 1: {}", count);
    println!("Part 2: {}", total);

}

fn count_patterns(towel: &str, patterns: &[&str]) -> u32 {
    let mut count = 0;

    for pattern in patterns.iter() {
        if towel.starts_with(pattern) {
            let remainder = &towel[pattern.len()..];
            count += count_patterns(remainder, patterns);
        }
    }

    count
}
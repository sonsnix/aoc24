use std::fs;

mod part1;

fn main() {
    // Read entire input file
    let input = fs::read_to_string("input.txt").expect("Could not read input file");

    let result = part1::solve_warehouse_puzzle(&input);
    println!("Part 1: {}", result);
}

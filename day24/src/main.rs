use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut graph = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split("-");
        let computer_a = parts.next().unwrap();
        let computer_b = parts.next().unwrap();

        graph
            .entry(computer_a)
            .or_insert(HashSet::new())
            .insert(computer_b);

        graph
            .entry(computer_b)
            .or_insert(HashSet::new())
            .insert(computer_a);
    }

    
}

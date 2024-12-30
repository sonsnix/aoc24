use std::collections::{HashMap, HashSet};

use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .replace("\r\n", "\n");

    let mut split = input.split("\n\n");
    let initial_values = split.next().unwrap();
    let logics = split.next().unwrap();

    let mut wires: HashMap<String, i64> = HashMap::new();
    let mut logic = vec![];

    for line in initial_values.lines() {
        let (wire, value) = line.split_once(": ").unwrap();
        wires.insert(wire.to_string(), value.parse().unwrap());
    }

    for line in logics.lines() {
        let (lhs, rhs) = line.split_once(" -> ").unwrap();

        let mut split = lhs.split(' ');
        let wire_a = split.next().unwrap();
        let operation = split.next().unwrap();
        let wire_b = split.next().unwrap();

        let wire_c = rhs;

        logic.push((
            wire_a.to_string(),
            wire_b.to_string(),
            operation.to_string(),
            wire_c.to_string(),
        ));
    }

    let initial_wires = wires.clone();
    let initial_logic = logic.clone();

    propagate_wires(&mut wires, &logic);

    let z: i64 = calculate_value(&wires, "z");

    println!("Part 1: {}", z);

    let x = calculate_value(&wires, "x");
    let y = calculate_value(&wires, "y");

    let z_correct = x + y;
    let incorrect_outputs: Vec<_> = format!("{:b}", z ^ z_correct)
        .chars()
        .rev()
        .enumerate()
        .filter(|(_, c)| *c == '1')
        .map(|(i, _)| format!("z{:02}", i))
        .collect();

    let mut swap_wires = HashSet::new();
    let mut queue = Vec::from(incorrect_outputs);

    while let Some(wire) = queue.pop() {
        if wire.starts_with("x") || wire.starts_with("y") || !swap_wires.insert(wire.clone()) {
            continue;
        }

        for (wire_a, wire_b, _, wire_c) in logic.iter() {
            if wire_c != &wire {
                continue;
            }

            queue.push(wire_a.to_string());
            queue.push(wire_b.to_string());
        }
    }

    let pairs = swap_wires.iter().combinations(2);

    let mut count = 0;
    for combination in pairs.combinations(4) {
        count += 1;
    }
    println!("Part 2: {}", count);
}

fn propagate_wires(
    wires: &mut HashMap<String, i64>,
    logic: &Vec<(String, String, String, String)>,
) {
    let mut change = true;

    while change {
        change = false;

        for (wire_a, wire_b, operation, wire_c) in logic.iter() {
            if !wires.contains_key(wire_c)
                && wires.contains_key(wire_a)
                && wires.contains_key(wire_b)
            {
                let a = wires[wire_a];
                let b = wires[wire_b];

                wires.insert(
                    wire_c.clone(),
                    match operation.as_str() {
                        "AND" => a & b,
                        "OR" => a | b,
                        "XOR" => a ^ b,
                        _ => panic!("Unknown operation: {operation}"),
                    },
                );

                change = true;
            }
        }
    }
}

fn calculate_value(wires: &HashMap<String, i64>, starts_with: &str) -> i64 {
    wires
        .iter()
        .filter(|(k, _)| k.starts_with(starts_with))
        .sorted()
        .enumerate()
        .map(|(i, (_, v))| v * 2_i64.pow(i as u32))
        .sum()
}

use std::{
    collections::{HashMap, HashSet},
    fs,
};

use num::integer::gcd;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");

    let map: HashMap<(i32, i32), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect();

    let mut anti_nodes = HashSet::new();

    for (pos1, frequency) in map.iter().filter(|(_, &c)| c != '.') {
        for (pos2, _) in map
            .iter()
            .filter(|(pos2, &c)| c == *frequency && pos1 != *pos2)
        {
            let dir = (pos2.0 - pos1.0, pos2.1 - pos1.1);

            let anti_node = (pos2.0 + dir.0, pos2.1 + dir.1);

            if map.contains_key(&anti_node) {
                anti_nodes.insert(anti_node);
            }

            let anti_node = (pos1.0 - dir.0, pos1.1 - dir.1);

            if map.contains_key(&anti_node) {
                anti_nodes.insert(anti_node);
            }
        }
    }

    println!("Part 1: {}", anti_nodes.len());

    let mut anti_nodes = HashSet::new();

    for (pos1, frequency) in map.iter().filter(|(_, &c)| c != '.') {
        for (pos2, _) in map
            .iter()
            .filter(|(pos2, &c)| c == *frequency && pos1 != *pos2)
        {
            // calculate direction vector
            let dir = (pos2.0 - pos1.0, pos2.1 - pos1.1);

            // reduce to smallest integer vector
            let divisor = gcd(dir.0.abs(), dir.1.abs());
            let dir = (dir.0 / divisor, dir.1 / divisor);

            let mut pos = *pos1;

            while map.contains_key(&pos) {
                anti_nodes.insert(pos);
                pos = (pos.0 + dir.0, pos.1 + dir.1);
            }

            let mut pos = (pos1.0 - dir.0, pos1.1 - dir.1);

            while map.contains_key(&pos) {
                anti_nodes.insert(pos);
                pos = (pos.0 - dir.0, pos.1 - dir.1);
            }
        }
    }

    println!("Part 2: {}", anti_nodes.len());
}

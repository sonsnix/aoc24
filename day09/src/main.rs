use std::fs;

const EMPTY: u64 = u64::MAX;

fn main() {
    let input = fs::read_to_string("input2.txt").expect("Unable to read file");
    let mut disk = vec![];
    let max_id = (input.len() - 1) as u64 / 2;

    for (i, c) in input.chars().enumerate() {
        let size = c.to_digit(10).unwrap();

        for _ in 0..size {
            if i % 2 == 0 {
                disk.push((i as u64) / 2);
            } else {
                disk.push(EMPTY);
            }
        }
    }

    let initial_disk = disk.clone();

    // println!("{:?}", disk);

    loop {
        let last_file_pos: usize = disk.iter().rposition(|n| *n != EMPTY).unwrap();

        let first_empty_pos = disk.iter().position(|n| *n == EMPTY).unwrap();

        if first_empty_pos < last_file_pos {
            disk[first_empty_pos] = disk[last_file_pos];
            disk[last_file_pos] = EMPTY;
        } else {
            break;
        }
    }
    // println!("{:?}", disk);

    let sum: u64 = disk
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| match c {
            EMPTY => None,
            _ => Some((i as u64) * c),
        })
        .sum();

    println!("Part 1: {}", sum);

    let mut disk = initial_disk.clone();

    for id in (0..=max_id).rev() {
        let file_pos: usize = disk.iter().position(|n| *n == id).unwrap();

        let mut size = 1;

        while disk.get(file_pos + size) == Some(&id) {
            size += 1;
        }

        // find leftmost empty spot which is large enough
        let mut empty_pos = disk.iter().position(|n| *n == EMPTY).unwrap();

        loop {
            let mut gap_size = 1;

            while disk.get(empty_pos + gap_size) == Some(&EMPTY) {
                gap_size += 1;
            }

            if empty_pos >= file_pos {
                break;
            }

            if gap_size >= size {
                for i in 0..size {
                    disk[empty_pos + i] = id;
                    disk[file_pos + i] = EMPTY;
                }
                break;
            } else {
                empty_pos += gap_size;
                while disk.get(empty_pos) != Some(&EMPTY) {
                    empty_pos += 1;
                }
            }
        }
    }

    let sum: u64 = disk
        .iter()
        .enumerate()
        .filter_map(|(i, &c)| match c {
            EMPTY => None,
            _ => Some((i as u64) * c),
        })
        .sum();

    println!("Part 2: {}", sum);
}

use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

const PROD_SIZE: (i32, i32) = (71, 71);
const PROD_TIME: usize = 1024;

struct Grid {
    bytes: Vec<(i32, i32)>,
    width: i32,
    height: i32,
    start: (i32, i32),
    end: (i32, i32),
}

impl Grid {
    fn from(input: &str, width: i32, height: i32) -> Self {
        let bytes: Vec<(i32, i32)> = input
            .lines()
            .filter_map(|line| line.split(',').map(|n| n.parse().unwrap()).collect_tuple())
            .collect();

        let start = (0, 0);
        let end = (width - 1, height - 1);

        Self {
            bytes,
            width,
            height,
            start,
            end,
        }
    }

    fn find_shortest_path_after_bytes(&self, num_bytes: usize) -> i32 {
        let walls: HashSet<&(i32, i32)> = self.bytes.iter().take(num_bytes).collect();

        let mut queue = VecDeque::from([(self.start, 0)]);
        let mut visited = HashMap::new();

        let mut min_steps = i32::MAX;

        while let Some((pos, steps)) = queue.pop_front() {
            if pos == self.end {
                if steps < min_steps {
                    min_steps = steps;
                }
                continue;
            }

            if let Some(prev_steps) = visited.get(&pos) {
                if steps >= *prev_steps {
                    continue;
                }
            } else {
                visited.insert(pos, steps);
            }

            for dir in DIRS {
                let new_pos = (pos.0 + dir.0, pos.1 + dir.1);

                if new_pos.0 < 0
                    || new_pos.0 >= self.width
                    || new_pos.1 < 0
                    || new_pos.1 >= self.height
                    || walls.contains(&new_pos)
                {
                    continue;
                }

                queue.push_back((new_pos, steps + 1));
            }
        }
        min_steps
    }

    fn find_first_blocking_byte(&self, start: Option<usize>) -> Option<(i32, i32)> {
        let range = start.unwrap_or(1)..=self.bytes.len();

        let blocking_byte = range
            .into_par_iter()
            .find_first(|&num_bytes| self.find_shortest_path_after_bytes(num_bytes) == i32::MAX);

        match blocking_byte {
            Some(num_bytes) => Some(self.bytes[num_bytes - 1]),
            None => None,
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let grid = Grid::from(&input, PROD_SIZE.0, PROD_SIZE.1);

    println!("Part 1: {}", grid.find_shortest_path_after_bytes(PROD_TIME));
    println!(
        "Part 2: {:?}",
        grid.find_first_blocking_byte(Some(PROD_TIME)).unwrap()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("test.txt").unwrap();

        const TEST_SIZE: (i32, i32) = (7, 7);
        const TEST_TIME: usize = 12;

        let grid = Grid::from(&input, TEST_SIZE.0, TEST_SIZE.1);

        assert_eq!(grid.find_shortest_path_after_bytes(TEST_TIME), 22);
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("test.txt").unwrap();

        const TEST_SIZE: (i32, i32) = (7, 7);
        const TEST_TIME: usize = 12;

        let grid = Grid::from(&input, TEST_SIZE.0, TEST_SIZE.1);

        assert_eq!(grid.find_first_blocking_byte(Some(TEST_TIME)), Some((6, 1)));
    }
}

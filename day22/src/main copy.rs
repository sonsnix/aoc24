use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

struct DiffPatternIterator {
    current: (i64, i64, i64, i64),
    done: bool,
}

impl DiffPatternIterator {
    fn new() -> Self {
        Self {
            current: (-9, 0, 0, 0),
            done: false,
        }
    }

    fn is_valid_pattern(&self) -> bool {
        for start in 0..=9 {
            let mut valid = true;
            let mut num = start;

            for diff in [
                self.current.0,
                self.current.1,
                self.current.2,
                self.current.3,
            ] {
                let next = num + diff;
                if next < 0 || next > 9 {
                    valid = false;
                    break;
                }
                num = next;
            }

            if valid {
                return true;
            }
        }
        false
    }
}

impl Iterator for DiffPatternIterator {
    type Item = (i64, i64, i64, i64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        while !self.is_valid_pattern() {
            let mut arr = [
                self.current.0,
                self.current.1,
                self.current.2,
                self.current.3,
            ];
            let mut i = arr.len() - 1;
            loop {
                arr[i] += 1;
                if arr[i] > 9 {
                    arr[i] = -9;
                    if i == 0 {
                        self.done = true;
                        return None;
                    }
                    i -= 1;
                } else {
                    break;
                }
            }
            self.current = (arr[0], arr[1], arr[2], arr[3]);
        }

        let result = self.current;

        if result == (9, 0, 0, 0) {
            self.done = true;
        } else {
            let mut arr = [
                self.current.0,
                self.current.1,
                self.current.2,
                self.current.3,
            ];
            let mut i = arr.len() - 1;
            loop {
                arr[i] += 1;
                if arr[i] > 9 {
                    arr[i] = -9;
                    i -= 1;
                } else {
                    break;
                }
            }
            self.current = (arr[0], arr[1], arr[2], arr[3]);
        }

        Some(result)
    }
}

struct SecretDifferenceIterator {
    secret: u64,
    remaining: usize,
}

impl SecretDifferenceIterator {
    fn new(secret: u64) -> Self {
        Self {
            secret,
            remaining: 1999,
        }
    }
}

impl Iterator for SecretDifferenceIterator {
    type Item = (i64, u64);

    fn next(&mut self) -> Option<(i64, u64)> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            let new_secret = evolve(self.secret);
            let diff = (new_secret % 10) as i64 - (self.secret % 10) as i64;
            self.secret = new_secret;
            Some((diff, new_secret % 10))
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .map(|initial_secret| {
            let mut secret = initial_secret;
            for _ in 0..2000 {
                secret = evolve(secret);
            }
            secret
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    let patterns: Vec<_> = DiffPatternIterator::new().collect();

    patterns
        .par_iter()
        .map(|pattern| {
            let mut total_bananas = 0;

            for line in input.lines() {
                let initial_secret = line.parse::<u64>().unwrap();
                let secrets = SecretDifferenceIterator::new(initial_secret);

                if let Some(pos) =
                    secrets
                        .tuple_windows()
                        .find(|x: &((i64, _), (i64, _), (i64, _), (i64, _))| {
                            (x.0 .0, x.1 .0, x.2 .0, x.3 .0) == *pattern
                        })
                {
                    total_bananas += pos.3 .1;
                }
            }

            total_bananas
        })
        .max()
        .unwrap()
}

fn evolve(secret: u64) -> u64 {
    let mut new_secret = prune(mix(secret, secret * 64));
    new_secret = prune(mix(new_secret, new_secret / 32));
    new_secret = prune(mix(new_secret, new_secret * 2048));
    new_secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

fn mix(secret: u64, value: u64) -> u64 {
    secret ^ value
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_mix() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn test_prune() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn test_evolve() {
        let mut secret = 123;

        secret = evolve(secret);
        assert_eq!(secret, 15887950);

        secret = evolve(secret);
        assert_eq!(secret, 16495136);
    }

    #[test]
    fn test_evolve_file() {
        let input = std::fs::read_to_string("test.txt").unwrap();

        let results = HashMap::from([
            (1, 8685429),
            (10, 4700978),
            (100, 15273692),
            (2024, 8667524),
        ]);

        for line in input.lines() {
            let initial_secret = line.parse::<u64>().unwrap();

            let mut secret = initial_secret;

            for _ in 0..2000 {
                secret = evolve(secret);
            }

            assert_eq!(secret, results[&initial_secret]);
        }
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("test.txt").unwrap();
        assert_eq!(part1(&input), 37327623);
    }
}

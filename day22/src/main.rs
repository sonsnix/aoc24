use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::error::Error;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Clone, Copy)]
struct DiffPattern(i64, i64, i64, i64);

impl DiffPattern {
    const MIN_DIFF: i64 = -9;
    const MAX_DIFF: i64 = 9;

    fn is_valid(&self) -> bool {
        (0..=9).any(|start| {
            let mut num = start;
            [self.0, self.1, self.2, self.3]
                .iter()
                .all(|&diff| {
                    num = num + diff;
                    (0..=9).contains(&num)
                })
        })
    }
}

#[derive(Debug)]
struct DiffPatternIterator {
    current: DiffPattern,
    done: bool,
}

impl DiffPatternIterator {
    fn new() -> Self {
        Self {
            current: DiffPattern(DiffPattern::MIN_DIFF, 0, 0, 0),
            done: false,
        }
    }

    fn increment_pattern(&mut self) {
        let mut values = [self.current.0, self.current.1, self.current.2, self.current.3];
        
        for i in (0..values.len()).rev() {
            values[i] += 1;
            if values[i] <= DiffPattern::MAX_DIFF {
                break;
            }
            values[i] = DiffPattern::MIN_DIFF;
            if i == 0 {
                self.done = true;
                return;
            }
        }
        self.current = DiffPattern(values[0], values[1], values[2], values[3]);
    }
}

impl Iterator for DiffPatternIterator {
    type Item = DiffPattern;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        while !self.current.is_valid() {
            self.increment_pattern();
            if self.done {
                return None;
            }
        }

        let result = self.current;
        self.increment_pattern();
        Some(result)
    }
}

#[derive(Debug)]
struct SecretEvolution {
    secret: u64,
    remaining: usize,
}

impl SecretEvolution {
    const MAX_ITERATIONS: usize = 2000;
    const MODULUS: u64 = 16777216; // 2^24

    fn new(secret: u64) -> Self {
        Self {
            secret,
            remaining: Self::MAX_ITERATIONS,
        }
    }

    fn evolve(mut secret: u64) -> u64 {
        secret = Self::prune(Self::mix(secret, secret * 64));
        secret = Self::prune(Self::mix(secret, secret / 32));
        Self::prune(Self::mix(secret, secret * 2048))
    }

    #[inline]
    fn prune(secret: u64) -> u64 {
        secret % Self::MODULUS
    }

    #[inline]
    fn mix(secret: u64, value: u64) -> u64 {
        secret ^ value
    }
}

impl Iterator for SecretEvolution {
    type Item = (i64, u64);

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        self.remaining -= 1;
        let new_secret = Self::evolve(self.secret);
        let diff = (new_secret % 10) as i64 - (self.secret % 10) as i64;
        self.secret = new_secret;
        Some((diff, new_secret % 10))
    }
}

fn process_input(input: &str) -> Result<Vec<u64>> {
    input
        .lines()
        .map(|line| line.parse().map_err(Into::into))
        .collect()
}

fn part1(input: &str) -> Result<u64> {
    let secrets = process_input(input)?;
    Ok(secrets
        .into_par_iter()
        .map(|secret| {
            (0..SecretEvolution::MAX_ITERATIONS)
                .fold(secret, |s, _| SecretEvolution::evolve(s))
        })
        .sum())
}

fn part2(input: &str) -> Result<u64> {
    let secrets = process_input(input)?;
    let patterns: Vec<_> = DiffPatternIterator::new().collect();

    Ok(patterns
        .par_iter()
        .map(|&pattern| {
            secrets
                .iter()
                .filter_map(|&initial_secret| {
                    SecretEvolution::new(initial_secret)
                        .tuple_windows()
                        .find(|&x: &((i64, u64), (i64, u64), (i64, u64), (i64, u64))| {
                            (x.0.0, x.1.0, x.2.0, x.3.0) == 
                                (pattern.0, pattern.1, pattern.2, pattern.3)
                        })
                        .map(|x| x.3.1)
                })
                .sum()
        })
        .max()
        .unwrap_or(0))
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;
    println!("Part 1: {}", part1(&input)?);
    println!("Part 2: {}", part2(&input)?);
    Ok(())
}
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

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
            remaining: Self::MAX_ITERATIONS + 1,
        }
    }

    fn evolve(&mut self) {
        self.mix(self.secret * 64);
        self.prune();

        self.mix(self.secret / 32);
        self.prune();

        self.mix(self.secret * 2048);
        self.prune();
    }

    #[inline]
    fn prune(&mut self) {
        self.secret %= Self::MODULUS
    }

    #[inline]
    fn mix(&mut self, value: u64) {
        self.secret ^= value
    }
}

impl Iterator for SecretEvolution {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }

        self.remaining -= 1;
        let previous_secret = self.secret;
        self.evolve();

        Some(previous_secret)
    }
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input.txt")?;

    let mut patterns = HashMap::new();
    let mut sum_at_last_step = 0;

    for initial_secret in input.lines().map(|x| x.parse().unwrap()) {
        let secrets: Vec<_> = SecretEvolution::new(initial_secret).collect();
        let diffs: Vec<_> = secrets
            .iter()
            .tuple_windows()
            .map(|x: (&u64, &u64)| (x.1 % 10) as i64 - (x.0 % 10) as i64)
            .collect();

        sum_at_last_step += secrets.last().unwrap();

        let mut seen = HashSet::new();
        for i in 0..diffs.len() - 4 {
            let pattern = (diffs[i], diffs[i + 1], diffs[i + 2], diffs[i + 3]);
            if !seen.insert(pattern) {
                // only count patterns the first time we see them
                continue;
            }
            *patterns.entry(pattern).or_insert(0) += secrets[i + 4] % 10;
        }
    }

    println!("Part 1: {}", sum_at_last_step);
    println!("Part 2: {}", patterns.values().max().unwrap());

    Ok(())
}

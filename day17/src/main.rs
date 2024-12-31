use itertools::Itertools;

#[derive(Debug, Clone)]
struct Computer {
    registers: Vec<u64>,
    program: Vec<u64>,
    pos: usize,
}

impl Computer {
    fn from_input(input: &str) -> Self {
        let numbers: Vec<u64> = input
            .split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();

        Self {
            registers: numbers[0..3].to_vec(),
            program: numbers[3..].to_vec(),
            pos: 0,
        }
    }

    fn operand(&self, operand: u64) -> u64 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand as usize - 4],
            7 => unreachable!("Reserved, not implemented."),
            _ => panic!("Invalid operand: {operand}"),
        }
    }

    fn run(&mut self) -> Vec<u64> {
        let mut output = Vec::new();

        while self.pos + 1 < self.program.len() {
            let (instruction, operand) = (self.program[self.pos], self.program[self.pos + 1]);
            let combo = self.operand(operand);

            match instruction {
                0 => self.registers[0] = self.registers[0] >> combo,
                1 => self.registers[1] ^= operand,
                2 => self.registers[1] = combo % 8,
                3 => if self.registers[0] != 0 {
                    self.pos = operand as usize;
                    continue;
                },
                4 => self.registers[1] ^= self.registers[2],
                5 => output.push(combo % 8),
                6 => self.registers[1] = self.registers[0] >> combo,
                7 => self.registers[2] = self.registers[0] >> combo,
                _ => panic!("Invalid instruction: {instruction}"),
            }

            self.pos += 2;
        }
        output
    }
}

fn find_target_sequence(program: &[u64]) -> Option<u64> {
    let mut possible_values = vec![0u64];
    
    // Work backwards through the output sequence
    for target in program.iter().rev() {
        let next_values = possible_values.iter()
            .flat_map(|&current| {
                (0..8).filter_map(move |bits| {
                    let new_value = (current << 3) | bits;
                    verify_step(new_value, *target).map(|_| new_value)
                })
            })
            .collect();
            
        possible_values = next_values;
        
        if possible_values.is_empty() {
            return None;
        }
    }
    
    possible_values.into_iter().min()
}

fn verify_step(value: u64, target: u64) -> Option<u64> {
    let original = value % 8;
    let after_xor5 = original ^ 5;
    let shifted = value >> after_xor5;
    let after_xor6 = after_xor5 ^ 6;
    
    if (after_xor6 ^ shifted) % 8 == target {
        Some(original)
    } else {
        None
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let computer = Computer::from_input(&input);
    
    let mut part1 = computer.clone();
    println!("Part 1: {}", part1.run().iter().join(","));
    
    let initial_value = find_target_sequence(&computer.program).unwrap();
    println!("Part 2: {initial_value}");
}
use core::panic;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Computer {
    registers: Vec<u32>,
    program: Vec<u32>,
    pos: usize,
}

impl Computer {
    fn from_input(input: &str) -> Computer {
        let numbers: Vec<u32> = input
            .split(|c: char| !c.is_numeric()) // Split on any non-numeric character
            .filter_map(|s| s.parse::<u32>().ok()) // Try to parse each part as a number
            .collect();

        Self {
            registers: numbers[0..3].to_vec(),
            program: numbers[3..].to_vec(),
            pos: 0,
        }
    }

    fn operand(&self, operand: u32) -> u32 {
        match operand {
            0..=3 => operand,
            4..=6 => self.registers[operand as usize - 4],
            7 => unreachable!("Reserved, not implemented."),
            _ => panic!("Invalid operand: {}", operand),
        }
    }

    fn run(&mut self) -> Vec<u32> {
        let mut output = vec![];

        while self.pos + 1 < self.program.len() {
            let (instruction, operand) = (self.program[self.pos], self.program[self.pos + 1]);
            let combo = self.operand(operand);
            let mut jumped = false;

            match instruction {
                0 => self.registers[0] = self.registers[0] / 2u32.pow(combo),
                1 => self.registers[1] = self.registers[1] ^ operand,
                2 => self.registers[1] = combo % 8,
                3 => {
                    if self.registers[0] != 0 {
                        self.pos = operand as usize;
                        jumped = true;
                    }
                }
                4 => self.registers[1] = self.registers[1] ^ self.registers[2],
                5 => output.push(combo % 8),
                6 => self.registers[1] = self.registers[0] / 2u32.pow(combo),
                7 => self.registers[2] = self.registers[0] / 2u32.pow(combo),
                _ => panic!("Invalid instruction!"),
            }

            if !jumped {
                self.pos += 2;
            }
        }
        output
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut computer = Computer::from_input(&input);
    println!("{:?}", computer);

    println!("Part 1: {}", computer.run().iter().join(","));

}

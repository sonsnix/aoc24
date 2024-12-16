use regex::Regex;
use std::error::Error;
use std::fs;

const OFFSET: i128 = 10_000_000_000_000;

#[derive(Debug, Clone)]
struct ClawMachine {
    a_1: i128,
    a_2: i128,
    b_1: i128,
    b_2: i128,
    c_1: i128,
    c_2: i128,
}

fn parse_machines(input: &str) -> Result<Vec<ClawMachine>, Box<dyn Error>> {
    let machine_regex = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\r?\nButton B: X\+(\d+), Y\+(\d+)\r?\nPrize: X=(\d+), Y=(\d+)",
    )?;

    let machines: Vec<ClawMachine> = machine_regex
        .captures_iter(input)
        .map(|cap| {
            Ok(ClawMachine {
                a_1: cap[1].parse()?,
                a_2: cap[2].parse()?,
                b_1: cap[3].parse()?,
                b_2: cap[4].parse()?,
                c_1: cap[5].parse()?,
                c_2: cap[6].parse()?,
            })
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()?;

    Ok(machines)
}

fn solve_equation(machine: &ClawMachine, offset: i128) -> Option<(i128, i128, i128)> {
    let (a_1, a_2) = (machine.a_1, machine.a_2);
    let (b_1, b_2) = (machine.b_1, machine.b_2);
    let (c_1, c_2) = (machine.c_1, machine.c_2);

    let denominator = a_1 * b_2 - b_1 * a_2;
    if denominator == 0 {
        return None; // No solution if the determinant is zero
    }

    let x = (c_1 * b_2 - b_1 * c_2 + (b_2 - b_1) * offset) / denominator;
    let y = (a_1 * c_2 - c_1 * a_2 + (a_1 - a_2) * offset) / denominator;
    let tokens = 3 * x + y;

    Some((x, y, tokens))
}

fn solve_claw_machines(machines: &[ClawMachine], offset: i128) -> i128 {
    machines
        .iter()
        .filter_map(|machine| solve_equation(machine, offset))
        .filter(|&(x, y, _)| x >= 0 && y >= 0 && (offset != 0 || (x <= 100 && y <= 100)))
        .filter(|&(x, y, _)| {
            machines.iter().any(|m| {
                x * m.a_1 + y * m.b_1 == m.c_1 + offset &&
                x * m.a_2 + y * m.b_2 == m.c_2 + offset
            })
        })
        .map(|(_, _, tokens)| tokens)
        .sum()
}

fn main() -> Result<(), Box<dyn Error>> {
    // Read input file
    let input = fs::read_to_string("input.txt")?;

    // Parse machines from input
    let machines = parse_machines(&input)?;

    // Solve Part 1
    let part1_tokens = solve_claw_machines(&machines, 0);
    println!("Part 1: {}", part1_tokens);

    // Solve Part 2
    let part2_tokens = solve_claw_machines(&machines, OFFSET);
    println!("Part 2: {}", part2_tokens);

    Ok(())
}

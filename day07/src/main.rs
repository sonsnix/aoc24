use itertools::{repeat_n, Itertools};

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Mul,
    Cat,
}

impl Operator {
    fn apply(self, acc: i64, x: i64) -> i64 {
        match self {
            Operator::Add => acc + x,
            Operator::Mul => acc * x,
            Operator::Cat => format!("{}{}", acc, x)
                .parse()
                .expect("Failed to parse concatenated value"),
        }
    }
}

/// Reads equations from the input string.
/// Each line should have the format: `<test_value>: <numbers>`
/// Example: `42: 1 2 3`
fn read_equations(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .map(|line| {
            let (test_value, numbers) = line
                .split_once(": ")
                .expect("Each line must contain a ': ' separator");
            let test_value = test_value.parse().expect("Invalid test value");
            let numbers = numbers
                .split_whitespace()
                .map(|x| x.parse().expect("Invalid number"))
                .collect();
            (test_value, numbers)
        })
        .collect()
}

/// Computes the sum of test values for which the equations satisfy the condition.
fn compute_sum(equations: &[(i64, Vec<i64>)], operators: &[Operator]) -> i64 {
    equations
        .iter()
        .filter_map(|(test_value, numbers)| {
            if satisfies_equation(*test_value, numbers, operators) {
                Some(test_value)
            } else {
                None
            }
        })
        .sum()
}

/// Checks if a given test value can be achieved using the provided operators.
fn satisfies_equation(test_value: i64, numbers: &[i64], operators: &[Operator]) -> bool {
    repeat_n(operators, numbers.len() - 1)
        .multi_cartesian_product()
        .any(|operator_sequence| {
            let result = numbers[1..]
                .iter()
                .zip(operator_sequence.iter())
                .fold(numbers[0], |acc, (&x, &operator)| operator.apply(acc, x));
            result == test_value
        })
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("Failed to read input file");
    let equations = read_equations(&input);

    // Part 1: Using Add and Mul operators
    let operators_part1 = vec![Operator::Add, Operator::Mul];
    let sum_part1 = compute_sum(&equations, &operators_part1);
    println!("part1: {}", sum_part1);

    // Part 2: Including the Cat operator
    let operators_part2 = vec![Operator::Add, Operator::Mul, Operator::Cat];
    let sum_part2 = compute_sum(&equations, &operators_part2);
    println!("part2: {}", sum_part2);
}

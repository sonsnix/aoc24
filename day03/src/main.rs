use regex::Regex;
use std::fs;

fn main() {
    let filename = "input.txt";
    let file_content = fs::read_to_string(filename).expect("Failed to read the input file");

    let total_product = calculate_sum(&file_content, true);
    println!("Part 1: {}", total_product);

    let total_product = calculate_sum(&file_content, false);
    println!("Part 2: {}", total_product);
}

fn calculate_sum(instructions: &str, ignore_do_instruction: bool) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut is_multiplication_enabled = true;
    let sum: i32 = re
        .captures_iter(instructions)
        .map(|capture| match capture.get(0).unwrap().as_str() {
            "do()" => {
                is_multiplication_enabled = true;
                0
            }
            "don't()" => {
                is_multiplication_enabled = false;
                0
            }
            _ => {
                let num1: i32 = capture.get(1).unwrap().as_str().parse().unwrap();
                let num2: i32 = capture.get(2).unwrap().as_str().parse().unwrap();
                if is_multiplication_enabled || ignore_do_instruction {
                    num1 * num2
                } else {
                    0
                }
            }
        })
        .sum();
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let instructions =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

        assert_eq!(calculate_sum(instructions, true), 161);
    }

    #[test]
    fn test_part2() {
        let instructions =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

        assert_eq!(calculate_sum(instructions, false), 48);
    }
}

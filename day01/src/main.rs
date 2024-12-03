use std::fs;

fn read_and_parse_file(filename: &str) -> (Vec<i32>, Vec<i32>) {
    let contents = fs::read_to_string(filename).expect("Couldn't read input file.");

    let mut first_column = Vec::new();
    let mut second_column = Vec::new();

    for line in contents.lines() {
        let numbers: Vec<i32> = line
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect();

        first_column.push(numbers[0]);
        second_column.push(numbers[1]);
    }

    first_column.sort();
    second_column.sort();

    (first_column, second_column)
}

fn calculate_difference(first_column: &[i32], second_column: &[i32]) -> i32 {
    first_column
        .iter()
        .zip(second_column.iter())
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum()
}

fn calculate_weighted_sum(first_column: &[i32], second_column: &[i32]) -> i32 {
    first_column
        .iter()
        .map(|num1| num1 * second_column.iter().filter(|num2| num1 == *num2).count() as i32)
        .sum()
}

fn main() {
    let (first_column, second_column) = read_and_parse_file("input.txt");

    let difference = calculate_difference(&first_column, &second_column);

    println!("Part 1: {}", difference);

    let weighted_sum = calculate_weighted_sum(&first_column, &second_column);

    println!("Part 2: {}", weighted_sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (first_column, second_column) = read_and_parse_file("test.txt");

        let difference = calculate_difference(&first_column, &second_column);

        assert_eq!(difference, 11);
    }

    #[test]
    fn test_part2() {
        let (first_column, second_column) = read_and_parse_file("test.txt");

        let weighted_sum = calculate_weighted_sum(&first_column, &second_column);

        assert_eq!(weighted_sum, 31);
    }
}

use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Couldn't read input file.");

    let mut arr1 = vec![];
    let mut arr2 = vec![];

    for line in contents.lines() {
        let nums: Vec<_> = line
            .split_whitespace()
            .map(|num| num.parse::<i32>().unwrap())
            .collect();

        arr1.push(nums[0]);
        arr2.push(nums[1]);
    }

    arr1.sort();
    arr2.sort();

    let res: i32 = arr1
        .iter()
        .zip(arr2.iter())
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum();

    println!("Result: {}", res);

    let res: i32 = arr1
        .iter()
        .map(|num1| num1 * arr2.iter().filter(|&num2| num1 == num2).count() as i32)
        .sum();

    println!("Result: {}", res);
}

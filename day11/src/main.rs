use cached::proc_macro::cached;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let numbers: Vec<u64> = input.split(" ").map(|s| s.parse().unwrap()).collect();

    let part1 = numbers
        .iter()
        .fold(0, |count, &number| count + blink(number, 25));

    let part2 = numbers
        .iter()
        .fold(0, |count, &number| count + blink(number, 75));

    println!("Part 1: {}\nPart 2: {}", part1, part2);
}

#[cached]
fn blink(number: u64, remaining_blinks: u64) -> u64 {
    if remaining_blinks == 0 {
        return 1;
    }

    if number == 0 {
        blink(1, remaining_blinks - 1)
    } else if number.to_string().len() % 2 == 0 {
        let str_val = number.to_string();
        let left: u64 = str_val[0..str_val.len() / 2].parse().unwrap();
        let right: u64 = str_val[str_val.len() / 2..str_val.len()].parse().unwrap();

        blink(left, remaining_blinks - 1) + blink(right, remaining_blinks - 1)
    } else {
        blink(number * 2024, remaining_blinks - 1)
    }
}

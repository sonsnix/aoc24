use std::fs;

fn main() {
    let filename = "input.txt";
    let file_content = fs::read_to_string(filename).expect("Failed to read the input file");

    let grid = file_content
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let pattern = "XMAS";
    let dirs = [
        (-1, 0),
        (1, 0),
        (0, -1),
        (0, 1),
        (-1, -1),
        (-1, 1),
        (1, -1),
        (1, 1),
    ];

    let mut count = 0;
    let pattern_len = pattern.chars().count() as i32;

    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            for dir in dirs.iter() {
                if y as i32 + dir.0 * (pattern_len - 1) < 0
                    || y as i32 + dir.0 * (pattern_len - 1) >= grid.len() as i32
                    || x as i32 + dir.1 * (pattern_len - 1) < 0
                    || x as i32 + dir.1 * (pattern_len - 1) >= grid[0].len() as i32
                {
                    continue;
                }
                count += pattern.chars().enumerate().all(|(i, c)| {
                    c == grid[(y as i32 + dir.0 * (i as i32)) as usize]
                        [(x as i32 + dir.1 * (i as i32)) as usize]
                }) as i32;
            }
        }
    }

    println!("Part 1: {}", count);

    count = 0;

    for y in 1..grid.len() - 1 {
        for x in 1..grid[0].len() - 1 {
            let c = grid[y][x];

            if c == 'A'
                && ((grid[(y as i32 - 1) as usize][(x as i32 - 1) as usize] == 'M'
                    && grid[(y as i32 + 1) as usize][(x as i32 + 1) as usize] == 'S')
                    || (grid[(y as i32 - 1) as usize][(x as i32 - 1) as usize] == 'S'
                        && grid[(y as i32 + 1) as usize][(x as i32 + 1) as usize] == 'M'))
                && ((grid[(y as i32 - 1) as usize][(x as i32 + 1) as usize] == 'M'
                    && grid[(y as i32 + 1) as usize][(x as i32 - 1) as usize] == 'S')
                    || (grid[(y as i32 - 1) as usize][(x as i32 + 1) as usize] == 'S'
                        && grid[(y as i32 + 1) as usize][(x as i32 - 1) as usize] == 'M'))
            {
                count += 1;
            }
        }
    }

    println!("Part 2: {}", count);
}

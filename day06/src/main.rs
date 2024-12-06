use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut visited = HashSet::new();

    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    let mut map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i32, y as i32), c))
        })
        .collect::<HashMap<_, _>>();

    let initial_pos = map
        .iter()
        .find(|(_, &c)| c == '^')
        .map(|((x, y), _)| (*x, *y))
        .unwrap();

    map.entry(initial_pos).and_modify(|c| *c = '.');
    let initial_map = map.clone();

    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];
    let mut dir = 1;

    let mut pos = initial_pos;

    loop {
        visited.insert(pos);

        let (dx, dy) = dirs[dir];

        if let Some(c) = map.get(&(pos.0 + dx, pos.1 + dy)) {
            match c {
                '.' => pos = (pos.0 + dx, pos.1 + dy),
                '#' => dir = (dir + 1) % dirs.len(),
                _ => unreachable!(),
            }
        } else {
            break;
        }
    }

    println!("{:?}", visited.len());
    let mut count = 0;

    for y in 0..height {
        for x in 0..width {
            let mut visited = HashSet::new();
            let mut pos = initial_pos;
            let mut dir = 1;
            let mut map = initial_map.clone();
            map.entry((x as i32, y as i32)).and_modify(|c| *c = '#');

            if (x as i32,y as i32) == pos {
                continue;
            }

            loop {
                // if we were here before, this is a loop
                if !visited.insert((pos, dir)) {
                    count += 1;
                    break;
                }

                let (dx, dy) = dirs[dir];

                if let Some(c) = map.get(&(pos.0 + dx, pos.1 + dy)) {
                    match c {
                        '.' => pos = (pos.0 + dx, pos.1 + dy),
                        '#' => dir = (dir + 1) % dirs.len(),
                        _ => unreachable!(),
                    }
                } else {
                    // we've left the grid without entering a loop
                    break;
                }
            }
        }
        println!("Finished row {} of {}", y+1, height);
    }
    println!("Part 2: {}", count);
}

use std::{collections::LinkedList, fs};

#[derive(Debug)]
enum Block {
    File(i32, i32),
    Empty(i32),
}

impl Block {
    fn size(&self) -> i32 {
        match self {
            Block::File(size, _) => *size,
            Block::Empty(size) => *size,
        }
    }
}

fn main() {
    let input = fs::read_to_string("test.txt").expect("Unable to read file");

    let mut fs: LinkedList<Block> = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let size = i32::from_str_radix(&c.to_string(), 10).unwrap();
            if i % 2 == 0 {
                Block::File(size, (i as i32) / 2)
            } else {
                Block::Empty(size)
            }
        })
        .collect();

    println!("{:?}", fs);
}

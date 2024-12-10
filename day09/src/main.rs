use std::fs;

fn parse_disk_map(disk_map: &str) -> Vec<char> {
    let mut blocks = Vec::new();
    for (i, c) in disk_map.chars().enumerate() {
        let length = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            // File blocks, use file IDs
            blocks.extend(vec![(i / 2).to_string().chars().next().unwrap(); length as usize]);
        } else {
            // Free space, represented by '.'
            blocks.extend(vec!['.'; length as usize]);
        }
    }
    blocks
}

fn move_blocks(disk: &mut Vec<char>) {

    loop {
        // Find the rightmost file block
        if let Some(file_index) = disk.iter().rposition(|&c| c != '.') {
            // Find the leftmost free space
            if let Some(free_index) = disk.iter().position(|&c| c == '.') {
                if free_index < file_index {
                    // Move the file block to the leftmost free space
                    disk[free_index] = disk[file_index];
                    disk[file_index] = '.';
                    continue;
                }
            }
        }
        break; // Stop if no more moves can be made
    }

}

fn calculate_checksum(disk: &Vec<char>) -> u64 {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &block)| {
            if block != '.' {
                Some(pos as u64 * block.to_digit(10).unwrap() as u64)
            } else {
                None
            }
        })
        .sum()
}

fn main() {
    let disk_map = fs::read_to_string("input2.txt").expect("Unable to read file");

    // Parse the input into the initial disk layout
    let mut disk = parse_disk_map(&disk_map);

    // Simulate the block movements
    move_blocks(&mut disk);

    // Calculate and print the checksum
    let checksum = calculate_checksum(&disk);
    println!("\nFinal Checksum: {}", checksum);
    // println!("\n{}", disk.iter().collect::<String>());

}

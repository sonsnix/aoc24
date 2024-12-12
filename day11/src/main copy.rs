use slab::Slab;

#[derive(Copy, Clone)]
struct Item {
    value: u64,
    prev: Option<usize>,
    next: Option<usize>,
}

impl Item {
    fn print_forward(&self, slab: &Slab<Item>) {
        print!("{} ", self.value);
        let mut item = self;

        while let Some(next_id) = item.next {
            item = &slab[next_id];
            if item.next == None {
                print!("{}\n", item.value);
            } else {
                print!("{} ", item.value);
            }
        }
    }

    fn count(&self, slab: &Slab<Item>) -> u64 {
        let mut item = self;
        let mut count = 1;
        while let Some(next_id) = item.next {
            item = &slab[next_id];
            count += 1;
        }
        count
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut slab: Slab<Item> = Slab::new();
    let mut prev_item = None;

    let first = slab.vacant_key();

    for number in input.split(" ").map(|s| s.parse().unwrap()) {
        let item = slab.insert(Item {
            value: number,
            prev: prev_item,
            next: None,
        });

        if let Some(prev_item) = prev_item {
            slab[prev_item].next = Some(item);
        }

        prev_item = Some(item);
    }

    for i in 0..75 {
        // slab[first].print_forward(&slab);

        let mut cursor = first;

        loop {
            let item = slab[cursor];
            if item.value == 0 {
                slab[cursor].value = 1;
            } else if item.value.to_string().len() % 2 == 0 {
                let str_val = item.value.to_string();
                let left_str = &str_val[0..str_val.len() / 2];
                let right_str = &str_val[str_val.len() / 2..str_val.len()];
                let left_value = left_str.parse().unwrap();
                let right_value = right_str.parse().unwrap();

                let right_item_id = slab.insert(Item {
                    value: right_value,
                    prev: Some(cursor),
                    next: item.next,
                });

                slab[cursor].value = left_value;
                slab[cursor].next = Some(right_item_id);
            } else {
                slab[cursor].value *= 2024;
            }

            if let Some(next) = item.next {
                cursor = next;
            } else {
                break;
            }
        }
        println!("After {} blinks: {}", i + 1, slab[first].count(&slab));
    }

    println!("Part 2: {}", slab[first].count(&slab));
}

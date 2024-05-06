use std::fs;

use itertools::Itertools;

pub fn solution() {
    let raw: String = fs::read("./samples/loowater.txt")
        .expect("file not found")
        .iter()
        .map(|b| *b as char)
        .collect();
    let mut lines = raw.lines();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line == "0 0" {
            break;
        }

        let values = line.split(" ").collect_vec();
        let head_count = values[0].parse().unwrap();
        let knight_count = values[1].parse().unwrap();

        let mut heads = vec![];
        let mut knights: Vec<i32> = vec![];

        for _ in 0..head_count {
            if let Some(v) = lines.next() {
                heads.push(v.trim().parse().unwrap());
            }
        }

        heads.sort_unstable();

        for _ in 0..knight_count {
            if let Some(v) = lines.next() {
                knights.push(v.trim().parse().unwrap());
            }
        }
        knights.sort_unstable();

        let mut heads = heads.iter();
        let mut knights = knights.iter();

        let mut gold_coin_count = 0;

        'head: while let Some(head_diameter) = heads.next() {
            while let Some(knight_height) = knights.next() {
                if knight_height >= head_diameter {
                    gold_coin_count += knight_height;
                    continue 'head;
                }
            }
            gold_coin_count = -1;
        }

        if gold_coin_count == -1 {
            println!("Loowater is doomed!")
        } else {
            println!("{}", gold_coin_count);
        }
    }
}

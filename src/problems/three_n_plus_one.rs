use std::{cmp, collections::HashMap, fs::read_to_string};

pub fn solution() {
    // let i = get_number();
    // let j = get_number();
    let mut memo = HashMap::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        let values = line.trim().split(" ").collect::<Vec<_>>();
        let i = values[0].parse().unwrap();
        let j = values[1].parse().unwrap();

        let (i, j) = (cmp::min(i, j), cmp::max(i, j));
        let mut max_len = 0;
        for i in i..=j {
            let len = cycle_len(&mut memo, i);
            max_len = cmp::max(max_len, len);
        }

        // println!("range {}..={} has a max cycle length of {}", i, j, max_len);
    }
}

fn cycle_len(memo: &mut HashMap<u64, u64>, n: u64) -> u64 {
    if memo.contains_key(&n) {
        return *memo.get(&n).expect("memo problem");
    }
    let mut m = n;
    let mut len = 1;
    while m > 1 {
        if m % 2 == 0 {
            m /= 2;
            len += 1;
        } else {
            m = (3 * m + 1) / 2;
            len += 2;
        }
        if memo.contains_key(&m) {
            len += *memo.get(&m).expect("memo problem");
            m = 0;
        }
    }
    memo.insert(n, len);
    len
}

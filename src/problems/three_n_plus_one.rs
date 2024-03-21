use std::cmp;

use crate::helpers::{self, get_number};

pub fn solution() -> bool {
    let i = get_number();
    let j = get_number();
    let (i, j) = (cmp::min(i, j), cmp::max(i, j));

    let mut max_len = 0;
    for i in i..=j {
        let len = cycle_len(i);
        max_len = cmp::max(max_len, len);
    }

    println!("range {}..={} has a max cycle length of {}", i, j, max_len);

    helpers::end()
}

fn cycle_len(mut n: u32) -> i32 {
    let mut len = 1;
    while n > 1 {
        if n % 2 == 0 {
            n /= 2;
            len += 1;
        } else {
            n = (3 * n + 1) / 2;
            len += 2;
        }
    }
    len
}

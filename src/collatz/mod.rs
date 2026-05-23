use std::{collections::HashMap, time::Instant};

mod loopup;
use loopup::*;

pub fn start() {
    println!("Start collatz");
    let x = 90_000_000;

    // let now = Instant::now();
    // calc_unopmimized(x);
    // let elapsed_time = now.elapsed();
    // println!(
    //     "Unoptimized collatz took {} seconds.",
    //     elapsed_time.as_secs_f32()
    // );

    let mut lookup = Lookup::new(x);
    let now = Instant::now();
    calc_optimized(x as u64, &mut lookup);
    let elapsed_time = now.elapsed();
    println!(
        "Optimized collatz took {} seconds.",
        elapsed_time.as_secs_f32()
    );
}

fn calc_optimized(x: u64, lookup: &mut Lookup) {
    let mut max_len = 0;
    let mut num = 0;
    let mut items: Vec<(u64, u32)> = Vec::with_capacity(200);
    for n in 1..=x {
        let length = calc_op_max_len(n, lookup, &mut items);
        if length > max_len {
            num = n;
            max_len = length;
        }
        // println!("Max: {max}, Length: {}", length - 3);
        // x -= 1;
    }
    println!("MaxLength: {max_len}, Number: {num}");
}

#[inline]
fn calc_op_max_len(x: u64, lookup: &mut Lookup, items: &mut Vec<(u64, u32)>) -> u32 {
    items.clear();
    items.push((x, 0));
    let mut x = x;
    let mut length: u32 = 0;
    loop {
        if x & 1 == 0 {
            let tz = x.trailing_zeros();
            x >>= tz;
            length += tz;
        } else {
            if let Some(cached_len) = lookup.get(x).copied() {
                let total_len = length + cached_len;
                lookup.merge(items, total_len);
                return total_len;
            }
            x = (3 * x + 1) >> 1;
            length += 2;
        }
        items.push((x, length));
        if x == 1 {
            break;
        }
    }
    lookup.merge(items, length);

    length
}

fn calc_unopmimized(x: i64) {
    let mut x = x;
    let mut max_len = 0;
    let mut num = 0;
    while x > 0 {
        let length = calc_unop_max_len(x);
        if length > max_len {
            num = x;
            max_len = length;
        }
        println!("Length of {x}: {}", length);
        x -= 1;
    }
    println!("MaxLength: {max_len}, Number: {num}");
}

#[inline]
fn calc_unop_max_len(x: i64) -> i64 {
    let mut x = x;
    let mut length = 0;
    loop {
        length += 1;
        if x % 2 == 0 {
            x /= 2;
        } else {
            x = 3 * x + 1;
        }
        if x == 1 {
            break;
        }
    }

    length
}

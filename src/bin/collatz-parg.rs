use lazy_static::*;
use rayon::prelude::*;

use std::collections::HashSet;
use std::sync::RwLock;

lazy_static! {
    static ref CLOSED: RwLock<HashSet<u64>> =
        RwLock::new(HashSet::new());
}

fn collatz(n: u64) -> u64 {
    match n % 2 == 0 {
        true => n / 2,
        false => 3 * n + 1,
    }
}

fn close_collatz(mut n: u64, steps: u64) -> bool {
    let mut stack = vec![n];
    let closed = CLOSED.read().unwrap();
    for _ in 0..steps {
        n = collatz(n);
        stack.push(n);
        if n == 1 || closed.contains(&n) {
            drop(closed);
            let mut closed = CLOSED.write().unwrap();
            stack.into_iter().for_each(|n| {
                closed.insert(n);
            });
            return true;
        }
    }
    false
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let limit: u64 = args[1].parse().unwrap();
    let steps: u64 = args[2].parse().unwrap();
    (1..=limit).into_par_iter().for_each(|n| {
        if !close_collatz(n, steps) {
            println!("{}", n);
        }
    });
}

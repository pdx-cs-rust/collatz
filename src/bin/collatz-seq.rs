use std::collections::HashSet;

fn collatz(n: u64) -> u64 {
    match n % 2 == 0 {
        true => n / 2,
        false => 3 * n + 1,
    }
}

fn close_collatz(mut n: u64, steps: u64, closed: &mut HashSet<u64>) -> bool {
    let mut stack = vec![n];
    for _ in 0..steps {
        n = collatz(n);
        stack.push(n);
        if n == 1 || closed.contains(&n) {
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
    let mut closed = HashSet::new();
    (1..=limit).for_each(|n| {
        if !close_collatz(n, steps, &mut closed) {
            println!("{}", n);
        }
    });
}

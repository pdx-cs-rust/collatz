fn collatz(n: u64) -> u64 {
    match n % 2 == 0 {
        true => n / 2,
        false => 3 * n + 1,
    }
}

fn close_collatz(mut n: u64, steps: u64) -> bool {
    for _ in 0..steps {
        n = collatz(n);
        if n == 1 {
            return true;
        }
    }
    false
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let limit: u64 = args[1].parse().unwrap();
    let steps: u64 = args[2].parse().unwrap();
    (1..=limit).for_each(|n| {
        if !close_collatz(n, steps) {
            println!("{}", n);
        }
    });
}

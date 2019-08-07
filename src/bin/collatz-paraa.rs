#![feature(async_await)]

use futures::{executor::ThreadPool, task::SpawnExt};

fn collatz(n: u64) -> u64 {
    match n % 2 == 0 {
        true => n / 2,
        false => 3 * n + 1,
    }
}

async fn close_collatz(mut n: u64, steps: u64) {
    for _ in 0..steps {
        n = collatz(n);
        if n == 1 {
            return;
        }
    }
    println!("{}", n);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let limit: u64 = args[1].parse().unwrap();
    let steps: u64 = args[2].parse().unwrap();


    let mut pool =
        ThreadPool::new().expect("Failed to create threadpool");
    let mut hs = Vec::new();
    for n in 1..=limit {
        hs.push(pool.spawn_with_handle(close_collatz(n, steps)).unwrap());
    }
    for h in hs.into_iter() {
        let _ = async { let () = h.await; };
    }
}

use std::time::{Duration, Instant};

fn main() {
    let mut count = 0;
    let time_limit = Duration::new(1, 0); // Create Duration of 1s
    let start = Instant::now(); // System clock now

    while (Instant::now() - start) < time_limit {
        count += 1;
    }

    println!("{}", count);
}

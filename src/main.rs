mod p0000_perfect_square;

use std::env;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let arg = env::args().nth(1);

    match arg.as_deref() {
        Some("0")=> {
            let result: u128 = p0000_perfect_square::perfect_square();
            println!("Anser: {:?}", result);
        }
        _ => {
            println!("Usage: cargo run -- <problem_number>");
        }
    }
    let duration = start.elapsed(); // end timer
    println!("Execution time: {:?}", duration);
}
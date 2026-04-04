mod p0000_perfect_square;
mod p0001_multiples_of_3_or_5;
mod p0002_even_fibonacci_numbers;
mod p0025_1000_digit_fibonacci_number;

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
        Some("1")=> {
            let result: u32 = p0001_multiples_of_3_or_5::multiple();
            println!("Anser: {:?}", result);
        }
        Some("2")=> {
            let result: u64 = p0002_even_fibonacci_numbers::fibonacci_force();
            println!("Anser: {:?}", result);
        }
        Some("25")=> {
            let result: u64 = p0025_1000_digit_fibonacci_number::fibonacci_force();
            println!("Anser: {:?}", result);
        }
        _ => {
            println!("Usage: cargo run -- <problem_number>");
        }
    }
    let duration = start.elapsed(); // end timer
    println!("Execution time: {:?}", duration);
}
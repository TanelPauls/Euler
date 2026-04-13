mod p0000_perfect_square;
mod p0001_multiples_of_3_or_5;
mod p0002_even_fibonacci_numbers;
mod p0004_largest_palindrome_product;
mod p0007_10001st_prime;
mod p0018_maximum_path_sum_1;
mod p0025_1000_digit_fibonacci_number;
mod p0054_poker_hands;
mod p0067_maximum_path_sum_2;
mod p0092_square_digit_chains;
mod p0112_bouncy_numbers;

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
        Some("4")=> {
            let result: i32 = p0004_largest_palindrome_product::palindrome_product();
            println!("Anser: {:?}", result);
        }
        Some("7")=> {
            let n:u32 = 1;
            let result: u64 = p0007_10001st_prime::prime_sieve(n);
            println!("Anser: {:?}", result);
        }
        Some("18")=> {
            let result: i32 = p0018_maximum_path_sum_1::max_sum();
            println!("Anser: {:?}", result);
        }
        Some("25")=> {
            let result: u64 = p0025_1000_digit_fibonacci_number::fibonacci_force();
            println!("Anser: {:?}", result);
        }
        Some("54")=> {
            let result: u32 = p0054_poker_hands::poker_hands();
            println!("Anser: {:?}", result);
        }
        Some("67")=> {
            let result: i32 = p0067_maximum_path_sum_2::max_sum();
            println!("Anser: {:?}", result);
        }
        Some("92")=> {
            let result: u64 = p0092_square_digit_chains::square_digit_chains();
            println!("Anser: {:?}", result);
        }
        Some("112")=> {
            let percentage: u8 = 99;
            let result: u32 = p0112_bouncy_numbers::bouncy_numbers(percentage);
            println!("Anser: {:?}", result);
        }
        _ => {
            println!("Usage: cargo run -- <problem_number>");
        }
    }
    let duration = start.elapsed(); // end timer
    println!("Execution time: {:?}", duration);
}
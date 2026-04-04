use rug::{Assign, Integer};
use rug::ops::Pow;

pub fn fibonacci_force() -> u64 {
    let mut fib1 = Integer::from(1);
    let mut fib2 = Integer::from(1);
    let mut fib3 = Integer::from(0);
    let mut answer: u64 = 1;
    let limit = Integer::from(10).pow(999);

    while fib3 <= limit {
        fib3.assign(&fib1 + &fib2); // complete the lazy add into fib3
        fib1.assign(&fib2);         // copy value into fib1
        fib2.assign(&fib3);         // copy value into fib2
        answer+=1;
    }
    answer+1
    // Execution time: 800.735µs
}


/* fn fibonacci(n:i32) -> Integer {
    if n==0 {
        return Integer::from(1);
    }
    else if n==1{
        return Integer::from(1);
    }
    return fibonacci(n-2) + fibonacci(n-1)
} */
pub fn fibonacci_force() -> u64 {

    let mut fib1:u64 = 1;
    let mut fib2:u64 = 2;
    let mut fib3:u64 = 0;
    let mut answer:u64= fib2;

    while fib3<=4_000_000 {
        fib3=fib1+fib2;
        fib1=fib2;
        fib2=fib3;
        if fib3 %2 == 0 {
            answer+=fib3;
        }
    }

    answer
    //Execution time: 34.146µs
}
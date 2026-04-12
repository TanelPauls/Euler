pub fn square_digit_chains() -> u64 {
    let mut answer: u64 = 0;

    for x in 1..10_000_000 {
        if e89_or_1(x) == true {
            answer+=1;
        }
/*         if x%100_000==0{
            println!("Current x={}",x)
        } */
    }

    answer
    // Execution time: 5.494112942s
}

fn square_of_digits(mut n:u64) -> u32{
    let mut sum:u32 = 0;
    while n>0 {
        sum+=(n%10).pow(2) as u32;
        n /= 10;
    }
    return sum
}

fn e89_or_1(mut n: u64) -> bool {
    while n != 1 && n != 89 {
        n = square_of_digits(n) as u64;
    }
    n == 89
}
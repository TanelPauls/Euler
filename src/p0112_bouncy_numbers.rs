pub fn bouncy_numbers(_percentage:u8) -> u32 {

    let answer= 0;
    let mut bouncy = 0;


    for x in 0..4000000 {
        if x > 100 && !check_increasing(x) && !check_decreasing(x) {
            bouncy += 1;
        }

        /* Avoid floating point entirely. Compare integers:

        if bouncy * 100 >= x * (percentage as u32) {
            return x;
        } */

        if (bouncy as f64) / (x as f64)>=0.99 {
            //println!("{}", x);
            return x;
        }
    }
    //println!("Total bouncy: {}", bouncy);

    //Execution time: 109.423186ms

    answer
}

fn check_increasing(mut n:u32)->bool{
    if n<=100 {
        return false
    }
    let mut prev = 10; 
    while n > 0 {
        let digit = n % 10;
        if digit > prev {
            return false;
        }
        prev = digit;
        n /= 10;
    }
    true
}

fn check_decreasing(mut n:u32)->bool{
    if n<=100 {
        return false
    }
    let mut prev = 0;
    while n > 0 {
        let digit = n % 10;
        if digit < prev {
            return false;
        }
        prev = digit;
        n /= 10;
    }
    true
}

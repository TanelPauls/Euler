fn is_palindrome(mut number: i32) -> bool {
    if number < 0 {
        number = number * -1;
    }
    let check = number;

    if number < 10 {
        return true;
    }

    let mut reverse: i32 = 0;

    while number > 0 {
        reverse += number%10;
        number /= 10;
        reverse *= 10;
    }

    if check == reverse/10 {
        return true;
    }

    return false
}

pub fn palindrome_product() -> i32 {
    let mut answer = 0;
    //println!("{}", is_palindrome(-31313));

    /* for x in 1..1000 {
        for y in 1..1000 {
            if is_palindrome(x * y) {
                if x * y > answer {
                    answer = x *y
                }
            }
        }
    }
    answer 
    // xecution time: 157.778587ms */
    for x in (1..1000).rev() {
        for y in (x..1000).rev() {
            let product = x * y;

            if product <= answer {
                break;
            }

            if is_palindrome(product) {
                answer = product;
            }
        }
    }
    answer
    // Execution time: 468.03µs
}
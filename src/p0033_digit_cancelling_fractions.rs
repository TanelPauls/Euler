fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn digit_cancelling_fraction() -> i32 {
    let mut num_product = 1;
    let mut den_product = 1;

    for a in 1..10 {
        for b in 1..10 {
            for d in 1..10 {
                let numerator = 10 * a + b;
                let denominator = 10 * b + d;

                if numerator >= denominator {
                    continue;
                }

                // check "digit cancelling"
                if numerator * d == denominator * a {
                    num_product *= numerator;
                    den_product *= denominator;
                }
            }
        }
    }

    let g = gcd(num_product, den_product);
    den_product / g
    // Execution time: 63.277µs
}

/* pub fn digit_cancelling_fraction() -> i32 {
    let mut a;
    let mut b;
    let mut c;
    let mut d;
    let mut values: Vec<Vec<i32>> = Vec::new();
    for i in 11..100 {
        if i % 10 == 0 {
            continue;
        }
        a = i/10;
        b = i%10;
        for j in (i+1)..100 {
            if j % 10 == 0 {
                continue;
            }
            c = j/10;
            d = j%10;
            if a == c {
                if i * d == j * b {
                    values.push(vec![i,j]);
                }
            }
            if a == d {
                if i * c == j * b {
                    values.push(vec![i,j]);
                }
            }
            if b == c {
                if i * d == j * a {
                    values.push(vec![i,j]);
                }
            }
            if b == d {
                if i * c == j * a {
                    values.push(vec![i,j]);
                }
            }
        }

    }
    println!("{:?}", values);
    5
    // Execution time: 198.609µs
} */
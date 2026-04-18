pub fn digit_cancelling_fraction() -> i32 {
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
}
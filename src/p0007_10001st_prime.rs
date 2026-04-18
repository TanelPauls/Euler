pub fn prime_sieve() -> u64 {

    // upper bound for nth prime = n(ln n + ln ln n)
    let s = sieve_of_eratosthenes(110000);

    println!("{}", s.len());
    if s.len()>10001 {
        return s[10000];
    }
    return 0;
    //Execution time: 18.667631ms
}

fn sieve_of_eratosthenes(n: u32) -> Vec<u64> {
    if n<2 {
        return vec![];
    }
    // There is a Vec<bool> which is much better, than duplicating here
    let mut numbers: Vec<(u64, bool)> = vec![];
    for i in 0..(n+1) {
        numbers.push((i as u64, true));
    }
    numbers[0].1=false;
    numbers[1].1=false;

    for i in 2..((n as f64).sqrt() as u64)+1 {
        if numbers[i as usize].1==true{
            for j in (i*i..(n+1) as u64).step_by(i as usize){
                numbers[j as usize].1=false;
            }
        }
    }
    
    let mut answer = vec![];
    for i in numbers {
        if i.1==true {
            answer.push(i.0);
        }
    }
    
    //println!("{:?}", numbers);
    
    //println!("{}", (n as f64).sqrt() as u64);
    //println!("{}", (25.0_f64).sqrt() as u64);
    //println!("{}", (26.0_f64).sqrt() as u64);
    
    return answer;
}
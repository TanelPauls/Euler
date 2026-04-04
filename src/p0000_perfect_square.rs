pub fn perfect_square() -> u128 {

    let mut answer= 0;

    for i in 1..(288000+1) {
        if i%2==0 {
            continue;
        }
        if ((i*i)%2) != 0{
            answer+=i*i;
        }
    }

    answer
}
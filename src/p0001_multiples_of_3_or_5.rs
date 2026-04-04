pub fn multiple() -> u32 {

    let mut answer= 0;

    for i in 1..(999+1) {
        if i%3==0 || i%5==0 {
            answer+=i;
        }
    }

    answer
    // Execution time: 51.323µs
}
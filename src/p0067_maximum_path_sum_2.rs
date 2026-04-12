use std::array;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::cmp::max;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn triangle() -> Vec<Vec<i32>> {
    let mut triangle:Vec<Vec<i32>>=vec![];

    if let Ok(lines) = read_lines("./files/p0067_triangle.txt") {
        for line in lines.map_while(Result::ok) {
            let numbers: Vec<i32> = line
                .split_whitespace()        // split by spaces
                .map(|s| s.parse().unwrap()) // convert to i32
                .collect();

            triangle.push(numbers);
        }
    }

    triangle
}

pub fn max_sum()-> i32 {

    let mut triangle = triangle();

    for x in (0..triangle.len()-1).rev() {
        for y in 0..triangle[x].len(){
            triangle[x][y]=triangle[x][y]+max(triangle[x+1][y],triangle[x+1][y+1]);
        }
    }
    
    triangle[0][0]
    //Execution time: 3.364194ms
}
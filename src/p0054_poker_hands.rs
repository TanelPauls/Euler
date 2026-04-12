use std::array;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_straight(hand: String) -> (bool, [i32; 5]) {
    println!("{}", hand);
    let mut values = [0,0,0,0,0];
    let mut count= 0;
    for n in 0..10 {
        if n%2==0 {
            if hand.chars().nth(n) == Some('T') {
                values[count]=10;
            }
            if hand.chars().nth(n) == Some('J') {
                values[count]=11;
            }
            else if hand.chars().nth(n) == Some('Q') {
                values[count]=12;
            }
            else if hand.chars().nth(n) == Some('K') {
                values[count]=13;
            }
            else if hand.chars().nth(n) == Some('A') {
                values[count]=14;
            }
            else {
                if let Some(d) = hand.chars().nth(n).unwrap().to_digit(10) {
                    values[count] = d as i32;
                }
            }
            count += 1;
        }
    }
    values.sort();
    if values[4]-values[0]==4 {
        return ( true , values)
    }
    //println!("{:?}", values);
    (false, values)
}

fn hand_ranker(hand: String) -> (u8, [i32; 5]) {
    //println!("{:?}", hand);
    if (hand.chars().nth(1) == hand.chars().nth(3)) && 
       (hand.chars().nth(1) == hand.chars().nth(5)) && 
       (hand.chars().nth(1) == hand.chars().nth(7)) && 
       (hand.chars().nth(1) == hand.chars().nth(9)) {
        println!("flush");
        if is_straight(hand).0 {
            if is_straight(hand.clone()).1[0] == 10 {
                return (10, is_straight(hand.clone()).1);
            }
        }
    }
    (5,[0,0,0,0,0])
    
}

// returns true, if first hand wins
fn poker_hand_comparer(hands: &String) -> bool {
    let hand_1: String = [
        hands.chars().nth(0).unwrap(),
        hands.chars().nth(1).unwrap(),
        hands.chars().nth(3).unwrap(),
        hands.chars().nth(4).unwrap(),
        hands.chars().nth(6).unwrap(),
        hands.chars().nth(7).unwrap(),
        hands.chars().nth(9).unwrap(),
        hands.chars().nth(10).unwrap(),
        hands.chars().nth(12).unwrap(),
        hands.chars().nth(13).unwrap(),
    ]
    .into_iter()
    .collect();

    let hand_2: String = [
        hands.chars().nth(15).unwrap(),
        hands.chars().nth(16).unwrap(),
        hands.chars().nth(18).unwrap(),
        hands.chars().nth(19).unwrap(),
        hands.chars().nth(21).unwrap(),
        hands.chars().nth(22).unwrap(),
        hands.chars().nth(24).unwrap(),
        hands.chars().nth(25).unwrap(),
        hands.chars().nth(27).unwrap(),
        hands.chars().nth(28).unwrap(),
    ]
    .into_iter()
    .collect();

    let hand_1_rank = hand_ranker(hand_1);
    let hand_1_rank = hand_ranker(hand_2);


    false
}

pub fn poker_hands() -> u32 {
    let mut wins = 0;

    //if let Ok(lines) = read_lines("./files/p0054_poker_hands.txt") {
    if let Ok(lines) = read_lines("./files/test.txt") {
        for line in lines.map_while(Result::ok) {
            if poker_hand_comparer(&line) {
                wins += 1;
            }
        }
    }

    wins
}
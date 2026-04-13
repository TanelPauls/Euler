use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn poker_hands() -> u32 {
    let mut wins = 0;
    let mut hands_to_compare: (Hand, Hand);

    if let Ok(lines) = read_lines("./files/p0054_poker_hands.txt") {
    //if let Ok(lines) = read_lines("./files/test.txt") {
        for line in lines.map_while(Result::ok) {
            hands_to_compare = divide_line_to_hands(line);
            if compare_hands(hands_to_compare.0,hands_to_compare.1) {
                wins+=1;
            }
        }
    }
    wins
}
fn divide_line_to_hands(line: String)-> (Hand, Hand) {
    let hand_1: String = [
        line.chars().nth(0).unwrap(),
        line.chars().nth(1).unwrap(),
        line.chars().nth(3).unwrap(),
        line.chars().nth(4).unwrap(),
        line.chars().nth(6).unwrap(),
        line.chars().nth(7).unwrap(),
        line.chars().nth(9).unwrap(),
        line.chars().nth(10).unwrap(),
        line.chars().nth(12).unwrap(),
        line.chars().nth(13).unwrap(),
    ]
    .into_iter()
    .collect();

    let hand_2: String = [
        line.chars().nth(15).unwrap(),
        line.chars().nth(16).unwrap(),
        line.chars().nth(18).unwrap(),
        line.chars().nth(19).unwrap(),
        line.chars().nth(21).unwrap(),
        line.chars().nth(22).unwrap(),
        line.chars().nth(24).unwrap(),
        line.chars().nth(25).unwrap(),
        line.chars().nth(27).unwrap(),
        line.chars().nth(28).unwrap(),
    ]
    .into_iter()
    .collect();


    //println!("{:?}", hand_1);
    //println!("{:?}", hand_2);

    let hand_11 = Hand::new(hand_1);
    let hand_22 = Hand::new(hand_2);
    (hand_11,hand_22)
}

struct Hand {
    hand_rank: u8,
    sorted_hand_values: [u8; 5]
}

impl Hand {
    pub fn new(hand: String) -> Hand {
        
        let flush = Self::is_flush(&hand);
        let mut sorted_hand_values = Self::sort_hand_values(&hand);
        let mut hand_rank:u8;

        if flush {
            hand_rank=6;
            if 
            (sorted_hand_values[4]-sorted_hand_values[0] == 4 &&
            sorted_hand_values[0]!=sorted_hand_values[1] &&
            sorted_hand_values[1]!=sorted_hand_values[2] &&
            sorted_hand_values[2]!=sorted_hand_values[3] &&
            sorted_hand_values[3]!=sorted_hand_values[4]) ||
            sorted_hand_values==[2,3,4,5,14] {
                if sorted_hand_values==[2,3,4,5,14] {
                    sorted_hand_values=[1,2,3,4,5];
                }
                hand_rank=9;
            }
            if 
            sorted_hand_values==[10,11,12,13,14] {
                hand_rank=10;
            }
        } else if 
        (sorted_hand_values[4]-sorted_hand_values[0] == 4 &&
        sorted_hand_values[0]!=sorted_hand_values[1] &&
        sorted_hand_values[1]!=sorted_hand_values[2] &&
        sorted_hand_values[2]!=sorted_hand_values[3] &&
        sorted_hand_values[3]!=sorted_hand_values[4]) ||
        sorted_hand_values==[2,3,4,5,14] {
            if sorted_hand_values==[2,3,4,5,14] {
                sorted_hand_values=[1,2,3,4,5];
            }
            hand_rank=5;
        } else if 
            ( sorted_hand_values[0]==sorted_hand_values[1] &&
            sorted_hand_values[0]==sorted_hand_values[2] &&
            sorted_hand_values[0]==sorted_hand_values[3] ) || 
            ( sorted_hand_values[1]==sorted_hand_values[2] &&
            sorted_hand_values[1]==sorted_hand_values[3] &&
            sorted_hand_values[1]==sorted_hand_values[4] )
            {
                hand_rank=8;
        } else if 
            (sorted_hand_values[0]==sorted_hand_values[1] &&
            sorted_hand_values[0]==sorted_hand_values[2] &&
            sorted_hand_values[3]==sorted_hand_values[4]) ||
            (sorted_hand_values[0]==sorted_hand_values[1] &&
            sorted_hand_values[2]==sorted_hand_values[3] &&
            sorted_hand_values[2]==sorted_hand_values[4])
            {
                hand_rank=7;
        } else if 
            (sorted_hand_values[0]==sorted_hand_values[1] &&
            sorted_hand_values[0]==sorted_hand_values[2]) ||
            (sorted_hand_values[1]==sorted_hand_values[2] &&
            sorted_hand_values[1]==sorted_hand_values[3]) ||
            (sorted_hand_values[2]==sorted_hand_values[3] &&
            sorted_hand_values[2]==sorted_hand_values[4])
            {
                hand_rank=4;
        } else if 
            (sorted_hand_values[0]==sorted_hand_values[1] &&
            sorted_hand_values[2]==sorted_hand_values[3]) ||
            (sorted_hand_values[0]==sorted_hand_values[1] &&
            sorted_hand_values[3]==sorted_hand_values[4]) ||
            (sorted_hand_values[1]==sorted_hand_values[2] &&
            sorted_hand_values[3]==sorted_hand_values[4])
            {
                hand_rank=3;
        } else if 
            (sorted_hand_values[0]==sorted_hand_values[1]) ||
            (sorted_hand_values[1]==sorted_hand_values[2]) ||
            (sorted_hand_values[2]==sorted_hand_values[3]) ||
            (sorted_hand_values[3]==sorted_hand_values[4])
            {
                hand_rank=2;
        } else {
            hand_rank=1;
        }

        //println!("hand rank: {:?}", hand_rank);
        Hand {
            hand_rank: hand_rank,
            sorted_hand_values: sorted_hand_values,
        }
    }
    fn is_flush(hand: &str) -> bool {
        let suits: Vec<char> = hand.chars().skip(1).step_by(2).collect();
        let is_flush = suits.iter().all(|&s| s == suits[0]);
        // println!("{:?}", suits);
        if is_flush {
            return true;
        }
        false
    }
    fn sort_hand_values(hand: &str) -> [u8; 5] {
        let hand_values_char: Vec<char> = hand.chars().step_by(2).collect();
        let mut hand_values_int: [u8; 5] = [0,0,0,0,0];
        for i in 0..hand_values_char.len() {
            if hand_values_char[i] == 'T' {
                hand_values_int[i]=10;
            }
            else if hand_values_char[i] == 'J' {
                hand_values_int[i]=11;
            }
            else if hand_values_char[i] == 'Q' {
                hand_values_int[i]=12;
            }
            else if hand_values_char[i] == 'K' {
                hand_values_int[i]=13;
            }
            else if hand_values_char[i] == 'A' {
                hand_values_int[i]=14;
            }
            else {
                hand_values_int[i] = hand_values_char[i].to_digit(10).unwrap() as u8;
            }
        }
        hand_values_int.sort();
        hand_values_int
    }
}

fn compare_hands(hand_1: Hand,hand_2: Hand) -> bool {
    if hand_1.hand_rank > hand_2.hand_rank {
        return true;
    } else if hand_1.hand_rank < hand_2.hand_rank {
        return false;
    } else {
        if hand_1.hand_rank==9 {
            if hand_1.sorted_hand_values[4]>hand_2.sorted_hand_values[4] {
                return true;
            } else {
                return false;
            }
        } else if hand_1.hand_rank==8 {
            let big_hand_1: u8;
            if hand_1.sorted_hand_values[0]==hand_1.sorted_hand_values[1] {
                big_hand_1=hand_1.sorted_hand_values[0];
            } else {
                big_hand_1=hand_1.sorted_hand_values[1];
            }
            let big_hand_2: u8;
            if hand_2.sorted_hand_values[0]==hand_2.sorted_hand_values[1] {
                big_hand_2=hand_2.sorted_hand_values[0];
            } else {
                big_hand_2=hand_2.sorted_hand_values[1];
            }
            if big_hand_1>big_hand_2 {
                return true;
            }
            else {
                return false;
            }
        } else if hand_1.hand_rank==7 {
            let big_hand_1: u8;
            if 
                hand_1.sorted_hand_values[0]==hand_1.sorted_hand_values[1] &&
                hand_1.sorted_hand_values[0]==hand_1.sorted_hand_values[2] {
                big_hand_1=hand_1.sorted_hand_values[0];
            } else {
                big_hand_1=hand_1.sorted_hand_values[2];
            }
            let big_hand_2: u8;
            if 
                hand_2.sorted_hand_values[0]==hand_2.sorted_hand_values[1] &&
                hand_2.sorted_hand_values[0]==hand_2.sorted_hand_values[2] {
                big_hand_2=hand_2.sorted_hand_values[0];
            } else {
                big_hand_2=hand_2.sorted_hand_values[2];
            }
            if big_hand_1>big_hand_2 {
                return true;
            }
            else {
                return false;
            }
        } else if hand_1.hand_rank==6 {
            if hand_1.sorted_hand_values[4]>hand_2.sorted_hand_values[4]{
                return true;
            } else if hand_1.sorted_hand_values[4]<hand_2.sorted_hand_values[4]{
                return false;
            } else {
                if hand_1.sorted_hand_values[3]>hand_2.sorted_hand_values[3]{
                    return true;
                } else if hand_1.sorted_hand_values[3]<hand_2.sorted_hand_values[3]{
                    return false;
                } else {
                    if hand_1.sorted_hand_values[2]>hand_2.sorted_hand_values[2]{
                        return true;
                    } else if hand_1.sorted_hand_values[2]<hand_2.sorted_hand_values[2]{
                        return false;
                    } else {
                        if hand_1.sorted_hand_values[1]>hand_2.sorted_hand_values[1]{
                            return true;
                        } else if hand_1.sorted_hand_values[1]<hand_2.sorted_hand_values[1]{
                            return false;
                        } else {
                            if hand_1.sorted_hand_values[0]>hand_2.sorted_hand_values[0]{
                                return true;
                            } else {
                                return false;
                            }
                        }
                        
                    }
                }
            }
        } else if hand_1.hand_rank==5 {
            if hand_1.sorted_hand_values[4] > hand_2.sorted_hand_values[4] {
                return true;
            } else {
                return false;
            }
        } else if hand_1.hand_rank==4 {
            let big_hand_1: u8;
            if hand_1.sorted_hand_values[0]==hand_1.sorted_hand_values[1]{
                big_hand_1=hand_1.sorted_hand_values[0];
            } else if hand_1.sorted_hand_values[1]==hand_1.sorted_hand_values[2]{
                big_hand_1=hand_1.sorted_hand_values[1];
            } else {
                big_hand_1=hand_1.sorted_hand_values[2];
            }
            let big_hand_2: u8;
            if hand_2.sorted_hand_values[0]==hand_2.sorted_hand_values[1]{
                big_hand_2=hand_2.sorted_hand_values[0];
            } else if hand_2.sorted_hand_values[1]==hand_2.sorted_hand_values[2]{
                big_hand_2=hand_2.sorted_hand_values[1];
            } else {
                big_hand_2=hand_2.sorted_hand_values[2];
            }
            if big_hand_1>big_hand_2 {
                return true;
            }
            else {
                return false;
            }
        } else if hand_1.hand_rank==3 {
            let mut big_hand_1_1: u8;
            let mut big_hand_1_2: u8;
            let mut swap: u8;
            let small_hand_1: u8;
            if hand_1.sorted_hand_values[0]==hand_1.sorted_hand_values[1] && hand_1.sorted_hand_values[2]==hand_1.sorted_hand_values[3] {
                big_hand_1_1=hand_1.sorted_hand_values[0];
                big_hand_1_2=hand_1.sorted_hand_values[2];
                small_hand_1=hand_1.sorted_hand_values[4];
                if big_hand_1_1 < big_hand_1_2 {
                    swap = big_hand_1_1;
                    big_hand_1_1 = big_hand_1_2;
                    big_hand_1_2 = swap;
                }
            } else if hand_1.sorted_hand_values[0]==hand_1.sorted_hand_values[1] && hand_1.sorted_hand_values[3]==hand_1.sorted_hand_values[4] {
                big_hand_1_1=hand_1.sorted_hand_values[0];
                big_hand_1_2=hand_1.sorted_hand_values[3];
                small_hand_1=hand_1.sorted_hand_values[2];
                if big_hand_1_1 < big_hand_1_2 {
                    swap = big_hand_1_1;
                    big_hand_1_1 = big_hand_1_2;
                    big_hand_1_2 = swap;
                }
            } else {
                big_hand_1_1=hand_1.sorted_hand_values[1];
                big_hand_1_2=hand_1.sorted_hand_values[3];
                small_hand_1=hand_1.sorted_hand_values[0];
                if big_hand_1_1 < big_hand_1_2 {
                    swap = big_hand_1_1;
                    big_hand_1_1 = big_hand_1_2;
                    big_hand_1_2 = swap;
                }
            }
            let mut big_hand_2_1: u8;
            let mut big_hand_2_2: u8;
            let small_hand_2: u8;
            if hand_2.sorted_hand_values[0]==hand_2.sorted_hand_values[1] && hand_2.sorted_hand_values[2]==hand_2.sorted_hand_values[3] {
                big_hand_2_1=hand_2.sorted_hand_values[0];
                big_hand_2_2=hand_2.sorted_hand_values[2];
                small_hand_2=hand_2.sorted_hand_values[4];
                if big_hand_2_1 < big_hand_2_2 {
                    swap = big_hand_2_1;
                    big_hand_2_1 = big_hand_2_2;
                    big_hand_2_2 = swap;
                }
            } else if hand_2.sorted_hand_values[0]==hand_2.sorted_hand_values[1] && hand_2.sorted_hand_values[3]==hand_2.sorted_hand_values[4] {
                big_hand_2_1=hand_2.sorted_hand_values[0];
                big_hand_2_2=hand_2.sorted_hand_values[3];
                small_hand_2=hand_2.sorted_hand_values[2];
                if big_hand_2_1 < big_hand_2_2 {
                    swap = big_hand_2_1;
                    big_hand_2_1 = big_hand_2_2;
                    big_hand_2_2 = swap;
                }
            } else {
                big_hand_2_1=hand_2.sorted_hand_values[1];
                big_hand_2_2=hand_2.sorted_hand_values[3];
                small_hand_2=hand_2.sorted_hand_values[0];
                if big_hand_2_1 < big_hand_2_2 {
                    swap = big_hand_2_1;
                    big_hand_2_1 = big_hand_2_2;
                    big_hand_2_2 = swap;
                }
            }
            if big_hand_1_1>big_hand_2_1 {
                return true;
            } else if big_hand_1_1<big_hand_2_1 {
                return false;
            } else {
                if big_hand_1_2>big_hand_2_2 {
                    return true;
                } else if big_hand_1_2 < big_hand_2_2 {
                    return false;
                } else {
                    if small_hand_1 > small_hand_2 {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        } else if hand_1.hand_rank==2 {
            let big_hand_1: u8;
            let small_hand_1_1: u8;
            let small_hand_1_2: u8;
            let small_hand_1_3: u8;
            if hand_1.sorted_hand_values[0]==hand_1.sorted_hand_values[1] {
                big_hand_1=hand_1.sorted_hand_values[0];
                small_hand_1_1=hand_1.sorted_hand_values[4];
                small_hand_1_2=hand_1.sorted_hand_values[3];
                small_hand_1_3=hand_1.sorted_hand_values[2];
            } else if hand_1.sorted_hand_values[1]==hand_1.sorted_hand_values[2] {
                big_hand_1=hand_1.sorted_hand_values[1];
                small_hand_1_1=hand_1.sorted_hand_values[4];
                small_hand_1_2=hand_1.sorted_hand_values[3];
                small_hand_1_3=hand_1.sorted_hand_values[0];
            } else if hand_1.sorted_hand_values[2]==hand_1.sorted_hand_values[3] {
                big_hand_1=hand_1.sorted_hand_values[2];
                small_hand_1_1=hand_1.sorted_hand_values[4];
                small_hand_1_2=hand_1.sorted_hand_values[1];
                small_hand_1_3=hand_1.sorted_hand_values[0];
            } else {
                big_hand_1=hand_1.sorted_hand_values[3];
                small_hand_1_1=hand_1.sorted_hand_values[2];
                small_hand_1_2=hand_1.sorted_hand_values[1];
                small_hand_1_3=hand_1.sorted_hand_values[0];
            }
            let big_hand_2: u8;
            let small_hand_2_1: u8;
            let small_hand_2_2: u8;
            let small_hand_2_3: u8;
            if hand_2.sorted_hand_values[0]==hand_2.sorted_hand_values[1] {
                big_hand_2=hand_2.sorted_hand_values[0];
                small_hand_2_1=hand_2.sorted_hand_values[4];
                small_hand_2_2=hand_2.sorted_hand_values[3];
                small_hand_2_3=hand_2.sorted_hand_values[2];
            } else if hand_2.sorted_hand_values[1]==hand_2.sorted_hand_values[2] {
                big_hand_2=hand_2.sorted_hand_values[1];
                small_hand_2_1=hand_2.sorted_hand_values[4];
                small_hand_2_2=hand_2.sorted_hand_values[3];
                small_hand_2_3=hand_2.sorted_hand_values[0];
            } else if hand_2.sorted_hand_values[2]==hand_2.sorted_hand_values[3] {
                big_hand_2=hand_2.sorted_hand_values[2];
                small_hand_2_1=hand_2.sorted_hand_values[4];
                small_hand_2_2=hand_2.sorted_hand_values[1];
                small_hand_2_3=hand_2.sorted_hand_values[0];
            } else {
                big_hand_2=hand_2.sorted_hand_values[3];
                small_hand_2_1=hand_2.sorted_hand_values[2];
                small_hand_2_2=hand_2.sorted_hand_values[1];
                small_hand_2_3=hand_2.sorted_hand_values[0];
            }
            if big_hand_1>big_hand_2 {
                return true;
            } else if big_hand_1<big_hand_2 {
                return false;
            } else {
                if small_hand_1_1>small_hand_2_1 {
                    return true;
                } else if small_hand_1_1<small_hand_2_1 {
                    return false;
                } else {
                    if small_hand_1_2>small_hand_2_2 {
                        return true;
                    } else if small_hand_1_2<small_hand_2_2 {
                        return false;
                    } else {
                        if small_hand_1_3>small_hand_2_3 {
                            return true;
                        } else {
                            return false;
                        }
                    }
                }
            }
        } else if hand_1.hand_rank==1 {
            if hand_1.sorted_hand_values[4]>hand_2.sorted_hand_values[4] {
                return true;
            } else if hand_1.sorted_hand_values[4]<hand_2.sorted_hand_values[4] {
                return false;
            } else {
                if hand_1.sorted_hand_values[3]>hand_2.sorted_hand_values[3] {
                    return true;
                } else if hand_1.sorted_hand_values[3]<hand_2.sorted_hand_values[3] {
                    return false;
                } else {
                    if hand_1.sorted_hand_values[2]>hand_2.sorted_hand_values[2] {
                        return true;
                    } else if hand_1.sorted_hand_values[2]<hand_2.sorted_hand_values[2] {
                        return false;
                    } else {
                        if hand_1.sorted_hand_values[1]>hand_2.sorted_hand_values[1] {
                            return true;
                        } else if hand_1.sorted_hand_values[1]<hand_2.sorted_hand_values[1] {
                            return false;
                        } else {
                            if hand_1.sorted_hand_values[0]>hand_2.sorted_hand_values[0] {
                                return true;
                            } else {
                                return false;
                            }
                        }
                    }
                }
            }
        }
    }
    false
    //Execution time: 40.820037ms
}
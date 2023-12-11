

/*

EXAMPLE INPUT:

Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11

*/

use fnv::FnvHashSet;
use std::fs::File;
use std::env;
use std::io::{BufRead, BufReader};


fn score_card(line: &str) -> u32 {
    let halves : Vec<FnvHashSet<u32>> = 
        line.split(":").nth(1)
            .expect("cannot parse card header")
            .split("|")
            .map(|half| half.split_whitespace()
                .map(|num| num.parse()
                    .expect("cannot parse card number"))
                .collect())
            .collect();
    let count = halves[0].intersection(&halves[1]).count() as u32;
    match count {
        0 => 0,
        _ => 2_u32.pow(count - 1)
    }
}

fn main() {
    let file = File::open(env::args().nth(1)
            .expect("no file specified"))
        .expect("cannot open file");
    let total = BufReader::new(file).lines()
        .map(|line| line.expect("cannot read line"))
        .map(|line| score_card(&line))
        .sum::<u32>();
    println!("Total Score: {}", total);
}

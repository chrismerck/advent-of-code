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

fn score_card_part2(line: &str) -> u32 {
    let mut x = score_card(line);
    if x == 0 {
        return 0;
    }
    let mut count = 1;
    while x & 1 == 0 {
        x >>= 1;
        count += 1;
    }
    count
}

fn main() {
    // Part 1
    let file = File::open(env::args().nth(1)
            .expect("no file specified"))
        .expect("cannot open file");
    let total = BufReader::new(file).lines()
        .map(|line| line.expect("cannot read line"))
        .map(|line| score_card(&line))
        .sum::<u32>();
    println!("Total Part 1 Score: {}", total);

    // Part 2
    let file = File::open(env::args().nth(1)
            .expect("no file specified"))
        .expect("cannot open file");
    let scores : Vec<u32> = BufReader::new(file).lines()
        .map(|line| line.expect("cannot read line"))
        .inspect(|line| println!("{}", line))
        .map(|line| score_card_part2(&line))
        .collect();
    println!("{:?}", scores);
    let mut counts = vec![1; scores.len()];
    for i in 0..scores.len() {
        for j in 1..(scores[i]+1) as usize {
            counts[i + j] += 1 * counts[i];
        }
    }
    println!("{:?}", counts.iter().sum::<u32>());
}

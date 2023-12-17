
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Type {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

const CARDS: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T",
    "J", "Q", "K", "A",
];

fn cmp_card(a: char, b: char) -> std::cmp::Ordering {
    let a = CARDS.iter().position(|&c| c == a.to_string()).unwrap();
    let b = CARDS.iter().position(|&c| c == b.to_string()).unwrap();
    b.cmp(&a)
}

fn cmp_hand_str(a: &str, b: &str) -> std::cmp::Ordering {
    // compare strings by card value
    let a = a.chars().collect::<Vec<char>>();
    let b = b.chars().collect::<Vec<char>>();
    for i in 0..a.len() {
        let cmp = cmp_card(a[i], b[i]);
        if cmp != std::cmp::Ordering::Equal {
            return cmp;
        }
    }
    std::cmp::Ordering::Equal
}

fn cmp_hand(a: &str, b: &str) -> std::cmp::Ordering {
    // compare hands by type
    let a_type = get_hand_type(a);
    let b_type = get_hand_type(b);
    if a_type != b_type {
        return a_type.cmp(&b_type);
    }
    // compare hands by card value
    cmp_hand_str(a, b)
}

use std::collections::HashMap;

fn get_hand_type(hand: &str) -> Type {
    // count number of each card in hand
    let mut card_count: HashMap<char, u8> = HashMap::new();
    for card in hand.chars() {
        let count = card_count.entry(card).or_insert(0);
        *count += 1;
    }
    let mut count_count: HashMap<u8, u8> = HashMap::new();
    for count in card_count.values() {
        let count_count = count_count.entry(*count).or_insert(0);
        *count_count += 1;
    }
    // check for five of a kind
    if count_count.contains_key(&5) {
        return Type::FiveOfAKind;
    }
    // check for four of a kind
    if count_count.contains_key(&4) {
        return Type::FourOfAKind;
    }
    // check for full house
    if count_count.contains_key(&3) && count_count.contains_key(&2) {
        return Type::FullHouse;
    }
    // check for three of a kind
    if count_count.contains_key(&3) {
        return Type::ThreeOfAKind;
    }
    // check for two pair
    if count_count.contains_key(&2) && count_count.get(&2).unwrap() == &2 {
        return Type::TwoPair;
    }
    // check for one pair
    if count_count.contains_key(&2) {
        return Type::OnePair;
    }
    // high card
    Type::HighCard
}

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn main() {
    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    let reader = BufReader::new(file);
    // input lines are: <hand-str> <bid>
    let mut hands_and_bids: Vec<(String, u64)> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<u64>().unwrap();
        hands_and_bids.push((hand.to_string(), bid));
    }
    // sort hands_and_bids by hand
    hands_and_bids.sort_by(|a, b| cmp_hand(&a.0, &b.0));
    hands_and_bids.reverse();
    let mut score = 0;
    for (i, hand_and_bid) in hands_and_bids.iter().enumerate() {
        //println!("{:?} \t {:?}", hand_and_bid, get_hand_type(&hand_and_bid.0));
        let bid = hand_and_bid.1;
        score += (i + 1) as u64 * bid;
    }
    println!("{}", score);
}

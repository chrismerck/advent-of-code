
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

/// each line contains a list of numbers
fn parse_input() -> Vec<Vec<i64>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut numbers: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut line_numbers: Vec<i64> = Vec::new();
        for number in line.split_whitespace() {
            line_numbers.push(number.parse::<i64>().unwrap());
        }
        numbers.push(line_numbers);
    }
    numbers
}

fn differentiate(numbers: &Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for i in 0..numbers.len() - 1 {
        result.push(numbers[i + 1] - numbers[i]);
    }
    result
}

fn predict(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|x| x == &0) {
        0
    } else {
        predict(&differentiate(numbers)) + numbers[numbers.len() - 1]
    }
}

fn predict_2(numbers: &Vec<i64>) -> i64 {
    if numbers.iter().all(|x| x == &0) {
        0
    } else {
        numbers[0] - predict_2(&differentiate(numbers))
    }
}

fn main() {
    let seqs = parse_input();
    let result : i64 = seqs.iter().map(|seq| predict(seq)).sum();
    println!("Part 1: {}", result);
    let result : i64 = seqs.iter().map(|seq| predict_2(seq)).sum();
    println!("Part 2: {}", result);
}

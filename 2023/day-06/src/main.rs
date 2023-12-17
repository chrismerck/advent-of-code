
fn roots(a: f64, b: f64, c: f64) -> (f64, f64) {
    let d = (b * b - 4.0 * a * c).sqrt();
    ((-b + d) / (2.0 * a), (-b - d) / (2.0 * a))
}

fn difference_of_roots(a: f64, b: f64, c: f64) -> f64 {
    let (mut x1, mut x2) = roots(a, b, c);
    if x1 == x1.floor() {
        x1 -= 1.0;
    }
    if x2 == x2.ceil() {
        x2 += 1.0;
    }
    x1.floor() - x2.ceil() + 1.0
}

fn options(t: u64, d: u64) -> u64 {
    let a = 1.0;
    let b = -(t as f64);
    let c = d as f64;
    let diff = difference_of_roots(a, b, c);
    diff as u64
}

use std::io::{self, BufRead};
use std::fs::File;
use std::path::Path;
use std::env;

// read input from specified file
// example:
/*
Time:      7  15   30
Distance:  9  40  200
*/
fn read_input(path: &str) -> Vec<(u64, u64)> {
    let path = Path::new(path);
    let file = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines = reader.lines();
    let time_line = lines.next().unwrap().unwrap();
    let distance_line = lines.next().unwrap().unwrap();
    let times: Vec<u64> = time_line.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let distances: Vec<u64> = distance_line.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    times.into_iter().zip(distances.into_iter()).collect()
}

fn main() {
    let mut acc = 1;
    for (t, d) in read_input(env::args().nth(1).unwrap().as_str()) {
        let opt = options(t, d);
        acc *= opt;
    }
    println!("{}", acc);
}

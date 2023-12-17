

/* EXAMPLE INPUT:
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
*/

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn parse_input() -> (Vec<char>, HashMap<String, (String, String)>) {
    let mut map = HashMap::new();
    let mut instr = Vec::new();
    let file = File::open(env::args().nth(1).unwrap()).unwrap();
    // first line is [RL].*
    let mut lines = BufReader::new(file).lines();
    let mut line = lines.next().unwrap().unwrap();
    for c in line.chars() {
        instr.push(c);
    }
    // skip empty line
    let _ = lines.next();
    // parse the rest
    for line in lines {
        let line = line.unwrap();
        let line = line.chars().filter(|c| c.is_alphabetic() || c.is_whitespace()).collect::<String>();
        let line = line.replace("  ", " ");
        let mut line = line.split(" ").collect::<Vec<&str>>();
        let key = line.remove(0).to_string();
        let value = (line.remove(0).to_string(), line.remove(0).to_string());
        map.insert(key, value);
    }
    (instr, map)
}

fn step(instr: char, map: &HashMap<String, (String, String)>, start: &String) -> String {
    let (left, right) = map.get(start).unwrap();
    match instr {
        'L' => left.clone(),
        'R' => right.clone(),
        _ => panic!("Invalid instruction: {}", instr),
    }
}

fn main() {
    let (instr, map) = parse_input();
    let mut pos = "AAA".to_string();
    let mut step_count = 0;
    loop {
        for i in &instr {
            step_count += 1;
            pos = step(*i, &map, &pos);
            if pos == "ZZZ" {
                println!("Found ZZZ in {} steps", step_count);
                return;
            }
        }
    }
}

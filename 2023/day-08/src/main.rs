

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

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use num::integer::lcm;

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
        let line = line.chars().filter(|c| 
               c.is_alphabetic() 
            || c.is_whitespace()
            || c.is_numeric()
        ).collect::<String>();
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

fn part1(instr: &Vec<char>, map: &HashMap<String, (String, String)>) {
    let mut pos = "AAA".to_string();
    let mut step_count = 0;
    loop {
        for i in instr {
            step_count += 1;
            pos = step(*i, &map, &pos);
            if pos == "ZZZ" {
                println!("Found ZZZ in {} steps", step_count);
                return;
            }
        }
    }
}

/// get the steps to get to xxZ, for any xx, up to cycle
/// 11A --> 11B --> 11Z --> 12Z --> 11A would be [2, 3]
fn get_z_times(instr: &Vec<char>, map: &HashMap<String, (String, String)>, 
           start: &String) -> Vec<u64> {
    let mut pos = start.clone();
    let mut step_count = 0;
    let mut rv = Vec::new();
    let mut seen = HashSet::new();
    // add starting pos to seen
    seen.insert(pos.clone());
    loop {
        for i in instr {
            step_count += 1;
            pos = step(*i, &map, &pos);
            if pos.ends_with("Z") {
                rv.push(step_count);
            }
        }
        if seen.contains(&pos) {
            return rv;
        }
        seen.insert(pos.clone());
    }
}

fn part2(instr: &Vec<char>, map: &HashMap<String, (String, String)>) {
    let mut positions : Vec<String> = 
        map.keys().filter(|k| k.ends_with("A")).map(|k| k.clone()).collect();
    // get z_times for each position
    let mut z_times = Vec::new();
    for pos in &positions {
        let this_z_times = get_z_times(instr, map, pos);
        assert_eq!(this_z_times.len(), 1);
        z_times.push(this_z_times[0]);
    }
    // get lcm of z_times
    let mut acc = z_times[0];
    for i in 1..z_times.len() {
        acc = lcm(acc, z_times[i]);
    }
    println!("Part 2: {}", acc);
}

fn main() {
    let (instr, map) = parse_input();
    println!();
    //part1(&instr, &map);
    part2(&instr, &map);
}

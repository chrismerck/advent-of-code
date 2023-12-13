/**
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
*/

use std::env;
use std::io::{BufRead, BufReader};
use std::fs::File;

#[derive(Debug)]
struct Map {
    source_category: String,
    destination_category: String,
    mapping: Vec<Vec<u64>>,
}

impl Map {
    fn new(source_category: String, destination_category: String, mapping: Vec<Vec<u64>>) -> Map {
        Map {
            source_category: source_category,
            destination_category: destination_category,
            mapping: mapping,
        }
    }

    /// parse a map given header and data lines
    fn parse(lines: &Vec<String>) -> Map {
        let header : Vec<&str> = lines[0].split_whitespace().next()
            .expect("cannot parse map header")
            .split("-to-")
            .collect();
        let source_category = header[0].to_string();
        let destination_category = header[1].to_string();
        let mapping : Vec<Vec<u64>> = lines[1..].iter()
            .map(|line| line.split_whitespace()
                .map(|num| num.parse()
                    .expect("cannot parse map number"))
                .collect())
            .collect();
        Map::new(source_category, destination_category, mapping)
    }

    fn lookup(&self, in_val : u64) -> u64 {
        for rule in self.mapping.iter() {
            let out_start = rule[0];
            let in_start = rule[1];
            let len = rule[2];
            if in_val >= in_start && in_val < in_start + len {
                return out_start + (in_val - in_start);
            }
        }
        in_val
    }
}

fn main() {
    let file = File::open(env::args().nth(1)
            .expect("no file specified"))
        .expect("cannot open file");
    let lines : Vec<String> = BufReader::new(file).lines()
        .map(|line| line.expect("cannot read line"))
        .collect();
    // get seeds list from first line
    let seeds : Vec<u64> = lines[0].split_whitespace()
        .skip(1)
        .map(|num| num.parse()
            .expect("cannot parse seed number"))
        .collect();
    println!("seeds: {:?}", seeds);

    // split on blank lines
    let mut maps = Vec::new();
    let mut map_lines = Vec::new();
    for line in lines[2..].iter() {
        if line.len() == 0 {
            maps.push(Map::parse(&map_lines));
            map_lines = Vec::new();
        } else {
            map_lines.push(line.to_string());
        }
    }
    maps.push(Map::parse(&map_lines));

    println!("maps: {:?}", maps);

    // for each seed, apply each map in order
    let mut locations = Vec::new();
    for seed in seeds.iter() {
        let mut val = *seed;
        println!("seed: {}", val);
        for map in maps.iter() {
            val = map.lookup(val);
            println!("  {} {}", map.destination_category, val);
        }
        locations.push(val);
    }

    println!("min location: {:?}", locations.iter().min().unwrap());

}

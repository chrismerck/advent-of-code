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

#[derive(Debug, PartialOrd, PartialEq, Eq)]
struct Range {
    start: u64,
    len: u64,
}

impl Ord for Range {
    fn cmp(&self, other: &Range) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl Range {
    fn new(start: u64, len: u64) -> Range {
        Range {
            start: start,
            len: len,
        }
    }

    fn intersect(&self, other: &Range) -> Option<Range> {
        let start = if self.start > other.start { self.start } else { other.start };
        let end = if self.end() < other.end() { self.end() } else { other.end() };
        if start < end {
            Some(Range::new(start, end - start))
        } else {
            None
        }
    }

    fn subtract(&self, other: &Range) -> Vec<Range> {
        let mut rv = Vec::new();
        if self.start < other.start {
            rv.push(Range::new(self.start, other.start - self.start));
        }
        if self.end() > other.end() {
            rv.push(Range::new(other.end(), self.end() - other.end()));
        }
        rv
    }

    fn end(&self) -> u64 {
        self.start + self.len
    }
}

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

    /// map a single input range to one or more output ranges
    fn lookup_range(&self, in_range : Range) -> Vec<Range> {
        let mut in_ranges = vec![in_range];
        let mut out_ranges = Vec::new();
        for rule in self.mapping.iter() {
            println!("     rule: {:?}", rule);
            let out_start = rule[0];
            let in_start = rule[1];
            let len = rule[2];
            let rule_in_range = Range::new(in_start, len);
            let mut new_in_ranges = Vec::new();
            for in_range in in_ranges.drain(..) {
                println!("       in_range: {:?}", in_range);
                if let Some(intersect) = in_range.intersect(&rule_in_range) {
                    println!("         intersect: {:?}", intersect);
                    let out_range = Range::new(out_start + intersect.start - in_start, intersect.len);
                    out_ranges.push(out_range);
                    let subtracted = in_range.subtract(&rule_in_range);
                    println!("         subtracted: {:?}", subtracted);
                    new_in_ranges.extend(subtracted);
                } else {
                    println!("         no intersect");
                    new_in_ranges.push(in_range);
                }
            }
            in_ranges = new_in_ranges;
        }
        out_ranges.extend(in_ranges);
        out_ranges
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

    // for each seed, apply each map in order
    let mut locations = Vec::new();
    for seed in seeds.iter() {
        let mut val = *seed;
        for map in maps.iter() {
            val = map.lookup(val);
        }
        locations.push(val);
    }

    println!("Part 1: min location: {:?}", locations.iter().min().unwrap());

    // PART 2

    // parse seed ranges
    let mut ranges = Vec::new();
    for i in 0..seeds.len()/2 {
        let seed_start = seeds[i*2];
        let len = seeds[i*2+1];
        println!("seed_start: {}, len: {}", seed_start, len);
        ranges.push(Range::new(seed_start, len));
    }

    // apply each map in order to the ranges
    for map in maps.iter() {
        println!("");
        println!("Ranges: {:?}", ranges);
        println!("Applying map: {:?}", map);
        let mut new_ranges = Vec::new();
        for range in ranges.drain(..) {
            println!("  range: {:?}", range);
            let mut out_ranges = map.lookup_range(Range::new(range.start, range.len));
            new_ranges.append(&mut out_ranges);
        }
        ranges = new_ranges;
    }

    // find lowest location number in ranges
    println!("Part 2: location: {:?}", ranges.iter().min().unwrap());

}

use std::fs::File;
use std::io::{BufReader, BufRead};
use std::env;
use regex::Regex;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
struct PartNumber {
    number: u32,
    x: i32,
    y: i32,
    length: i32,
}

impl PartNumber {
    /// return an iterator over all the neighbors of this number
    fn neighbors(&self, xmax : i32, ymax : i32) -> Vec<(i32, i32)> {
        let mut neighbors = Vec::new();
        // row above
        for x in self.x - 1..self.x + self.length + 1 {
            neighbors.push((x, self.y - 1));
        }
        // left and right
        neighbors.push((self.x - 1, self.y));
        neighbors.push((self.x + self.length, self.y));
        // row below
        for x in self.x - 1..self.x + self.length + 1 {
            neighbors.push((x, self.y + 1));
        }
        // remove any neighbors that are out of bounds
        neighbors.retain(|(x, y)| *x >= 0 && *x < xmax && *y >= 0 && *y < ymax);
        neighbors
    }
}

struct PartsGrid {
    height: i32,
    width: i32,
    parts: Vec<Vec<bool>>,
    gears: Vec<Vec<bool>>,
    numbers: Vec<PartNumber>,
}

impl PartsGrid {
    fn new() -> PartsGrid {
        PartsGrid {
            height: 0,
            width: 0,
            parts: Vec::new(),
            gears: Vec::new(),
            numbers: Vec::new(),
        }
    }

    fn add_line_parts(&mut self, line: &str) {
        let mut parts = Vec::new();
        let mut gears = Vec::new();
        for c in line.chars() {
            match c {
                // blank
                '.' => parts.push(false),
                // any digit is NOT a part
                '0'..='9' => parts.push(false),
                // any other symbol is a part
                _ => parts.push(true),
            }
            if c == '*' {
                gears.push(true);
            } else {
                gears.push(false);
            }
        }
        self.parts.push(parts);
        self.gears.push(gears);
    }

    fn add_line_numbers(&mut self, line: &str) {
        // find all numbers in the line
        let re = Regex::new(r"\d+").unwrap();
        //let mut numbers = Vec::new();
        for cap in re.captures_iter(line) {
            let cap0 = cap.get(0).unwrap();
            let index = cap0.start();
            let length = cap0.as_str().len() as i32;
            let content = cap0.as_str();
            let x = index as i32;
            let y = self.height as i32;
            let number = content.parse().unwrap();
            self.numbers.push(PartNumber { number, x, y, length });
        }
    }

    fn add_line(&mut self, line: &str) {
        self.add_line_parts(line);
        self.add_line_numbers(line);
        self.height += 1;
        self.width = self.parts[0].len() as i32;
    }

    fn _print(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.parts[y as usize][x as usize] {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }

    fn get_part_locations(&self) -> HashSet<(i32, i32)> {
        // create a HashSet of part locations from parts grid
        (0..self.height).flat_map(|y| (0..self.width)
            .filter(move |x| self.parts[y as usize][*x as usize])
            .map(move |x| (x, y)))
        .collect()
    }

    fn sum_true_parts(&self) -> u32 {
        let part_locations = self.get_part_locations();
        self.numbers.iter()
            .map(|n| (n, n.neighbors(self.width, self.height)))
            .filter(|(_n, neighbors)| 
                neighbors.iter()
                    .any(|(x, y)| 
                        part_locations.contains(&(*x, *y))))
            .map(|(n, _)| n.number)
            .sum()
    }

    fn sum_gear_ratios(&self) -> u32 {
        // get map from parts positions to the numbers neighboring them
        let mut part_numbers : HashMap<(i32, i32), Vec<u32>> = HashMap::new();
        let part_locations = self.get_part_locations();
        for n in &self.numbers {
            for (x, y) in n.neighbors(self.width, self.height) {
                if part_locations.contains(&(x, y)) {
                    part_numbers.entry((x, y)).or_insert(Vec::new()).push(n.number);
                }
            }
        }
        // sum the gear ratios
        part_numbers.iter()
            .filter(|((x, y), _numbers)| self.gears[*y as usize][*x as usize])
            .filter(|(_part, numbers)| numbers.len() == 2)
            .map(|(_part, numbers)| numbers[0] * numbers[1])
            .sum()
    }

}

fn main() {
    let filename = env::args().nth(1).expect("Usage: day-03 <input-file>");
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    // read one, populating the grid
    let mut grid = PartsGrid::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        grid.add_line(&line);
    }
    println!("Sum of True Parts: {}", grid.sum_true_parts());
    println!("Sum of Gear Ratios: {}", grid.sum_gear_ratios());
}

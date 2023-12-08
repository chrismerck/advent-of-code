use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;
use regex::Regex;
use std::error::Error;
use std::collections::HashSet;


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
    numbers: Vec<PartNumber>,
}

impl PartsGrid {
    fn new() -> PartsGrid {
        PartsGrid {
            height: 0,
            width: 0,
            parts: Vec::new(),
            numbers: Vec::new(),
        }
    }

    fn add_line_parts(&mut self, line: &str) {
        let mut parts = Vec::new();
        for c in line.chars() {
            match c {
                // blank
                '.' => parts.push(false),
                // any digit is NOT a part
                '0'..='9' => parts.push(false),
                // any other symbol is a part
                _ => parts.push(true),
            }
        }
        self.parts.push(parts);
    }

    fn add_line_numbers(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
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
        Ok(())
    }

    fn add_line(&mut self, line: &str) {
        self.add_line_parts(line);
        self.add_line_numbers(line);
        self.height += 1;
        self.width = self.parts[0].len() as i32;
    }

    fn print(&self) {
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

    fn sum_true_parts(&self) -> u32 {
        // create a HashSet of part locations from parts grid
        let part_locations : HashSet<(i32, i32)> = 
            (0..self.height).flat_map(|y| (0..self.width)
                .filter(move |x| self.parts[y as usize][*x as usize])
                .map(move |x| (x, y)))
            .collect();
        self.numbers.iter()
            .map(|n| (n, n.neighbors(self.width, self.height)))
            .inspect(|(n, neighbors)| 
                println!("{}: {} {} {} {:?}", 
                    n.number, n.x, n.y, n.length,
                    neighbors))
            .filter(|(n, neighbors)| 
                neighbors.iter()
                    .any(|(x, y)| 
                        part_locations.contains(&(*x, *y))))
            .map(|(n, _)| n.number)
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
    grid.print();
    println!("Sum of True Parts: {}", grid.sum_true_parts());
}

// AoC 2023 -- Day 01

// Read a file and sum the two-digit numbers
// formed by combining the first and last digit
// of each line, which may be the same character.

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let fn_input = std::env::args().skip(1).next().unwrap();
    let f_input = File::open(fn_input).unwrap();
    let mut acc = 0;
    let re = Regex::new(r"^[^\d]*(\d).*?(\d)?[^\d]*$")?;
    for line_result in BufReader::new(f_input).lines() {
        let line = line_result?;
        let caps = re.captures(&line).unwrap();
        let first_digit = &caps[1];
        let second_digit = caps.get(2).map(|m| m.as_str()).unwrap_or(first_digit);
        let num : i32 = format!("{}{}", first_digit, second_digit).parse()?;
        acc += num;
    }
    println!("Total: {}", acc);
    Ok(())
}

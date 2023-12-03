// AoC 2023 -- Day 01

// Read a file and sum the two-digit numbers
// formed by combining the first and last digit
// of each line, which may be the same character.

use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::error::Error;

fn part1() -> Result<(), Box<dyn Error>> {
    let fn_input = std::env::args().skip(1).next().unwrap();
    let f_input = File::open(fn_input).unwrap();
    let mut acc = 0;
    let re = Regex::new(r"^[^\d]*(\d).*?(\d)?[^\d]*$")?;
    for line_result in BufReader::new(f_input).lines() {
        let line = line_result?;
        let caps = re.captures(&line).ok_or(format!("first digit not found in: '{}'", line))?;
        let first_digit = &caps[1];
        let second_digit = caps.get(2).map(|m| m.as_str()).unwrap_or(first_digit);
        let num : i32 = format!("{}{}", first_digit, second_digit).parse()?;
        acc += num;
    }
    println!("Total - Part 1: {}", acc);
    Ok(())
}

fn parse_elf_digit(elf_digit : &str) -> Result<i32, std::num::ParseIntError> {
    Ok(match elf_digit {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => elf_digit.parse()?
    })
}


fn part2() -> Result<(), Box<dyn Error>> {
    let fn_input = std::env::args().skip(1).next().unwrap();
    let f_input = File::open(fn_input).unwrap();
    let mut acc = 0;
    const RE_DIGIT : &str = r"(one|two|three|four|five|six|seven|eight|nine|[\d])";
    let pattern = format!(r"^[^\d]*?{}.*{}", RE_DIGIT, RE_DIGIT);
    let pattern_single = format!(r"{}", RE_DIGIT);
    let re = Regex::new(&pattern)?;
    let re_single = Regex::new(&pattern_single)?;
    for line_result in BufReader::new(f_input).lines() {
        let line = line_result?;
        let caps = re.captures(&line).unwrap_or_else(|| 
            re_single.captures(&line).unwrap_or_else(|| panic!("single pattern failed: '{}'", line))
        );
        let first_digit = &caps.get(1).ok_or(format!("first digit not found in: '{}'", line))?.as_str();
        let second_digit = caps.get(2).map(|m| m.as_str()).unwrap_or(first_digit);
        let num : i32 = format!("{}{}", 
            parse_elf_digit(first_digit)?, 
            parse_elf_digit(second_digit)?
        ).parse()?;
        println!("{} + {} = {}", first_digit, second_digit, num);
        acc += num;
    }
    println!("Total - Part 2: {}", acc);
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1().unwrap_or_else(|e| eprintln!("Error - Part1: {}", e));
    part2()
}
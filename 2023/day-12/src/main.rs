
/*
Input Example:

???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
*/

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;

fn parse_input() -> Vec<(Vec<char>, Vec<usize>)> {
    let mut input = Vec::new();
    let filename = env::args().nth(1).unwrap();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        // split on space
        // first half --> chars
        // second half --> nums (split on ',')
        let mut chars = Vec::new();
        let mut nums = Vec::new();
        let mut parts = line.split(" ");
        let char_part = parts.next().unwrap();
        let num_part = parts.next().unwrap();
        for c in char_part.chars() {
            chars.push(c);
        }
        for n in num_part.split(",") {
            nums.push(n.parse::<usize>().unwrap());
        }
        input.push((chars, nums));
    }
    input
}

fn solve(chars: &[char], nums: &[usize]) -> usize {
    let mut result = 0;
    if chars.len() == 0 {
        if nums.len() == 0 {
            return 1; //return vec![vec![]]; // single empty solution
        } else {
            return 0; // return vec![]; // no solution
        }
    }
    if chars[0] == '.' || chars[0] == '?' {
        result += solve(&chars[1..], &nums);
    }
    if chars[0] == '#' || chars[0] == '?' {
        // we are forced to consume the first num
        if nums.len() == 0 {
            // no solution
        } else if nums[0] > chars.len() {
            // no solution
        } else if chars[0..nums[0]].iter().any(|&c| c == '.') {
            // no solution
        } else if chars.len() == nums[0] {
            result += solve(&chars[nums[0]..], &nums[1..]);
        } else if chars[nums[0]] == '#' {
            // no solution
        } else {
            result += solve(&chars[nums[0] + 1..], &nums[1..]);
        }
    }
    result
}

fn fold(chars: &Vec<char>, nums: &Vec<usize>) -> (Vec<char>, Vec<usize>) {
    let mut new_chars = Vec::new();
    let mut new_nums = Vec::new();
    for i in 0..4 {
        new_nums.extend_from_slice(&nums);
        new_chars.extend_from_slice(&chars);
        new_chars.push('?');
    }
    new_nums.extend_from_slice(&nums);
    new_chars.extend_from_slice(&chars);
    (new_chars, new_nums)
}

fn main() {
    let input = parse_input();
    let mut acc = 0;
    for (chars, nums) in &input {
        let solutions = solve(&chars, &nums);
        println!("Input: {} {:?}", 
            chars.into_iter().collect::<String>(),
            nums);
        println!("{} solutions", solutions);
        /*
        for solution in &solutions {
            println!("   {}", 
                solution.into_iter().collect::<String>());
        }*/
        acc += solutions;
        println!("");
    }
    println!("Part 1: {}", acc);

    // Part 2
    let mut acc = 0;
    for (chars, nums) in &input {
        let (chars, nums) = fold(chars, nums);

        // time this function call to see how long it takes
        let time0 = std::time::Instant::now();
        let solutions = solve(&chars, &nums);
        let time1 = std::time::Instant::now();
        println!("Input: {} {:?}", 
            chars.into_iter().collect::<String>(),
            nums);
        println!("{} solutions", solutions);
        println!("Time: {:?}", time1 - time0);
        /*for solution in &solutions {
            println!("   {}", 
                solution.into_iter().collect::<String>());
        }*/
        acc += solutions;
        println!("");
    }
    println!("Part 2: {}", acc);
}

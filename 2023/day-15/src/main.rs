use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use once_cell::sync::Lazy;

fn hash(input: &str) -> i32 {
    input.as_bytes().to_vec().iter()
        .fold(0, |acc, &x| 
            (acc + x as i32) * 17 % 256)
}

#[derive(Debug)]
enum Operation {
    Add,
    Remove,
}

struct OpCode {
    label: String,
    operation: Operation,
    focal_length: Option<i32>,
}

impl OpCode {
    fn new(opcode_str: &str) -> OpCode {
        static RE: Lazy<Regex> = Lazy::new(|| 
            Regex::new(r"([a-z]+)(-|=)([0-9])?").unwrap());
        let caps = RE.captures(opcode_str).unwrap();
        let label = caps.get(1).unwrap().as_str().to_string();
        let operation = match caps.get(2).unwrap().as_str() {
            "-" => Operation::Remove,
            "=" => Operation::Add,
            _ => panic!("Invalid operation"),
        };
        let focal_length = match caps.get(3) {
            Some(x) => Some(x.as_str().parse::<i32>().unwrap()),
            None => None,
        };
        OpCode {
            label,
            operation,
            focal_length,
        }
    }
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let focal_length = match self.focal_length {
            Some(x) => x.to_string(),
            None => String::from(""),
        };
        write!(f, "{}{}{}", self.label, 
            match self.operation {
                Operation::Add => "=",
                Operation::Remove => "-",
            },
            focal_length)
    }
}

struct Lens {
    label: String,
    focal_length: i32,
}

struct State {
    boxes: [Vec<Lens>; 256],
}

impl State {
    fn new() -> State {
        State {
            boxes: [(); 256].map(|_| Vec::new()),
        }
    }

    fn exec(&mut self, opcode: &OpCode) {
        let focal_length = match opcode.focal_length {
            Some(x) => x,
            None => 0,
        };
        let mut mybox = &mut self.boxes[hash(&opcode.label) as usize];
        match opcode.operation {
            Operation::Add => {
                for lens in mybox.iter_mut() {
                    if lens.label == opcode.label {
                        lens.focal_length = focal_length;
                        return;
                    }
                }
                mybox.push(Lens {
                    label: opcode.label.clone(),
                    focal_length,
                });
            },
            Operation::Remove => {
                mybox.retain(|x| x.label != opcode.label);
            },
        }
    }

    fn print(&self) {
        for (i, mybox) in self.boxes.iter().enumerate() {
            if mybox.len() > 0 {
                println!("Box {}: {}", i, 
                    mybox.iter().map(|x| format!("[{} {}]", x.label, x.focal_length))
                        .collect::<Vec<String>>().join(" "));
            }
        }
    }

    fn focusing_power(&self) -> i32 {
        self.boxes.iter().enumerate()
            .map(|(i, mybox)| {
                mybox.iter().enumerate().map(|(j, x)| {
                    (i + 1) as i32 * (j + 1) as i32 * x.focal_length
                }).sum::<i32>()
            }).sum::<i32>()
    }
}

fn main() {
    let input = BufReader::new(
        File::open(env::args().nth(1).unwrap()).unwrap())
        .lines().next().unwrap().unwrap();
    let result = input.split(',')
        .map(hash)
        .sum::<i32>();
    println!("Part 1: {}", result);

    let mut state = State::new();
    let result = input.split(',')
        .map(OpCode::new)
        .inspect(|x| println!("After \"{}\":", x))
        .for_each(|x| {
            state.exec(&x);
            state.print();
        });
    println!("Part 2: {}", state.focusing_power());
}

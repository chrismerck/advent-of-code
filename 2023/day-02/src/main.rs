/*
Example Input: 
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
*/

use std::fs::File;
use std::io::{self, BufReader, BufRead};
use std::env;

struct Game {
    id: i32,
    sets: Vec<GameSet>,
}

struct GameSet {
    items: Vec<(i32, String)>,
}

impl Game {
    fn parse(line: &str) -> Game {
        let mut parts = line.split(':');
        let game_id: i32 = parts.next().unwrap().split_whitespace().last().unwrap().parse().unwrap();
        let sets_str = parts.next().unwrap();

        let sets = sets_str.split(';')
            .map(GameSet::parse)
            .collect();

        Game { id: game_id, sets }
    }

    fn is_possible(&self) -> bool {
        self.sets.iter().all(GameSet::is_possible)
    }
}

impl GameSet {
    fn parse(set_str: &str) -> GameSet {
        let items = set_str.split(',')
            .map(|num_color| {
                let num_color = num_color.trim();
                let mut parts = num_color.split_whitespace();
                let num: i32 = parts.next().unwrap().parse().unwrap();
                let color = parts.next().unwrap().to_string();
                (num, color)
            })
            .collect();

        GameSet { items }
    }

    fn is_possible(&self) -> bool {
        self.items.iter().all(|&(num, ref color)| check_possible(num, color))
    }
}

fn check_possible(num: i32, color: &str) -> bool {
    match color {
        "red" => num <= 12,
        "green" => num <= 13,
        "blue" => num <= 14,
        _ => panic!("Invalid color: {}", color),
    }
}

fn main() -> io::Result<()> {
    let fn_input = env::args().skip(1).next().expect("Please provide a file name");
    let reader = BufReader::new(File::open(fn_input)?);

    let games: Vec<Game> = reader.lines()
        .map(|line| Game::parse(&line.expect("Error reading line")))
        .collect();

    let sum: i32 = games.iter()
        .filter(|game| game.is_possible())
        .map(|game| game.id)
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}

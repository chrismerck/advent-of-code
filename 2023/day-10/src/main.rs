
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
use std::collections::HashSet;

fn parse_input() -> Vec<Vec<char>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid
}

fn explore(grid: &Vec<Vec<char>>, coord: (usize, usize), path: &mut HashSet<(usize, usize)>) -> u32 {
    let dirs = match grid[coord.0][coord.1] {
        'S' => vec!['N', 'E', 'S', 'W'],
        '|' => vec!['N', 'S'],
        '-' => vec!['E', 'W'],
        '7' => vec!['S', 'W'],
        'L' => vec!['N', 'E'],
        'F' => vec!['S', 'E'],
        'J' => vec!['N', 'W'],
        '.' => vec![],
        _ => panic!("invalid char"),
    };
    let mut next_coords = Vec::new();
    if dirs.contains(&'N') && coord.0 > 0 {
        next_coords.push((coord.0 - 1, coord.1));
    }
    if dirs.contains(&'E') && coord.1 < grid[0].len() - 1 {
        next_coords.push((coord.0, coord.1 + 1));
    }
    if dirs.contains(&'S') && coord.0 < grid.len() - 1 {
        next_coords.push((coord.0 + 1, coord.1));
    }
    if dirs.contains(&'W') && coord.1 > 0 {
        next_coords.push((coord.0, coord.1 - 1));
    }
    for next_coord in next_coords {
        if !path.contains(&next_coord) {
            path.insert(next_coord);
            explore(grid, next_coord, path);
        }
    }
    path.len() as u32
}

use std::thread;

fn main() {
    // find coord of 'S'
    let grid = parse_input();
    let S_coord = grid.iter().enumerate().find_map(|(i, row)| {
        row.iter().enumerate().find_map(|(j, c)| {
            if *c == 'S' {
                Some((i, j))
            } else {
                None
            }
        })
    }).unwrap();

    let mut path = HashSet::new();
    path.insert(S_coord);

    // call explore in new thread with big stack
    let num_rooms = thread::Builder::new().stack_size(32 * 1024 * 1024).spawn(move || {
        explore(&grid, S_coord, &mut path)
    }).unwrap().join().unwrap();

    println!("Part 1: {}", num_rooms / 2);


}

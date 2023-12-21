
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input() -> Vec<Vec<bool>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut grid = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(c == '#');
        }
        grid.push(row);
    }
    grid
}

/// return a new grid expanded
fn expand(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    // start with all rows, cols selected
    let mut rows_to_expand = (0..grid.len()).collect::<Vec<usize>>();
    let mut cols_to_expand = (0..grid[0].len()).collect::<Vec<usize>>();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] {
                rows_to_expand.retain(|&x| x != i);
                cols_to_expand.retain(|&x| x != j);
            }
        }
    }

    // construct new grid
    let mut new_grid = Vec::new();
    for i in 0..grid.len() {
        let mut row = Vec::new();
        for j in 0..grid[0].len() {
            row.push(grid[i][j]);
            if cols_to_expand.contains(&j) {
                row.push(false);
            }
        }
        new_grid.push(row);
        if rows_to_expand.contains(&i) {
            let mut row = Vec::new();
            for _ in 0..new_grid[0].len() {
                row.push(false);
            }
            new_grid.push(row);
        }
    }
    new_grid
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn grid_to_coords(grid: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] {
                coords.push((i, j));
            }
        }
    }
    coords
}

fn taxi_distance(coord1: (usize, usize), coord2: (usize, usize)) -> usize {
    let (x1, y1) = coord1;
    let (x2, y2) = coord2;
    (x1 as i32 - x2 as i32).abs() as usize + (y1 as i32 - y2 as i32).abs() as usize
}

fn sum_of_pairwise_distances(coords: &Vec<(usize, usize)>) -> usize {
    let mut sum = 0;
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            sum += taxi_distance(coords[i], coords[j]);
        }
    }
    sum
}

fn sum_of_pairwise_distances2(grid : &Vec<Vec<bool>>, factor : usize) -> usize {
    let coords = grid_to_coords(&grid);
    // start with all rows, cols selected
    let mut rows_to_expand = (0..grid.len()).collect::<Vec<usize>>();
    let mut cols_to_expand = (0..grid[0].len()).collect::<Vec<usize>>();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] {
                rows_to_expand.retain(|&x| x != i);
                cols_to_expand.retain(|&x| x != j);
            }
        }
    }

    let mut sum = 0;
    for i in 0..coords.len() {
        for j in i+1..coords.len() {
            sum += taxi_distance(coords[i], coords[j]);
            for k in &rows_to_expand {
                if coords[i].0 > *k && coords[j].0 < *k 
                    || coords[i].0 < *k && coords[j].0 > *k {
                    sum += factor;
                }
            }
            for k in &cols_to_expand {
                if coords[i].1 > *k && coords[j].1 < *k 
                    || coords[i].1 < *k && coords[j].1 > *k {
                    sum += factor;
                }
            }
        }
    }
    sum
}

fn main() {
    let grid = parse_input();
    println!("original grid:");
    print_grid(&grid);
    println!("Part 2: sum of pairwise distances: {}", sum_of_pairwise_distances2(&grid, 1000000 - 1));
    println!("expanded grid:");
    let grid = expand(&grid);
    print_grid(&grid);
    let coords = grid_to_coords(&grid);
    println!("coords: {:?}", coords);
    println!("Part 1: sum of pairwise distances: {}", sum_of_pairwise_distances(&coords));
}

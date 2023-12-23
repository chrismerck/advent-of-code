
/*
EXAMPLE FILE:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

*/

use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// input contains grids
/// grids are separated by blank lines
fn parse_input() -> Vec<Vec<Vec<bool>>> {
    BufReader::new(File::open(env::args().nth(1).unwrap()).unwrap())
        .lines()
        .map(|line| line.unwrap())
        .fold(vec![vec![]], |mut acc, line| {
            if line.len() == 0 {
                acc.push(vec![]);
            } else {
                let mut grid = acc.pop().unwrap();
                grid.push(line.chars().map(|c| c == '#').collect());
                acc.push(grid);
            }
            acc
        })
}

fn parse_line(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        for cell in row {
            if *cell {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

/** return list of symmetry lines for given row or column vector
 * [ # .|. # . ] -> [ 2 ]
 * [ #|#|#|#|# ] -> [ 1, 2, 3, 4 ]
 * [ #|# . # . ] -> [ 1 ]
 * [ #|# .|. # ] -> [ 1, 3 ]
 * [ #|# . #|# ] -> [ 1, 4 ]
 */
fn find_vector_symmetries(vec: &Vec<bool>, sym: &mut Vec<usize>) {
    let mut rem_sym = Vec::new();
    for s in sym.iter() {
        for i in 0..*s {
            let j = s + (s - i) - 1;
            if j >= vec.len() {
                continue;
            }
            if vec[i] != vec[j] {
                rem_sym.push(*s);
                break;
            }
        }
    }
    sym.retain(|&s| !rem_sym.contains(&s));
}

fn assert_find_vector_symmetries(line: &str, expected: &Vec<usize>) {
    let vec = parse_line(line);
    let mut sym = (1..vec.len()).collect();
    println!("{} -> {:?}", line, expected);
    find_vector_symmetries(&vec, &mut sym);
    assert_eq!(&sym, expected);
}

#[test]
fn test_find_vector_symmetries() {
    assert_find_vector_symmetries("#..#.", &vec![2]);
    assert_find_vector_symmetries("#####", &vec![1, 2, 3, 4]);
    assert_find_vector_symmetries("##.#.", &vec![1]);
    assert_find_vector_symmetries("##..#", &vec![1, 3]);
    assert_find_vector_symmetries("##.##", &vec![1, 4]);
}

/// return list of vertical and horizontal symmetry lines for given grid
fn find_grid_symmetries(grid: &Vec<Vec<bool>>) -> (Vec<usize>, Vec<usize>) {
    let mut sym_v = (1..grid[0].len()).collect();
    let mut sym_h = (1..grid.len()).collect();
    for row in grid {
        find_vector_symmetries(row, &mut sym_v);
    }
    for col in 0..grid[0].len() {
        let mut vec = Vec::new();
        for row in grid {
            vec.push(row[col]);
        }
        find_vector_symmetries(&vec, &mut sym_h);
    }
    (sym_v, sym_h)
}

fn main() {
    let grid = parse_input();
    println!("{:?}",
        grid.iter()
            .map(find_grid_symmetries)
            .map(|(sym_v, sym_h)| -> usize { (sym_v.iter().sum::<usize>() + 100 * sym_h.iter().sum::<usize>()) })
            .sum::<usize>());
}

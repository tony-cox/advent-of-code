extern crate itertools;
use itertools::Itertools;
use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";
const EXPANSION_RATE: u64 = 1000000;

fn main() {
    let data = get_data();
    part_1(&data);
    part_2(&data);
}

fn expand_space_naive(data: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let all_blank_column_indexes = get_blank_column_indexes(data);

    let new_row_length = data[0].len() + all_blank_column_indexes.len(); // used for making new blank rows

    // now we make a copy of lines one row at a time, but we insert an empty row where required and an empty cell where required
    let mut full_space_lines: Vec<Vec<usize>> = Vec::new();
    for line in data {
        if line.iter().sum::<usize>() == 0 {
            full_space_lines.push([0].repeat(new_row_length));
        }
        full_space_lines.push(
            line.iter()
                .enumerate()
                .flat_map(|(idx, val)| {
                    if all_blank_column_indexes.contains(&idx) {
                        vec![0 as usize, 0 as usize]
                    } else {
                        vec![*val]
                    }
                })
                .collect(),
        );
    }
    full_space_lines
}

fn get_blank_column_indexes(data: &Vec<Vec<usize>>) -> Vec<usize> {
    (0..data.len())
        .filter(|idx| data.iter().all(|line| line[*idx] == 0))
        .collect()
}

fn expand_space_smarter(data: &Vec<Vec<usize>>) -> Vec<Vec<char>> {
    // instead of a grid of usize, we want a grid of characters.
    // 1 is a galaxy (like naive version)
    // 0 is open space (like naive spec)
    // $ is expansion character (do we need separate horizontal/vertical/combined?)
    let all_blank_column_indexes = get_blank_column_indexes(data);
    let new_row_length = data[0].len() + all_blank_column_indexes.len(); // used for making new blank rows

    let mut full_space_lines: Vec<Vec<char>> = Vec::new();
    for line in data {
        if line.iter().sum::<usize>() == 0 {
            // empty space, add a complete row of expansions
            full_space_lines.push(['$'].repeat(new_row_length));
        }
        full_space_lines.push(
            line.iter()
                .enumerate()
                .flat_map(|(idx, val)| {
                    if all_blank_column_indexes.contains(&idx) {
                        vec!['$', '0']
                    } else {
                        vec![char::from_digit(*val as u32, 10).unwrap()]
                    }
                })
                .collect(),
        );
    }
    full_space_lines
}

fn get_data() -> Vec<Vec<usize>> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| parse_line(x))
        .collect()
}

fn part_1(data: &Vec<Vec<usize>>) {
    let data = expand_space_naive(data);
    let mut coords: Vec<(usize, usize)> = Vec::new();
    for (row_idx, row) in data.iter().enumerate() {
        for (col_idx, &val) in row.iter().enumerate() {
            if val > 0 {
                coords.push((row_idx, col_idx));
            }
        }
    }
    let total_distance = coords
        .iter()
        .combinations(2)
        .map(|x| {
            x.first().unwrap().0.abs_diff(x.last().unwrap().0)
                + x.first().unwrap().1.abs_diff(x.last().unwrap().1)
        })
        .sum::<usize>();
    println!(
        "Total distance between all galaxy pairs part 1: {}",
        total_distance
    );
}

fn part_2(raw_data: &Vec<Vec<usize>>) {
    let data = expand_space_smarter(raw_data);
    let mut coords: Vec<(u64, u64)> = Vec::new();
    let mut row_idx: u64 = 0;
    for row in data.iter() {
        let mut col_idx: u64 = 0;
        if row.iter().all(|&x| x == '$') {
            row_idx += EXPANSION_RATE - 1;
        } else {
            // only need to look for galaxies in non-expansion rows
            for &val in row.iter() {
                if val == '$' {
                    col_idx += EXPANSION_RATE - 1;
                } else if val == '1' {
                    coords.push((row_idx, col_idx));
                    col_idx += 1;
                } else {
                    col_idx += 1;
                }
            }
            row_idx += 1;
        }
    }
    let total_distance = coords
        .iter()
        .combinations(2)
        .map(|x| {
            x.first().unwrap().0.abs_diff(x.last().unwrap().0)
                + x.first().unwrap().1.abs_diff(x.last().unwrap().1)
        })
        .sum::<u64>();
    println!(
        "Total distance between all galaxy pairs part 2: {}",
        total_distance
    );
}

fn parse_line(line: &str) -> Vec<usize> {
    line.chars()
        .map(|ch| if ch == '#' { 1 } else { 0 })
        .collect()
}

extern crate itertools;
use itertools::Itertools;
use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";
const PART_1_EXPANSION_RATE: u64 = 2;
const PART_2_EXPANSION_RATE: u64 = 1000000;

fn main() {
    let data = get_data();
    calculate_total_distance(&data, PART_1_EXPANSION_RATE);
    calculate_total_distance(&data, PART_2_EXPANSION_RATE);
}

fn get_blank_column_indexes(data: &Vec<Vec<char>>) -> Vec<usize> {
    (0..data.len())
        .filter(|idx| data.iter().all(|line| line[*idx] == '.'))
        .collect()
}

fn expand_space(data: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    // replace any complete row or col with a row/col of '$' characters
    let all_blank_column_indexes = get_blank_column_indexes(data);
    data.iter()
        .map(|line| {
            if line.iter().all(|&x| x == '.') {
                line.iter().map(|_| '$').collect()
            } else {
                line.iter()
                    .enumerate()
                    .map(|(idx, &val)| {
                        if all_blank_column_indexes.contains(&idx) {
                            '$'
                        } else {
                            val
                        }
                    })
                    .collect()
            }
        })
        .collect()
}

fn get_data() -> Vec<Vec<char>> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| x.chars().collect())
        .collect()
}

fn get_all_galaxy_coords(raw_data: &Vec<Vec<char>>, expansion_rate: u64) -> Vec<(u64, u64)> {
    // this is hard to write as idiomatic rust because of the need to jump the index forward
    // by leaps when hitting an expansion character.  Possible with a combination of folds
    // but it looks messy compared to the imperative version that increments the coords below
    let data = expand_space(raw_data);
    let mut coords: Vec<(u64, u64)> = Vec::new();
    let mut row_idx: u64 = 0;
    for row in data.iter() {
        let mut col_idx: u64 = 0;
        if row.iter().all(|&x| x == '$') {
            row_idx += expansion_rate;
        } else {
            // only need to look for galaxies in non-expansion rows
            for &val in row.iter() {
                if val == '$' {
                    col_idx += expansion_rate;
                } else if val == '#' {
                    coords.push((row_idx, col_idx));
                    col_idx += 1;
                } else {
                    col_idx += 1;
                }
            }
            row_idx += 1;
        }
    }
    coords
}

fn get_distances_between_all_possible_pairs_of_galaxies(coords: &Vec<(u64, u64)>) -> Vec<u64> {
    coords
        .iter()
        .combinations(2)
        .map(|x| {
            x.first().unwrap().0.abs_diff(x.last().unwrap().0)
                + x.first().unwrap().1.abs_diff(x.last().unwrap().1)
        })
        .collect()
}

fn calculate_total_distance(raw_data: &Vec<Vec<char>>, expansion_rate: u64) {
    let coords = get_all_galaxy_coords(raw_data, expansion_rate);
    println!(
        "Total distance between all galaxy pairs with expansion rate of {}: {}",
        expansion_rate,
        get_distances_between_all_possible_pairs_of_galaxies(&coords)
            .iter()
            .sum::<u64>()
    );
}

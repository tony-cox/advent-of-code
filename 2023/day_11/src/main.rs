extern crate itertools;
use itertools::Itertools;
use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";
fn main() {
    let data = get_data();
    part_1(&data);
    part_2(&data);
}

fn get_data() -> Vec<Vec<usize>> {
    let mut lines: Vec<Vec<usize>> = fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| parse_line(x))
        .collect();
    let all_blank_row_indexes: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter(|(idx, line)| line.iter().sum::<usize>() == 0)
        .map(|(idx, line)| idx)
        .collect();
    let all_blank_column_indexes: Vec<usize> = (0..lines.len())
        .filter(|idx| lines.iter().all(|line| line[*idx] == 0))
        .collect();
    let new_row_length = lines[0].len() + all_blank_column_indexes.len(); // used for making new blank rows
                                                                          // now we make a copy of lines one row at a time, but we insert an empty row where required and an empty cell where required
    let mut full_space_lines: Vec<Vec<usize>> = Vec::new();
    for line in lines {
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

fn part_1(data: &Vec<Vec<usize>>) {
    // get all galaxy coords - naive version
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
        .map(|foo| {
            foo.first().unwrap().0.abs_diff(foo.last().unwrap().0)
                + foo.first().unwrap().1.abs_diff(foo.last().unwrap().1)
        })
        .sum::<usize>();
    println!(
        "Total distance between all galaxy pairs: {}",
        total_distance
    );
}

fn part_2(data: &Vec<Vec<usize>>) {
    // println!("Part 2: {}", res);
}

fn parse_line(line: &str) -> Vec<usize> {
    line.chars()
        .enumerate()
        .map(|(i, ch)| if ch == '#' { i + 1 } else { 0 })
        .collect()
}

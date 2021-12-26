use std::collections::HashMap;
use std::fs;
use std::iter;

const INPUT_FILE_NAME: &str = "input";

fn main() {
    let input = get_input();
    part_one(&input);
    part_two(&input);
}

fn get_input() -> Vec<u128> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u128>().unwrap())
        .collect()
}

fn part_one(input: &Vec<u128>) {
    println!("Part One");
    let min_point = (0..input.len()).min_by_key(|&point| get_distance(input, point as u128)).unwrap();
    let shortest_distance = get_distance(input, min_point as u128);
    println!("Shortest distance: {} at point {}", shortest_distance, min_point);
}

fn part_two(input: &Vec<u128>) {
    println!("Part Two");
    let min_point = (0..input.len()).min_by_key(|&point| get_total_triangular_distance_of_all_points(input, point as u128)).unwrap();
    let shortest_distance = get_total_triangular_distance_of_all_points(input, min_point as u128);
    println!("Shortest distance {} at point {}", shortest_distance, min_point as u16);
}

fn triangular_distance(left: u128, right: u128) -> u128 {
    let distance = (if right > left {right - left} else {left - right});
    (1 + distance) * distance / 2
}

fn get_total_triangular_distance_of_all_points(positions: &Vec<u128>, point: u128) -> u128 {
    positions.iter().map(|&x| triangular_distance(x, point)).sum()
}

fn get_distance(positions: &Vec<u128>, point: u128) -> u128 {
    positions.iter().map(|&x| if x > point {x - point} else {point - x}).sum()
}
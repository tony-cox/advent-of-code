use std::collections::HashMap;
use std::fs;

use std::time::Instant;

const INPUT_FILE_NAME: &str = "input.txt";
// const INSTRUCTIONS: &str = "LR";
const INSTRUCTIONS: &str = "LLRRRLRLLRLRRLRLRLRRRLLRRLRRRLRRRLRRRLRRRLRRRLRRLRLLRRRLRRLLRLRLLLRRLRRLRLRLRLRRRLRLRRRLRRLLLRRRLLRRLLRRLLRRRLLLLRLRLRRRLRLRRRLRLLLRLRRLRRRLRRRLRRRLRRRLLRRLLLLRRLLRRLLRRLRLRRRLRRRLRRRLRRLRRRLRRLRRLRRLRLRRRLRRLRRRLRRRLRRLRLRRRLRRLLRLRRLRRRLRLRRLRRRLRRLRRLRRRLLRRRR";

fn main() {
    let start = Instant::now();
    let data = get_data();
    part_1(&data);
    part_2_fast_mode(&data);
    println!("Elapsed seconds: {}", start.elapsed().as_secs_f64());
}

fn part_1(data: &HashMap<String, (String, String)>) {
    let instructions: Vec<char> = INSTRUCTIONS.chars().collect();
    let mut current_location = "AAA";
    let mut num_moves: u32 = 0;
    while current_location != "ZZZ" {
        for instruction in &instructions {
            let (left, right) = data.get(current_location).unwrap();
            if instruction == &'L' {
                current_location = left;
            } else {
                current_location = right;
            }
            num_moves += 1;
            if current_location == "ZZZ" {
                break;
            }
        }
    }
    println!("Number of moves: {}", num_moves);
}

fn part_2_fast_mode(data: &HashMap<String, (String, String)>) {
    let starting_locations: Vec<&str> = data
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| x.as_str())
        .collect();
    let mut total_moves_to_victory: Vec<u64> = Vec::new();
    let instructions: Vec<char> = INSTRUCTIONS.chars().collect();

    for starting_location in starting_locations {
        let mut current_location = starting_location;
        let mut num_moves: u64 = 0;
        while !current_location.ends_with("Z") {
            for instruction in &instructions {
                let (left, right) = data.get(current_location).unwrap();
                if instruction == &'L' {
                    current_location = left;
                } else {
                    current_location = right;
                }
                num_moves += 1;
                if current_location.ends_with("Z") {
                    break;
                }
            }
            if current_location.ends_with("Z") {
                total_moves_to_victory.push(num_moves);
                break;
            }
        }
    }
    let lcm = multi_lcm(&total_moves_to_victory);
    println!("LCM: {}", lcm);
}

fn multi_lcm(numbers: &[u64]) -> u64 {
    numbers
        .iter()
        .skip(1)
        .fold(numbers[0], |acc, num| num::integer::lcm(acc, *num))
}

fn part_2_brute_force(data: &HashMap<String, (String, String)>) {
    let starting_locations: Vec<&str> = data
        .keys()
        .filter(|x| x.ends_with("A"))
        .map(|x| x.as_str())
        .collect();
    let mut current_locations: Vec<&str> = starting_locations.clone();
    let mut num_moves: u128 = 0;
    let instructions: Vec<char> = INSTRUCTIONS.chars().collect();

    loop {
        for instruction in &instructions {
            let mut new_locations: Vec<&str> = Vec::new();
            // move every current location forward by 1 step
            for location in &current_locations {
                let (left, right) = data.get(*location).unwrap();
                if instruction == &'L' {
                    new_locations.push(left);
                } else {
                    new_locations.push(right);
                }
            }
            num_moves += 1;
            if all_locations_end_in_z(&new_locations) {
                println!("Number of moves: {}", num_moves);
                return;
            }
            current_locations = new_locations.clone();
            if (num_moves % 10000000) == 0 {
                println!("Progress: {}", num_moves);
            }
        }
    }
}

fn all_locations_end_in_z(locations: &Vec<&str>) -> bool {
    for location in locations {
        if !location.ends_with("Z") {
            return false;
        }
    }
    true
}

fn get_data() -> HashMap<String, (String, String)> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .skip(2)
        .map(|x| parse_line(x))
        .collect()
}

fn parse_line(line: &str) -> (String, (String, String)) {
    let (name, directions) = line.split_once(" = ").unwrap();
    let (left, right) = directions[1..9].split_once(", ").unwrap();
    (name.to_owned(), (left.to_owned(), right.to_owned()))
}

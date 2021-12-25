use std::fs;
use std::iter;
use std::collections::HashMap;

const INPUT_FILE_NAME: &str = "input";

fn main() {
    let input = get_input();
    part_one(&input);
    part_two(&input);
}

fn get_input() -> Vec<u8> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .next().unwrap()
        .split(",")
        .map(|x| x.parse::<u8>().unwrap())
        .collect()
}


fn part_one(input: &Vec<u8>) {
    let mut school = input.clone();
    println!("Part One");
    println!("Starting school size: {}", school.len());
    age_school(&mut school, 80);
    println!("After 80 days: {}", school.len());
}


fn part_two(input: &Vec<u8>) {
    let mut school = get_initial_counts(input);
    println!("Part Two");
    println!("Starting school size: {}", input.len());
    age_aggregated_counts(&mut school, 256);
    println!("After 256 days: {}", count_aggregated_fish(&school));
}

// region part_one_internals
fn age_school(school: &mut Vec<u8>, num_days: u16) {
    // this is incredibly inefficient, as it mutates a growing vector that can get very large
    // left here and used in part_one, but part two solution is much better
    println!("Num days: {}. Num fish: {}", num_days, school.len());
    if num_days > 0 {
        let num_new_fish = school.iter().cloned().filter(|&x| x == 0).count();
        println!("Num new fish: {}", num_new_fish);
        for fish in school.iter_mut() {
            *fish = progress_fish_timer(fish);
        }
        school.extend(iter::repeat(8).take(num_new_fish));
        age_school(school, num_days - 1);
    }
}


fn progress_fish_timer(fish: &u8) -> u8 {
    match fish {
        0 => 6,
        _ => fish - 1,
    }
}
// endregion

// region part_two_internals
fn get_initial_counts(input: &Vec<u8>) -> HashMap<u8, u128> {
    let mut counter: HashMap<u8, u128> = HashMap::new();
    input.iter().for_each(|&age| *counter.entry(age).or_insert(0) += 1);
    counter
}

fn age_aggregated_counts(school: &mut HashMap<u8, u128>, num_days: u16) {
    println!("Num days remaining: {}. Num fish: {}", num_days, count_aggregated_fish(school));
    if num_days > 0 {
        let num_new_fish =  *school.entry(0).or_insert(0);
        // age all fish of ages 1 to 8 to be ages 0 to 7 respectively
        (1..=8).for_each(|fish_age| {
            let num_fish_in_age = *school.entry(fish_age).or_insert(0);
            school.insert(fish_age - 1, num_fish_in_age);
        });
        // now set previous 0s to be added to 6s, and also create this many new fish as 8s
        *school.entry(6).or_insert(0) += num_new_fish;
        school.insert(8, num_new_fish);
        age_aggregated_counts(school, num_days - 1);
    }
}

fn count_aggregated_fish(school: &HashMap<u8, u128>) -> u128 {
    school.values().sum()
}
// endregion
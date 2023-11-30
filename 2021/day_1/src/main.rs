use std::fs;

const INPUT_FILE_NAME: &str = "input";

fn main() {
    let numbers = get_numbers();
    part_a(&numbers);
    part_b(&numbers);
}

fn get_numbers() -> Vec<u16> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| x.parse::<u16>().unwrap())
        .collect()
}

fn part_a(numbers: &Vec<u16>) {
    let num_increasing = numbers
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();
    println!("Total increasing in part A:\n{}", num_increasing);
}

fn part_b(numbers: &Vec<u16>) {
    let num_increasing = numbers
        .windows(3)
        .collect::<Vec<_>>()
        .windows(2)
        .filter(|x| x[0].iter().sum::<u16>() < x[1].iter().sum::<u16>())
        .count();
    println!("Total increasing in part B:\n{}", num_increasing);
}

use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

const WORDY_DIGITS: [(&'static str, u16); 19] = [
    ("1", 1), 
    ("2", 2), 
    ("3", 3), 
    ("4", 4), 
    ("5", 5), 
    ("6", 6), 
    ("7", 7), 
    ("8", 8), 
    ("9", 9),
    ("0", 0),
    ("one", 1), 
    ("two", 2), 
    ("three", 3), 
    ("four", 4), 
    ("five", 5), 
    ("six", 6), 
    ("seven", 7), 
    ("eight", 8), 
    ("nine", 9)
];

fn main() {
    part_a();
    part_b();
}

fn is_digit(x: &char) -> bool {
    x.to_digit(10).is_some()    
}

fn get_num_from_line(line: &str) -> u16 {
    let first_digit = line.chars().find(|x| is_digit(x)).unwrap();
    let last_digit = line.chars().rev().find(|x| is_digit(x)).unwrap();
    format!("{}{}", first_digit, last_digit).parse::<u16>().unwrap()    
}

fn get_numbers() -> Vec<u16> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()        
        .lines()
        .map(|x|  get_num_from_line(x))
        .collect()
}

fn part_a() {
    let numbers = get_numbers();
    let total: u16 = numbers.iter().sum();
    println!("{}", total);
}

fn part_b() {
    let numbers = get_numbers_including_from_words();
    let total: u16 = numbers.iter().sum();
    println!("{}", total);
}

fn optional_get_max(left: Option<(usize, u16)>, right: Option<(usize, u16)>) -> Option<(usize, u16)>
{
    match left {
        None => right,
        Some((l_idx, _l_val)) => {
            match right {
                None => left,
                Some((r_idx, _r_val)) => if l_idx > r_idx {left} else {right}
            }
        }
    }
}

fn optional_get_min(left: Option<(usize, u16)>, right: Option<(usize, u16)>) -> Option<(usize, u16)>
{    
    match left {
        None => right,
        Some((l_idx, _l_val)) => {
            match right {
                None => left,
                Some((r_idx, _r_val)) => if l_idx < r_idx {left} else {right}
            }
        }
    }
}

fn get_num_from_line_maybe_from_word(line: &str) -> u16 {
    let mut min_idx_val: Option<(usize, u16)> = None;
    let mut max_idx_val: Option<(usize, u16)> = None;    
    for (name, val) in WORDY_DIGITS {
        min_idx_val = optional_get_min(min_idx_val, line.find(name).map(|idx| (idx, val)));
        max_idx_val = optional_get_max(max_idx_val, line.rfind(name).map(|idx| (idx, val)));
    }
    format!("{}{}", min_idx_val.unwrap().1, max_idx_val.unwrap().1).parse::<u16>().unwrap()
}

fn get_numbers_including_from_words() -> Vec<u16> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()        
        .lines()
        .map(|x|  get_num_from_line_maybe_from_word(x))
        .collect()
}
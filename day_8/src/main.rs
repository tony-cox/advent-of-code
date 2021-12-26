use std::collections::{HashMap, HashSet};
use std::fs;

const INPUT_FILE_NAME: &str = "input";

#[derive(Hash, Eq, PartialEq, Debug)]
enum Wire {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

#[derive(Eq, PartialEq, Debug)]
struct Digit {
    wires: HashSet<Wire>,
}

fn main() {
    let input = get_lines();
    part_one(&input);
    part_two(&input);
}

fn get_lines() -> Vec<String> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn part_one(input: &Vec<String>) {
    println!("Part One");
    let digits = get_all_output_digits(input);
    let num_known_a_priori = digits
        .iter()
        .filter(|digit| digit.is_known_a_priori())
        .count();
    println!("Num easy digits: {}", num_known_a_priori);
}

fn part_two(input: &Vec<String>) {
    println!("Part Two");
    let res: u64 = input.iter().map(get_output_value_for_line).sum();
    println!("Total of all output values: {}", res);
}

fn get_all_output_digits(input: &Vec<String>) -> Vec<Digit> {
    input.iter().flat_map(get_output_digits_for_line).collect()
}

fn get_output_value_for_line(line: &String) -> u64 {
    let input_digits = get_input_digits_for_line(line);
    let known_digits = solve_digit_values(&input_digits);
    let output_digits = get_output_digits_for_line(line);
    let output_value: u64 = output_digits
        .iter()
        .map(|digit| char::from_digit(digit.get_value(&known_digits).unwrap() as u32, 10).unwrap())
        .collect::<String>()
        .parse::<u64>()
        .unwrap();
    output_value
}

fn solve_digit_values(digits: &Vec<Digit>) -> HashMap<u8, &Digit> {
    let mut known_digits: HashMap<u8, &Digit> = HashMap::new();
    for digit in digits.iter().cycle() {
        if known_digits.len() == 10 {
            break;
        }
        if let Some(value) = digit.get_value(&known_digits) {
            known_digits.insert(value, digit);
        }
    }
    known_digits
}

fn get_input_digits_for_line(line: &String) -> Vec<Digit> {
    line.split(" | ")
        .next()
        .unwrap()
        .split(" ")
        .map(|wires_str| digit_from_str(wires_str))
        .collect()
}

fn get_output_digits_for_line(line: &String) -> Vec<Digit> {
    line.split(" | ")
        .last()
        .unwrap()
        .split(" ")
        .map(|wires_str| digit_from_str(wires_str))
        .collect()
}

fn digit_from_str(wires_str: &str) -> Digit {
    let wires: HashSet<_> = wires_str.chars().map(|ch| wire_from_char(&ch)).collect();
    Digit { wires }
}

fn wire_from_char(ch: &char) -> Wire {
    match ch {
        'a' => Wire::A,
        'b' => Wire::B,
        'c' => Wire::C,
        'd' => Wire::D,
        'e' => Wire::E,
        'f' => Wire::F,
        'g' => Wire::G,
        _ => panic!("Unknown wire character {}", ch),
    }
}

impl Digit {
    fn a_priori_value(&self) -> Option<u8> {
        match self.wires.len() {
            2 => Some(1),
            4 => Some(4),
            3 => Some(7),
            7 => Some(8),
            _ => None,
        }
    }

    fn is_known_a_priori(&self) -> bool {
        self.a_priori_value().is_some()
    }

    fn get_value(&self, known_digits: &HashMap<u8, &Digit>) -> Option<u8> {
        if let Some((&key, _value)) = known_digits.iter().find(|(_key, &val)| val == self) {
            Some(key) // already known, shortcut out
        } else if let Some(value) = self.a_priori_value() {
            Some(value)
        } else {
            match self.wires.len() {
                5 => self.get_value_from_five_segments(known_digits),
                6 => self.get_value_from_six_segments(known_digits),
                _ => panic!("Expected digit to be known a priori or have either 5 or 6 digits"),
            }
        }
    }

    fn get_value_from_five_segments(&self, known_digits: &HashMap<u8, &Digit>) -> Option<u8> {
        if let Some(seven) = known_digits.get(&7) {
            if self.wraps(seven) {
                return Some(3); // only three fully wraps seven
            }
        }
        if let Some(six) = known_digits.get(&6) {
            if six.wraps(self) {
                return Some(5); // only five is fully wrapped by six
            }
        }
        if known_digits.get(&3).is_some() && known_digits.get(&5).is_some() {
            Some(2)
        } else {
            None
        }
    }

    fn get_value_from_six_segments(&self, known_digits: &HashMap<u8, &Digit>) -> Option<u8> {
        if let Some(four) = known_digits.get(&4) {
            if self.wraps(four) {
                return Some(9); // only nine fully wraps four
            }
        }
        if let Some(seven) = known_digits.get(&7) {
            if known_digits.get(&9).is_some() && self.wraps(seven) {
                return Some(0); // only zero and nine wrap seven, and nine is already known
            }
        }
        if known_digits.get(&9).is_some() && known_digits.get(&0).is_some() {
            Some(6)
        } else {
            None
        }
    }

    fn wraps(&self, other: &Digit) -> bool {
        other.wires.is_subset(&self.wires)
    }
}

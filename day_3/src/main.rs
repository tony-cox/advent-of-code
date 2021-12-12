use std::fs;

const INPUT_FILE_NAME: &str = "input";
const FILE_LENGTH: u16 = 1000;
const WORD_LENGTH: usize = 12;

fn main() {
    let lines = get_lines();
    part_one(&lines);
    part_two(&lines);
}

fn update_counts(mut counts: Vec<u16>, line: &str) -> Vec<u16> {
    line.chars()
        .enumerate()
        .for_each(|(idx, ch)| counts[idx] += ch.to_digit(10).unwrap() as u16);
    counts
}

fn get_lines() -> Vec<String> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}
fn get_counts(lines: &Vec<String>) -> Vec<u16> {
    lines
        .iter()
        .fold(vec![0; WORD_LENGTH], |acc, el| update_counts(acc, &el[..]))
}

fn part_one(lines: &Vec<String>) {
    let counts: Vec<u16> = get_counts(&lines);
    let gamma_rate = counts
        .iter()
        .map(|x| {
            if (*x as f64) / FILE_LENGTH as f64 > 0.5 {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();
    let epsilon_rate = gamma_rate
        .chars()
        .map(|x| if x == '0' { '1' } else { '0' })
        .collect::<String>();
    let power_rate = u32::from_str_radix(&gamma_rate[..], 2).unwrap()
        * u32::from_str_radix(&epsilon_rate[..], 2).unwrap();
    println!(
        "Part One\nGamma Rate: {}\nEpsilon Rate: {}\nPower Rate: {}\n",
        gamma_rate, epsilon_rate, power_rate
    );
}

fn reduce_lines<'a>(lines: &'a Vec<String>, position: usize, keep_majority: bool) -> Vec<String> {
    if lines.len() == 1 {
        return lines.clone();
    }
    assert!(
        position < WORD_LENGTH,
        "unable to reduce lines to a single line from algorithm"
    );
    let counts: Vec<u16> = get_counts(&lines);
    let char_to_keep = if counts[position] as f64 / lines.len() as f64 >= 0.5 {
        if keep_majority {
            '1'
        } else {
            '0'
        }
    } else {
        if keep_majority {
            '0'
        } else {
            '1'
        }
    };
    reduce_lines(
        &lines
            .iter()
            .cloned()
            .filter(|x| x.as_bytes()[position] as char == char_to_keep)
            .collect(),
        position + 1,
        keep_majority,
    )
}

fn part_two(lines: &Vec<String>) {
    let oxygen_rate = &reduce_lines(&lines, 0, true)[0];
    let co2_rate = &reduce_lines(&lines, 0, false)[0];
    let life_support_rate = u32::from_str_radix(&oxygen_rate[..], 2).unwrap()
        * u32::from_str_radix(&co2_rate[..], 2).unwrap();
    println!(
        "Part Two\nOxygen Rate: {}\nCO2 Rate: {}\nLife Support Rate: {}\n",
        oxygen_rate, co2_rate, life_support_rate
    );
}

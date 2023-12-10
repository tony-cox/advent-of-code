use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";
fn main() {
    let data = get_data();
    part_1(&data);
    part_2(&data);
}

fn get_data() -> Vec<Vec<i32>> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| parse_line(x))
        .collect()
}

fn part_1(data: &Vec<Vec<i32>>) {
    let res: i32 = data
        .iter()
        .map(|line| {
            let difference_vectors = get_all_difference_vectors(line);
            calculate_next_value_from_difference_vectors(&difference_vectors)
        })
        .sum();
    println!("Part 1: {}", res);
}

fn part_2(data: &Vec<Vec<i32>>) {
    let res: i32 = data
        .iter()
        .map(|line| {
            let difference_vectors = get_all_difference_vectors(line);
            calculate_previous_value_from_difference_vectors(&difference_vectors)
        })
        .sum();
    println!("Part 2: {}", res);
}

fn get_all_difference_vectors(line: &Vec<i32>) -> Vec<Vec<i32>> {
    std::iter::successors(Some(line.clone()), |last: &Vec<i32>| {
        let differences = calculate_differences(last);
        if differences.iter().any(|x| *x != 0) {
            Some(differences)
        } else {
            None
        }
    })
    .collect()
}

fn calculate_differences(line: &Vec<i32>) -> Vec<i32> {
    line.windows(2).map(|x| x[1] - x[0]).collect()
}

fn calculate_next_value_from_difference_vectors(difference_vectors: &Vec<Vec<i32>>) -> i32 {
    // we could rewrite this and the "previous_value" version to use successors and a fold,
    // but it's far more readable just using a mutable variable.
    let mut next_value_for_current_line: i32 = 0;
    for vec in difference_vectors.iter().rev() {
        next_value_for_current_line += vec.last().unwrap();
    }
    next_value_for_current_line
}

fn calculate_previous_value_from_difference_vectors(difference_vectors: &Vec<Vec<i32>>) -> i32 {
    let mut previous_value_for_current_line: i32 = 0;
    for vec in difference_vectors.iter().rev() {
        previous_value_for_current_line = vec.first().unwrap() - previous_value_for_current_line;
    }
    previous_value_for_current_line
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

use std::collections::HashSet;
use std::convert::TryInto;
use std::fs;
use std::iter::FromIterator;

const INPUT_FILE_NAME: &str = "input";
const SQUARE_SIZE: usize = 5;

struct Board {
    rows: [[u16; SQUARE_SIZE]; SQUARE_SIZE],
}

impl Board {
    fn has_won(&self, numbers: &[u16]) -> bool {
        (0..SQUARE_SIZE).any(|x| {
            Self::is_complete(&self.rows[x], numbers) || Self::is_complete(&self.column(x), numbers)
        })
    }

    fn winning_score_if_winner(&self, numbers: &[u16]) -> Option<u32> {
        if self.has_won(numbers) {
            Some(self.metric_total(numbers))
        } else {
            None
        }
    }

    fn idx_of_winning_number(&self, all_numbers: &[u16]) -> usize {
        (SQUARE_SIZE..all_numbers.len())
            .find(|idx| self.has_won(&all_numbers[0..*idx]))
            .unwrap()
    }

    fn winning_score_when_winner(&self, all_numbers: &[u16]) -> u32 {
        self.winning_score_if_winner(&all_numbers[0..self.idx_of_winning_number(&all_numbers)])
            .unwrap()
    }

    fn column(&self, idx: usize) -> [u16; SQUARE_SIZE] {
        (0..SQUARE_SIZE)
            .map(|i| self.rows[i][idx])
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    fn metric_total(&self, numbers: &[u16]) -> u32 {
        let all_values = HashSet::<_>::from_iter(self.rows.into_iter().flatten());
        let all_numbers = HashSet::<_>::from_iter(numbers.iter().cloned());
        all_values.difference(&all_numbers).sum::<u16>() as u32 * numbers[numbers.len() - 1] as u32
    }

    fn is_complete(row_or_column: &[u16; SQUARE_SIZE], numbers: &[u16]) -> bool {
        HashSet::from(*row_or_column)
            .intersection(&HashSet::<_>::from_iter(numbers.iter().cloned()))
            .count()
            == SQUARE_SIZE
    }
}

fn main() {
    let lines = get_lines();
    let numbers: Vec<u16> = get_numbers(&lines);
    let boards: Vec<Board> = get_boards(&lines);
    part_one(&numbers, &boards);
    part_two(&numbers, &boards);
}

fn part_one(numbers: &Vec<u16>, boards: &Vec<Board>) {
    let final_score_of_first_winner = (SQUARE_SIZE..numbers.len())
        .find_map(|x| {
            boards
                .iter()
                .find_map(|b| b.winning_score_if_winner(&numbers[0..x]))
        })
        .unwrap();
    println!(
        "Winning board found with final score {}",
        final_score_of_first_winner
    )
}

fn part_two(numbers: &Vec<u16>, boards: &Vec<Board>) {
    let final_board = boards
        .iter()
        .max_by_key(|board| board.idx_of_winning_number(&numbers))
        .unwrap();
    println!(
        "Winning board found with final score {}",
        final_board.winning_score_when_winner(&numbers)
    );
}

fn get_lines() -> Vec<String> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

fn get_numbers(lines: &Vec<String>) -> Vec<u16> {
    lines[0]
        .split(",")
        .map(|x| x.parse::<u16>().unwrap())
        .collect()
}

fn get_boards(lines: &Vec<String>) -> Vec<Board> {
    lines[2..]
        .split(|line| line.is_empty())
        .map(build_board)
        .collect()
}

fn build_board(str_array: &[String]) -> Board {
    let rows: [[u16; SQUARE_SIZE]; SQUARE_SIZE] = str_array
        .iter()
        .map(|x| {
            x.split_whitespace()
                .map(|y| y.parse::<u16>().unwrap())
                .collect::<Vec<u16>>()
                .try_into()
                .unwrap()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();
    Board { rows }
}

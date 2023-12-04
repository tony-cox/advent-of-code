use std::collections::HashSet;
use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let game_results = get_game_results();
    println!("Part 1: {}", part1(&game_results));
    println!("Part 2: {}", part2(&game_results));
}

fn get_game_results() -> Vec<usize> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|line| line.split_at(line.find("|").unwrap()))
        .map(|(winning_numbers, held_numbers)| {
            num_matching(
                parse_numbers(&winning_numbers[9..]), // first nine chars id game
                parse_numbers(&held_numbers[1..]),    // first character is a pipe
            )
        })
        .collect()
}

fn parse_numbers(numbers: &str) -> HashSet<u32> {
    numbers
        .split_whitespace()
        .map(|number| number.parse::<u32>().unwrap())
        .collect()
}

fn num_matching(winning_numbers: HashSet<u32>, held_numbers: HashSet<u32>) -> usize {
    winning_numbers.intersection(&held_numbers).count()
}

fn part1(game_results: &Vec<usize>) -> u32 {
    game_results.iter().map(|game| score(*game)).sum()
}

fn part2(game_results: &Vec<usize>) -> usize {
    // the total scratchies is the number of copies + the original set of scratchies
    game_results
        .iter()
        .enumerate()
        .fold(0, |acc, (i, _game_result)| {
            // the +1 here is just so we count the original scratchie as well as the copies it generates
            acc + 1 + get_num_copies(i, &game_results)
        })
}

fn get_num_copies(game_index: usize, all_games: &Vec<usize>) -> usize {
    let game_result = all_games[game_index];
    // recursively gets all copies for a given scratchie - may be finding copies of copies, etc
    (game_index + 1..game_index + game_result + 1).fold(game_result, |acc, x| {
        if x >= all_games.len() {
            // we don't copy scratchies that don't exist
            return acc;
        }
        acc + get_num_copies(x, all_games)
    })
}

fn score(game_result: usize) -> u32 {
    if game_result == 0 {
        0
    } else {
        2u32.pow((game_result - 1) as u32)
    }
}

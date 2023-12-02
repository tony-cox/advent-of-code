use std::collections::HashMap;
use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Debug)]
struct Sample {
    n_red: u16,
    n_green: u16,
    n_blue: u16,
}

#[derive(Debug)]
struct Game {
    id: u16,
    samples: Vec<Sample>,
}

fn main() {
    let games = get_games();
    part_one(&games);
    part_two(&games);
}

fn get_games() -> Vec<Game> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|line| get_game_from_line(line))
        .collect()
}

fn part_one(games: &Vec<Game>) {
    println!("Part One");
    let id_total: u16 = games
        .iter()
        .filter_map(|x| {
            if x.is_valid(12, 13, 14) {
                Some(x.id)
            } else {
                None
            }
        })
        .sum();
    println!("Total: {}", id_total);
}

fn part_two(games: &Vec<Game>) {
    println!("Part Two");
    let power_total: u32 = games.iter().map(|x| x.min_valid_power() as u32).sum();
    println!("Total: {}", power_total);
}

fn get_game_from_line(line: &str) -> Game {
    // the game ID is everything between "Game" and ":"
    let colon_idx = line.find(":").unwrap();
    let id = line[5..colon_idx].parse::<u16>().unwrap();
    let samples: Vec<Sample> = (&line[colon_idx + 1..])
        .split(";")
        .map(|sample_str| get_sample_from_slice(sample_str))
        .collect();
    Game { id, samples }
}

fn get_sample_from_slice(slice: &str) -> Sample {
    // accepts a string slice representing the sample, such as "5 red, 1 green"
    // parses this and returns a Sample
    let n_samples_map: HashMap<&str, u16> = slice
        .split(",")
        .map(|x| x.trim())
        .map(|slice| {
            let (a, b) = slice.split_at(slice.find(" ").unwrap());
            (b.trim(), a.trim().parse::<u16>().unwrap())
        })
        .collect();
    Sample {
        n_red: n_samples_map.get("red").map(|x| *x).unwrap_or(0),
        n_green: n_samples_map.get("green").map(|x| *x).unwrap_or(0),
        n_blue: n_samples_map.get("blue").map(|x| *x).unwrap_or(0),
    }
}

impl Game {
    fn is_valid(&self, max_red: u16, max_green: u16, max_blue: u16) -> bool {
        self.samples.iter().all(|sample| {
            sample.n_red <= max_red && sample.n_green <= max_green && sample.n_blue <= max_blue
        })
    }

    fn min_valid_power(&self) -> u16 {
        let max_red = self.samples.iter().map(|x| x.n_red).max().unwrap();
        let max_green = self.samples.iter().map(|x| x.n_green).max().unwrap();
        let max_blue = self.samples.iter().map(|x| x.n_blue).max().unwrap();
        max_red * max_green * max_blue
    }
}

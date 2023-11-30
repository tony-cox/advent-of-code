use std::fs;

const INPUT_FILE_NAME: &str = "input";

fn main() {
    let commands = get_commands();
    part_one(&commands);
    part_two(&commands);
}

enum Commmand {
    Forward(u32),
    Up(u32),
    Down(u32),
}

fn get_command_from_line(input: &str) -> Commmand {
    let command: Vec<_> = input.split(" ").collect();
    match command[0] {
        "forward" => Commmand::Forward(command[1].parse::<u32>().unwrap()),
        "up" => Commmand::Up(command[1].parse::<u32>().unwrap()),
        "down" => Commmand::Down(command[1].parse::<u32>().unwrap()),
        _ => panic!("Unable to match input to a valid command"),
    }
}

fn get_commands() -> Vec<Commmand> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(get_command_from_line)
        .collect()
}

fn part_one(commands: &Vec<Commmand>) {
    let position = commands.iter().fold((0, 0), |pos, command| match command {
        Commmand::Forward(dist) => (pos.0 + dist, pos.1),
        Commmand::Up(dist) => (pos.0, pos.1 - dist),
        Commmand::Down(dist) => (pos.0, pos.1 + dist),
    });
    println!(
        "Part A\nTotal horizontal: {}\nTotal depth: {}\nMultiplied: {}\n",
        position.0,
        position.1,
        position.0 * position.1
    );
}

fn part_two(commands: &Vec<Commmand>) {
    let position = commands
        .iter()
        .fold((0, 0, 0), |pos, command| match command {
            Commmand::Forward(dist) => (pos.0 + dist, pos.1 + (dist * pos.2), pos.2),
            Commmand::Up(dist) => (pos.0, pos.1, pos.2 - dist),
            Commmand::Down(dist) => (pos.0, pos.1, pos.2 + dist),
        });
    println!(
        "Part B\nTotal horizontal: {}\nTotal depth: {}\nMultiplied: {}\n",
        position.0,
        position.1,
        position.0 * position.1
    );
}

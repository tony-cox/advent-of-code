use std::fs;

use std::time::Instant;

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(PartialEq, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(PartialEq, Debug)]
enum Pipe {
    Horizontal,
    Vertical,
    NorthEastCorner,
    NorthWestCorner,
    SouthEastCorner,
    SouthWestCorner,
    Unknown,
    Ground,
}

fn main() {
    let start = Instant::now();
    let data = get_data();
    let coords_in_loop = part_1(&data);
    part_2(&data, &coords_in_loop);
    println!("Elapsed seconds: {}", start.elapsed().as_secs_f64());
}

fn get_data() -> Vec<Vec<Pipe>> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| parse_line(x))
        .collect()
}

fn parse_line(line: &str) -> Vec<Pipe> {
    line.chars().map(|x| char_to_pipe(x)).collect()
}

fn char_to_pipe(c: char) -> Pipe {
    match c {
        '-' => Pipe::Horizontal,
        '|' => Pipe::Vertical,
        '7' => Pipe::NorthEastCorner,
        'F' => Pipe::NorthWestCorner,
        'J' => Pipe::SouthEastCorner,
        'L' => Pipe::SouthWestCorner,
        'S' => Pipe::Unknown,
        _ => Pipe::Ground,
    }
}

fn part_2(data: &Vec<Vec<Pipe>>, coords_in_loop: &Vec<(usize, usize)>) {
    let mut count: usize = 0;
    // we start on the "outside" for each column
    for (northing, row) in data.iter().enumerate() {
        let mut inside: bool = false; // we start on the "outside" for each row
        let mut previous_switching_pipe: Option<Pipe> = None;
        for (easting, pipe) in row.iter().enumerate() {
            if coords_in_loop.contains(&(northing, easting)) {
                match pipe {
                    Pipe::Horizontal => {
                        // horizontal pipes never switch
                        // inside_columnwise[easting] = !inside_columnwise[easting];
                    }
                    Pipe::Vertical => {
                        // vertical pipes always switch
                        inside = !inside;
                        previous_switching_pipe = Some(Pipe::Vertical)
                    }
                    Pipe::NorthWestCorner => {
                        inside = !inside;
                        previous_switching_pipe = Some(Pipe::NorthWestCorner);
                    }
                    Pipe::NorthEastCorner => {
                        // northeast is incompatible with southwest
                        if previous_switching_pipe != Some(Pipe::SouthWestCorner) {
                            inside = !inside;
                            previous_switching_pipe = Some(Pipe::NorthEastCorner);
                        }
                    }
                    Pipe::SouthWestCorner => {
                        inside = !inside;
                        previous_switching_pipe = Some(Pipe::SouthWestCorner);
                    }
                    Pipe::SouthEastCorner => {
                        // southeast is incompatible with northwest
                        if previous_switching_pipe != Some(Pipe::NorthWestCorner) {
                            inside = !inside;
                            previous_switching_pipe = Some(Pipe::SouthEastCorner);
                        }
                    }
                    Pipe::Unknown => {
                        // fill this in depending on what type the "S" is in the data set
                        // current example: northeast
                        if previous_switching_pipe != Some(Pipe::SouthWestCorner) {
                            inside = !inside;
                            previous_switching_pipe = Some(Pipe::NorthEastCorner);
                        }
                    }
                    _ => {
                        panic!("Unexpected pipe type {:?}", pipe);
                    }
                }
            } else {
                if inside {
                    count += 1;
                }
            }
        }
    }
    println!("Part 2 count: {}", count);
}

fn part_1(data: &Vec<Vec<Pipe>>) -> Vec<(usize, usize)> {
    let starting_coords = find_unknown_coords(data);
    // start by looking to the east east because we can see by looking at data that this will
    // work in both the example and our input data (cheating a bit)
    let mut coords_in_loop: Vec<(usize, usize)> = vec![(starting_coords.0, starting_coords.1)];
    let mut current_coords = (starting_coords.0, starting_coords.1 + 1);
    let mut num_steps = 1;
    let mut from_direction = Direction::West;
    while current_coords != starting_coords {
        coords_in_loop.push(current_coords);
        let current_pipe = &data[current_coords.0][current_coords.1];
        match (current_pipe, &from_direction) {
            (Pipe::Horizontal, Direction::East) => {
                current_coords.1 -= 1;
            }
            (Pipe::Horizontal, Direction::West) => {
                current_coords.1 += 1;
            }
            (Pipe::Vertical, Direction::North) => {
                current_coords.0 += 1;
            }
            (Pipe::Vertical, Direction::South) => {
                current_coords.0 -= 1;
            }
            (Pipe::NorthEastCorner, Direction::West) => {
                current_coords.0 += 1;
                from_direction = Direction::North;
            }
            (Pipe::NorthEastCorner, Direction::South) => {
                current_coords.1 -= 1;
                from_direction = Direction::East;
            }
            (Pipe::NorthWestCorner, Direction::East) => {
                current_coords.0 += 1;
                from_direction = Direction::North;
            }
            (Pipe::NorthWestCorner, Direction::South) => {
                current_coords.1 += 1;
                from_direction = Direction::West;
            }
            (Pipe::SouthEastCorner, Direction::West) => {
                current_coords.0 -= 1;
                from_direction = Direction::South;
            }
            (Pipe::SouthEastCorner, Direction::North) => {
                current_coords.1 -= 1;
                from_direction = Direction::East;
            }
            (Pipe::SouthWestCorner, Direction::East) => {
                current_coords.0 -= 1;
                from_direction = Direction::South;
            }
            (Pipe::SouthWestCorner, Direction::North) => {
                current_coords.1 += 1;
                from_direction = Direction::West;
            }
            _ => panic!(
                "Unexpected pipe type {:?} found at coords {},{} from direction {:?}",
                current_pipe, current_coords.0, current_coords.1, &from_direction
            ),
        };
        num_steps += 1;
    }
    println!("Part 1 total steps in loop: {}", num_steps);
    println!("Number of steps to halfway: {}", num_steps / 2);
    coords_in_loop
}

fn find_unknown_coords(data: &Vec<Vec<Pipe>>) -> (usize, usize) {
    for (y, row) in data.iter().enumerate() {
        for (x, pipe) in row.iter().enumerate() {
            if *pipe == Pipe::Unknown {
                return (y, x);
            }
        }
    }
    panic!("No unknown coords found");
}

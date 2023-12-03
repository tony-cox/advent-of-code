use std::collections::HashMap;
use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

struct CartesianNumber {
    value: u32,
    row_num: usize,
    column_num: usize,
}

fn main() {
    let grid = get_grid();
    let cartesian_numbers = get_cartesian_numbers(&grid);
    let engine_parts: Vec<u32> = cartesian_numbers
        .iter()
        .filter(|x| x.is_engine_part(&grid))
        .map(|x| x.value).collect();

    println!("Part 1: {}", engine_parts.into_iter().sum::<u32>());
    // for part 2, we need to map specific asterisk coords to vector of engine parts
    let mut parts_by_asterisk_coords: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    cartesian_numbers.iter().for_each(|cartesian_number| {
        if let Some(coords) = cartesian_number.adjacent_asterisk_coords(&grid) {
            parts_by_asterisk_coords.entry(coords).or_default().push(cartesian_number.value)
        }
    });
    let gear_ratios: u32 = parts_by_asterisk_coords.values().map(
        |parts| if parts.len() == 2 {
            parts.iter().product::<u32>()
        } else {
            0
        }
    ).sum();    
    println!("Part 2: {}", gear_ratios);

}

fn get_grid() -> Vec<Vec<char>> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_cartesian_numbers(grid: &Vec<Vec<char>>) -> Vec<CartesianNumber> {
    let mut cartesian_numbers: Vec<CartesianNumber> = Vec::new();
    for (row_num, row) in grid.iter().enumerate() {
        let mut current_number_word: Option<String> = None;
        for (column_num, cell) in row.iter().enumerate() {
            if cell.is_digit(10) {
                current_number_word =
                    Some(current_number_word.unwrap_or(String::new()) + &cell.to_string());
            } else {
                if current_number_word.is_some() {
                    let value = current_number_word.unwrap().parse::<u32>().unwrap();
                    cartesian_numbers.push(CartesianNumber {
                        value,
                        row_num,
                        column_num: (column_num - value.to_string().len()),
                    });
                    current_number_word = None;
                }
            }
        }
        // check if there's a current word at the end of loop - end of line word edge case
        if current_number_word.is_some() {
            let value = current_number_word.unwrap().parse::<u32>().unwrap();
            cartesian_numbers.push(CartesianNumber {
                value,
                row_num,
                column_num: (row.len() - value.to_string().len()),
            });
        }
    }
    cartesian_numbers
}


impl CartesianNumber {
    fn first_row_to_search(&self) -> usize {
        if self.row_num > 0 {
            self.row_num - 1
        } else {
            self.row_num
        }
    }

    fn last_row_to_search(&self, grid: &Vec<Vec<char>>) -> usize {
        if self.row_num < grid.len() - 1 {
            if self.row_num + 2 > grid.len()
            {
                panic!("bad index");
            }
            self.row_num + 2
        } else {
            self.row_num + 1
        }
    }

    fn first_column_to_search(&self) -> usize {
        if self.column_num > 0 {
            self.column_num - 1
        } else {
            self.column_num
        }
    }

    fn last_column_to_search(&self, row: &Vec<char>) -> usize {
        let final_column_of_word = self.column_num + self.value.to_string().len() - 1;
        if final_column_of_word < row.len() - 1 {
            final_column_of_word + 2
        } else {
            if final_column_of_word + 1 > row.len() { 
                panic!("bad index")
            }
            final_column_of_word + 1
        }
    }


    fn is_engine_part(&self, grid: &Vec<Vec<char>>) -> bool {
        for row_num in self.first_row_to_search()..self.last_row_to_search(grid) {
            let row = &grid[row_num];
            for column_num in self.first_column_to_search()..self.last_column_to_search(row) {
                let candidate = &row[column_num];
                if !candidate.is_numeric() && candidate != &'.' {
                    return true;
                }
            }
        }
        false
    }

    fn adjacent_asterisk_coords(&self, grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
        for row_num in self.first_row_to_search()..self.last_row_to_search(grid) {
            let row = &grid[row_num];
            for column_num in self.first_column_to_search()..self.last_column_to_search(row) {
                let candidate = &row[column_num];
                if candidate == &'*' {
                    return Some((row_num, column_num));
                }
            }
        }
        None
    }
}

use std::fs;

use std::time::Instant;

const INPUT_FILE_NAME: &str = "input.txt";

struct Pattern {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>, // just the rows transposed for convenience
}

fn main() {
    let start = Instant::now();
    let data = get_data();
    part_1(&data);
    println!("Elapsed seconds: {}", start.elapsed().as_secs_f64());
}

fn get_data() -> Vec<Pattern> {
    let raw_data: Vec<Vec<char>> = fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut data: Vec<Pattern> = Vec::new();
    let mut current_pattern_data: Vec<Vec<char>> = Vec::new();
    for line in raw_data {
        if line.len() == 0 {
            data.push(Pattern::new(current_pattern_data));
            current_pattern_data = Vec::new();
        } else {
            current_pattern_data.push(line);
        }
    }
    if current_pattern_data.len() > 0 {
        data.push(Pattern::new(current_pattern_data)); // Push the last pattern
    }
    data
}

fn part_1(data: &Vec<Pattern>) {
    let row_reflections = data.iter().filter_map(|pattern| find_reflective_index(&pattern.rows)).sum::<usize>();
    let column_reflections = data.iter().filter_map(|pattern| find_reflective_index(&pattern.columns)).sum::<usize>();
    println!("Part 1: {}", 100 * row_reflections + column_reflections);
}

fn is_reflection(char_vectors: &[Vec<char>]) -> bool {
    if char_vectors.len() % 2 != 0 {
        // an odd number of vectors can't be reflected
        return false;
    }
    let mid = char_vectors.len() / 2;
    for i in 0..mid {
        if char_vectors[i] != char_vectors[char_vectors.len() - 1 - i] {
            return false;
        }
    }
    true
}

fn find_reflective_index(possibly_reflected_chars: &Vec<Vec<char>>) -> Option<usize> {
    // start with a window that is size row.len() and see if there is a reflection.
    // if not, decrease the window size by 1 and move through the rows again.  Continue until
    // window size is 2 and then return None if no reflection is found.
    let initial_window_size = possibly_reflected_chars.len();
    for window_size in (2..=initial_window_size).rev() {
        let left_window = possibly_reflected_chars.windows(window_size).next().unwrap();
        if is_reflection(left_window) {
            // the number of rows to the "left" of the line of reflection is the reflective index.  
            // this will be equal to window size / 2
            return Some(window_size / 2);
        }
        let right_window = possibly_reflected_chars.windows(window_size).last().unwrap();
        if is_reflection(right_window) {
            // the number of rows to the "left" of the line of reflection is the reflective index.  
            // this will be equal to window size / 2 plus any rows to the left of the window
            return Some(possibly_reflected_chars.len() - window_size / 2);
        }
    }
    None
}

impl Pattern {
    fn new(rows: Vec<Vec<char>>) -> Pattern {
        let columns = Pattern::transpose(&rows);
        Pattern { rows, columns }
    }

    fn transpose(rows: &Vec<Vec<char>>) -> Vec<Vec<char>> {
        let mut n_columns = rows[0].len();

        let mut columns: Vec<Vec<char>> = vec![vec![]; n_columns]; // Initialize column vectors

        for (i, row) in rows.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                columns[j].push(ch);
            }
        }
        columns
    }
}

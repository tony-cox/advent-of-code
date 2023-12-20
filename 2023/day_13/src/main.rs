use std::fs;

use std::time::Instant;

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(Clone)]
struct Pattern {
    rows: Vec<Vec<char>>,
    columns: Vec<Vec<char>>, // just the rows transposed for convenience
}

fn main() {
    let start = Instant::now();
    let data = get_data();
    part_1(&data);
    part_2(&data);
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
    let reflection_summary = data
        .iter()
        .map(|pattern| pattern.find_first_reflection_value())
        .sum::<usize>();
    println!("Part 1: {}", reflection_summary);
}

fn part_2(data: &Vec<Pattern>) {
    let reflection_summary = data
        .iter()
        .map(|pattern| pattern.find_first_reflection_value_with_smudges())
        .sum::<usize>();
    println!("Part 2: {}", reflection_summary);
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

fn find_reflective_index(
    possibly_reflected_chars: &Vec<Vec<char>>,
    skip_index: Option<usize>,
) -> Option<usize> {
    // todo: this is finding the first reflection when we do the smudging,
    // which could actually be the same reflection line even though it's smudged.
    // we need to somehow have the ability to "continue" looking for a reflection
    // when it finds the same one.

    // start with a window that is size 2 and see if there is a reflection on the left edge.
    // if not, increase the window size by 1 try again.  Continue until
    // window size is the entire vector
    let max_window_size = possibly_reflected_chars.len();
    for window_size in (2..=max_window_size) {
        let left_window = possibly_reflected_chars
            .windows(window_size)
            .next()
            .unwrap();
        if is_reflection(left_window) {
            // the number of rows to the "left" of the line of reflection is the reflective index.
            // this will be equal to window size / 2
            let idx = window_size / 2;
            if skip_index.is_some() && idx == skip_index.unwrap() {
                continue;
            }
            return Some(idx);
        }
    }

    // then repeat this process, starting from the right edge with the "largest" window size,
    // slowly decreasing the window size until it is 2
    for window_size in (2..=max_window_size).rev() {
        if window_size == 2 {
            // println!("here");
        }
        let right_window = possibly_reflected_chars
            .windows(window_size)
            .last()
            .unwrap();
        if is_reflection(right_window) {
            // the number of rows to the "left" of the line of reflection is the reflective index.
            // this will be equal to window size / 2 plus any rows to the left of the window
            let idx = possibly_reflected_chars.len() - window_size / 2;
            if skip_index.is_some() && idx == skip_index.unwrap() {
                continue;
            }
            return Some(idx);
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

    fn find_first_reflection_value(&self) -> usize {
        let row_reflection = find_reflective_index(&self.rows, None);
        if row_reflection.is_some() {
            return row_reflection.unwrap() * 100;
        }

        let column_reflection = find_reflective_index(&self.columns, None);
        if column_reflection.is_some() {
            return column_reflection.unwrap();
        }

        panic!("Unable to find any reflection for pattern!")
    }

    fn find_first_reflection_value_with_smudges(&self) -> usize {
        // make a temporary pattern which is a copy of this pattern but with exactly one cell
        // smudged.
        // then find the reflection value for this pattern if it exists
        // return this value if it exists, otherwise smudge another cell and try again
        // repeat until a reflection value is found
        let original_reflection_value = self.find_first_reflection_value();
        let row_skip = if original_reflection_value >= 100 {
            Some(original_reflection_value / 100)
        } else {
            None
        };
        let column_skip = if original_reflection_value < 100 {
            Some(original_reflection_value)
        } else {
            None
        };
        let mut smudged_pattern = self.clone();
        for i in 0..self.rows.len() {
            for j in 0..self.rows[i].len() {
                if i == 8 && j == 15 {
                    // println!("here");
                }
                if smudged_pattern.rows[i][j] == '.' {
                    // smudge the i/j cell from . to #
                    smudged_pattern.rows[i][j] = '#';
                    smudged_pattern.columns[j][i] = '#';
                    // see if the smudged pattern has a reflection
                    let row_reflection = find_reflective_index(&smudged_pattern.rows, row_skip);
                    if row_reflection.is_some()
                        && row_reflection.unwrap() * 100 != original_reflection_value
                    {
                        return row_reflection.unwrap() * 100;
                    }

                    let column_reflection =
                        find_reflective_index(&smudged_pattern.columns, column_skip);
                    if column_reflection.is_some()
                        && column_reflection.unwrap() != original_reflection_value
                    {
                        return column_reflection.unwrap();
                    }
                    // put the smudge back how it was
                    smudged_pattern.rows[i][j] = '.';
                    smudged_pattern.columns[j][i] = '.';
                } else {
                    // smudge the i/j cell from # to .
                    smudged_pattern.rows[i][j] = '.';
                    smudged_pattern.columns[j][i] = '.';
                    // see if the smudged pattern has a reflection
                    let row_reflection = find_reflective_index(&smudged_pattern.rows, row_skip);
                    if row_reflection.is_some()
                        && row_reflection.unwrap() * 100 != original_reflection_value
                    {
                        return row_reflection.unwrap() * 100;
                    }

                    let column_reflection =
                        find_reflective_index(&smudged_pattern.columns, column_skip);
                    if column_reflection.is_some()
                        && column_reflection.unwrap() != original_reflection_value
                    {
                        return column_reflection.unwrap();
                    }
                    // put the smudge back how it was
                    smudged_pattern.rows[i][j] = '#';
                    smudged_pattern.columns[j][i] = '#';
                }
            }
        }
        panic!("Unable to find a new reflection for pattern after smudging every single character!")
    }
}

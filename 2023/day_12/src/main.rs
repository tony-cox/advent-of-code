extern crate itertools;
use itertools::Itertools;

use core::num;
use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

fn main() {
    let data = get_data();
    part_1(&data);
    part_2(&data);
}

fn get_data() -> Vec<(Vec<char>, Vec<usize>)> {
    // returns the data in the format ???.### 1,1,3
    // as two vectors, one with ???.### (as chars) and one with 1,1,3 (as usize)
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| {
            let sides = x.split_whitespace().collect::<Vec<&str>>();
            (
                sides[0].chars().collect(),
                sides[1]
                    .split(",")
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect(),
            )
        })
        .collect()
}

fn validate_condition_record(condition_record: &Vec<char>, insertion_indexes: &Vec<usize>, groupings: &Vec<usize>) -> bool {
    // generate the actual candidate string from insertion indexes
    let mut num_questions_seen = 0;
    let mut candidate: Vec<char> = Vec::new();
    for c in condition_record.iter() {
        if *c == '?' {
            if insertion_indexes.contains(&num_questions_seen) {
                candidate.push('#');
            } else {
                candidate.push('.');
            }
            num_questions_seen += 1;
        } else {
            candidate.push(*c);
        }
    }

    // now check if the candidate satisfies the groupings
    let mut it = candidate.into_iter();
    for grouping in groupings {
        let mut count = 0;  // distance we are through grouping
        loop {
            match it.next() {
                Some('#') => {
                    count += 1;
                    if count == *grouping {
                        // this is tricky - we need to enforce that the next item in the iterator is a '.' or None
                        // if it's not, then we have a grouping that is too long, so return false
                        match it.next() {
                            Some('.') => {
                                break;
                            },
                            Some(_) => {
                                return false;
                            },
                            None => {
                                break;
                            }
                        }
                    }
                },
                Some('.') => {
                    if count > 0 {
                        break;  // end of the group
                    } else {
                        // we're at start of group, so just keep going until we find a hash
                        continue;
                    }
                },
                Some(_) => {
                    panic!("Unexpected character in candidate string");
                },
                None => {
                    return false;
                }
            }
        }
        
    }
    // valid iterator will only have '.' characters after we have broken from the loop after the final group
    // so walk through the rest of the iterator until the end, and if we find any non-'.' characters, return 
    // false
    for c in it {
        if c != '.' {
            return false;
        }
    }

    true
    
}

fn calculate_number_of_possible_fits(
    condition_record: &Vec<char>,
    groupings: &Vec<usize>,
) -> usize {
    // this is the guts of the task. It takes a condition record (e.g. ???.###)
    // and a grouping (e.g. 1,1,3) and returns the number of possible fits such that
    // the ? characters could be replaced by # characters in a way that the # characters
    // are grouped in the way specified by the grouping, in the specified order given.  
    // Each grouping needs to be separated by at least one . character.
    // can we just generate all possible ways to replace the ? characters with # characters such that
    // the total number of # characters is equal to the sum of the grouping, and then check if the
    // grouping is satisfied? I think so.
    let total_number_of_hashes = groupings.iter().sum::<usize>();
    let number_of_hashes_to_place = total_number_of_hashes - condition_record.iter().fold(0, |acc, x| {
        if *x == '#' {
            acc + 1
        } else {
            acc
        }
    });
    let total_unknowns = condition_record.iter().fold(0, |acc, x| {
        if *x == '?' {
            acc + 1
        } else {
            acc
        }
    });
    // generate all possible ways to place the hashes
    let combinations = (0..total_unknowns)
        .combinations(number_of_hashes_to_place)
        .collect::<Vec<Vec<usize>>>();
    // now check if the combinations satisfy the grouping
    let valid_combinations = combinations.iter().filter(|x| validate_condition_record(condition_record, x, groupings));
    valid_combinations.count()

}

fn part_1(data: &Vec<(Vec<char>, Vec<usize>)>) {
    let total_number_of_possible_fits =
        data.iter().fold(0, |acc, (condition_records, groupings)| {
            let number_of_possible_fits =
                calculate_number_of_possible_fits(condition_records, groupings);
            // println!(
            //     "Condition record {:?} with groupings {:?} has {} possible fits",
            //     condition_records, groupings, number_of_possible_fits);
            acc + number_of_possible_fits
        });
    println!("Part 1: {}", total_number_of_possible_fits);
}

fn part_2(data: &Vec<(Vec<char>, Vec<usize>)>) {
    // TODO: Implement part 2 logic here
}

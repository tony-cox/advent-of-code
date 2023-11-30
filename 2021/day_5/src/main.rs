use std::collections::HashMap;
use std::fs;

const INPUT_FILE_NAME: &str = "input";

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u16,
    y: u16,
}

impl Point {
    fn from_delimited_str(input: &str) -> Point {
        // takes input such as 409,872 and creates a Point{x: 409, y: 872}
        let x_y: Vec<_> = input
            .split(",")
            .map(|x| x.parse::<u16>().unwrap())
            .collect();
        assert!(x_y.len() == 2, "expected exactly two coordinates for point");
        Point {
            x: x_y[0],
            y: x_y[1],
        }
    }

    fn is_horizontally_aligned(&self, other: &Point) -> bool {
        self.y == other.y
    }

    fn is_vertically_aligned(&self, other: &Point) -> bool {
        self.x == other.x
    }
}

fn main() {
    let input = get_input();
    part_one(&input);
    part_two(&input);
}

fn horizontal_points(start: Point, end: Point) -> Vec<Point> {
    (if start.x < end.x {
        start.x..=end.x
    } else {
        end.x..=start.x
    })
    .map(|x| Point { x, y: start.y })
    .collect()
}

fn vertical_points(start: Point, end: Point) -> Vec<Point> {
    (if start.y < end.y {
        start.y..=end.y
    } else {
        end.y..=start.y
    })
    .map(|y| Point { x: start.x, y })
    .collect()
}

fn diagonal_points(start: Point, end: Point) -> Vec<Point> {
    // it would be nice if we didn't have to collect the ranges into vectors as this is inefficient
    let horizontal_traverse: Vec<_> = if start.x < end.x {
        (start.x..=end.x).collect()
    } else {
        (end.x..=start.x).rev().collect()
    };
    let vertical_traverse: Vec<_> = if start.y < end.y {
        (start.y..=end.y).collect()
    } else {
        (end.y..=start.y).rev().collect()
    };
    horizontal_traverse
        .into_iter()
        .zip(vertical_traverse.into_iter())
        .map(|(x, y)| Point { x, y })
        .collect()
}

fn points_from_input_line(input: &str, count_diagonals: bool) -> Vec<Point> {
    let point_strs: Vec<&str> = input.split(" -> ").collect();
    assert_eq!(point_strs.len(), 2, "expected exactly two points for line");
    let start = Point::from_delimited_str(point_strs[0]);
    let end = Point::from_delimited_str(point_strs[1]);
    if start.is_horizontally_aligned(&end) {
        horizontal_points(start, end)
    } else if start.is_vertically_aligned(&end) {
        vertical_points(start, end)
    } else if count_diagonals {
        diagonal_points(start, end)
    } else {
        Vec::new()
    }
}

fn count_and_print(points: Vec<Point>) {
    let mut counter: HashMap<&Point, u16> = HashMap::new();
    points.iter().for_each(|point| *counter.entry(point).or_insert(0) += 1);
    let res = counter.values().filter(|count| **count > 1).count();
    println!("Result: {}", res);
}

fn part_one(input: &Vec<String>) {
    println!("Part One");
    let points: Vec<_> = input
        .iter()
        .map(|x| points_from_input_line(x.as_str(), false))
        .flatten()
        .collect();
    count_and_print(points);
}

fn part_two(input: &Vec<String>) {
    println!("Part Two");
    let points: Vec<_> = input
        .iter()
        .map(|x| points_from_input_line(x.as_str(), true))
        .flatten()
        .collect();
    count_and_print(points);
}

fn get_input() -> Vec<String> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect()
}

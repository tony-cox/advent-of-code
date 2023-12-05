use std::collections::HashSet;
use std::fs;

const INPUT_FILE_NAME: &str = "input.txt";

struct ConciseOffsetMap {
    source_start: i64,
    destination_start: i64,
    range_length: usize,
    offset: i64,
    source_end: i64,
    destination_end: i64,
}

fn main() {
    let (seeds, maps_by_type) = get_data();
    part_1(&seeds, &maps_by_type)
}

fn find_map_for_source(source: i64, maps: &Vec<ConciseOffsetMap>) -> Option<&ConciseOffsetMap> {
    for map in maps {
        if map.source_in_range(source) {
            return Some(map);
        }
    }
    None
}

fn get_destination_for_source(source: i64, maps: &Vec<ConciseOffsetMap>) -> i64 {
    if let Some(map) = find_map_for_source(source, maps) {
        map.get_destination(source).unwrap()
    } else {
        // Any source numbers that aren't mapped correspond to the same destination number
        source
    }
}

fn get_location_for_seed(seed: &i64, maps_by_type: &Vec<Vec<ConciseOffsetMap>>) -> i64 {
    let mut source = *seed;
    for map_type in maps_by_type {
        source = get_destination_for_source(source, map_type);
    }
    // print out the seed and final destination
    println!("{} -> {}", seed, source);
    source
}

fn part_1(seeds: &Vec<i64>, maps_by_type: &Vec<Vec<ConciseOffsetMap>>) {
    let min_location = seeds
        .iter()
        .map(|x| get_location_for_seed(x, maps_by_type))
        .min()
        .unwrap();
    println!("Min Location: {}", min_location);
}

fn parse_line_into_map(line: &str) -> Option<ConciseOffsetMap> {
    let mut split = line.split_whitespace();
    let destination_start = split.next()?.parse::<i64>().ok()?;
    let source_start = split.next()?.parse::<i64>().ok()?;
    let range_length = split.next()?.parse::<usize>().ok()?;
    Some(ConciseOffsetMap::new(
        source_start,
        destination_start,
        range_length,
    ))
}

fn get_data() -> (Vec<i64>, Vec<Vec<ConciseOffsetMap>>) {
    let full_text = fs::read_to_string(INPUT_FILE_NAME).unwrap();
    let first_line = full_text.lines().next().unwrap();
    let seeds: Vec<i64> = full_text.lines().next().unwrap()[7..]
        .split_whitespace()
        .map(|x| x.parse::<i64>().unwrap())
        .collect();
    let mut maps: Vec<Vec<ConciseOffsetMap>> = Vec::new();
    let mut map_type_idx = 0;
    maps.push(Vec::new());
    for line in full_text.lines().skip(3) {
        if line.is_empty() {
            // current index is finished.
            // Sort maps at current index by source_start and increment index
            maps[map_type_idx].sort_by(|a, b| a.source_start.cmp(&b.source_start));
            map_type_idx += 1;
            maps.push(Vec::new());
        } else {
            if let Some(map) = parse_line_into_map(line) {
                maps[map_type_idx].push(map);
            }
        }
    }
    (seeds, maps)
}

impl ConciseOffsetMap {
    fn new(source_start: i64, destination_start: i64, range_length: usize) -> Self {
        // offset is destination_start - source_start; source_end is source_start + range_length, etc
        let offset = destination_start - source_start;
        let source_end = source_start + range_length as i64;
        let destination_end = destination_start + range_length as i64;
        ConciseOffsetMap {
            source_start,
            destination_start,
            range_length,
            offset,
            source_end,
            destination_end,
        }
    }

    fn source_in_range(&self, source: i64) -> bool {
        source >= self.source_start && source < self.source_end
    }

    fn get_destination(&self, source: i64) -> Option<i64> {
        if self.source_in_range(source) {
            Some(source + self.offset)
        } else {
            None
        }
    }
}

use std::fs;
use std::time::Instant;
const INPUT_FILE_NAME: &str = "input.txt";

struct ConciseOffsetMap {
    source_start: i64,
    offset: i64,
    source_end: i64,
}

fn main() {
    let (seeds, maps_by_type) = get_data();
    println!("Part 1");
    part_1(&seeds, &maps_by_type);
    println!("Part 2");
    let start = Instant::now();
    let part_2_seeds = split_seeds_into_ranges_for_part_2(&seeds);
    println!("Number of seeds to map: {}", part_2_seeds.len());
    part_1(&part_2_seeds, &maps_by_type);
    println!("Elapsed seconds: {}", start.elapsed().as_secs());
}

fn split_seeds_into_ranges_for_part_2(seeds: &Vec<i64>) -> Vec<i64> {
    // first split seed vector into pairs of tuples of seeds with range
    let pairs: Vec<(i64, i64)> = seeds.chunks(2).map(|chunk| (chunk[0], chunk[1])).collect();
    pairs
        .iter()
        .flat_map(|(start_seed, range)| (*start_seed..=*start_seed + *range))
        .collect()
}
fn find_map_for_source(source: i64, maps: &Vec<ConciseOffsetMap>) -> Option<&ConciseOffsetMap> {
    let found_map_idx_res = maps.binary_search_by(|map| {
        if source < map.source_start {
            // this map is too big, i.e. greater than the source
            std::cmp::Ordering::Greater
        } else if source >= map.source_end {
            // this map is too small, i.e. less than the source
            std::cmp::Ordering::Less
        } else {
            // map found
            std::cmp::Ordering::Equal
        }
    });
    if let Ok(found_map_idx) = found_map_idx_res {
        Some(&maps[found_map_idx])
    } else {
        None
    }
}

fn get_destination_for_source(source: i64, maps: &Vec<ConciseOffsetMap>) -> i64 {
    if let Some(map) = find_map_for_source(source, maps) {
        map.get_destination(source)
    } else {
        source
    }
}

fn get_location_for_seed(seed: &i64, maps_by_type: &Vec<Vec<ConciseOffsetMap>>) -> i64 {
    let mut source = *seed;
    for map_type in maps_by_type {
        source = get_destination_for_source(source, map_type);
    }
    source
}

fn part_1(seeds: &Vec<i64>, maps_by_type: &Vec<Vec<ConciseOffsetMap>>) {
    let min_location = seeds
        .iter()
        .map(|seed| get_location_for_seed(seed, maps_by_type))
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
    // the final map hasn't been sorted yet
    maps[map_type_idx].sort_by(|a, b| a.source_start.cmp(&b.source_start));
    (seeds, maps)
}

impl ConciseOffsetMap {
    fn new(source_start: i64, destination_start: i64, range_length: usize) -> Self {
        // offset is destination_start - source_start; source_end is source_start + range_length, etc
        let offset = destination_start - source_start;
        let source_end = source_start + range_length as i64;
        ConciseOffsetMap {
            source_start,
            offset,
            source_end,
        }
    }

    fn get_destination(&self, source: i64) -> i64 {
        source + self.offset
    }
}

const INPUT_PAIRS: [(u64, u64); 4] = [(53, 250), (91, 1330), (67, 1081), (68, 1025)];

const PART_2_INPUT_PAIR: (u64, u64) = (53916768, 250133010811025);

fn main() {
    let part_1_result: u64 = INPUT_PAIRS
        .into_iter()
        .map(|(time, distance)| {
            calculate_all_winning_button_press_durations(time, distance).len() as u64
        })
        .product();
    println!("Part 1 result: {}", part_1_result);
    println!(
        "Part 2 result: {}",
        calculate_all_winning_button_press_durations(PART_2_INPUT_PAIR.0, PART_2_INPUT_PAIR.1)
            .len()
    );
}

fn calculate_all_winning_button_press_durations(
    total_race_time: u64,
    record_distance: u64,
) -> Vec<u64> {
    (1..total_race_time)
        .map(|i| get_race_distance_for_button_press_time(i, total_race_time))
        .filter(|distance| distance > &record_distance)
        .collect()
}

fn get_race_distance_for_button_press_time(button_press_time: u64, total_race_time: u64) -> u64 {
    let travel_time = total_race_time - button_press_time;
    button_press_time * travel_time
}

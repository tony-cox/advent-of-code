extern crate itertools;
use itertools::Itertools;
use std::fs;
use std::time::Instant;

const INPUT_FILE_NAME: &str = "input.txt";

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Debug)]
struct PokerHand {
    hand_type: HandType,
    fallback_rank: u32,
    bid: u16,
    cards: [u16; 5],
}

fn main() {
    let start = Instant::now();

    let mut hands = get_hands();
    hands.sort();
    // print out each hand as a string now that they are sorted to check the sort algorithm
    // for hand in &hands {
    //     println!("{:?}", hand);
    // }

    let scores: Vec<u32> = hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            let rank = index as u32 + 1;
            rank * hand.bid as u32
        })
        .collect();

    // for score in &scores {
    //     println!("Score: {}", score);
    // }

    let sum_of_scores: u32 = scores.iter().sum();
    println!("Sum of scores: {}", sum_of_scores);
    println!("Elapsed seconds: {}", start.elapsed().as_secs_f64());
}

fn get_hands() -> Vec<PokerHand> {
    fs::read_to_string(INPUT_FILE_NAME)
        .unwrap()
        .lines()
        .map(|x| get_hand_from_line(x))
        .collect()
}

fn get_hand_from_line(line: &str) -> PokerHand {
    let (card_strings, bid) = line.split_at(6);
    let card_strings = card_strings.trim();
    let bid = bid.parse::<u16>().unwrap();
    let cards: [u16; 5] = card_strings
        .chars()
        .map(|x| get_card_as_u16(&x))
        .collect::<Vec<u16>>()
        .try_into()
        .unwrap();
    PokerHand::new(cards, bid, true)
}

fn get_card_as_u16(card: &char) -> u16 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        _ => card.to_digit(10).unwrap() as u16,
    }
}

fn get_hand_type_from_cards(cards: &[u16; 5]) -> HandType {
    let counts = cards.iter().fold([0; 15], |mut counts, &card| {
        counts[card as usize] += 1;
        counts
    });

    let pair_count = counts.iter().filter(|&&count| count == 2).count();
    let three_of_a_kind = counts.iter().any(|&count| count == 3);
    let four_of_a_kind = counts.iter().any(|&count| count == 4);
    let five_of_a_kind = counts.iter().any(|&count| count == 5);

    match (pair_count, three_of_a_kind, four_of_a_kind, five_of_a_kind) {
        (0, false, false, true) => HandType::FiveOfAKind,
        (0, false, true, false) => HandType::FourOfAKind,
        (1, true, false, false) => HandType::FullHouse, // Add FullHouse case
        (0, true, false, false) => HandType::ThreeOfAKind,
        (1, false, false, false) => HandType::OnePair,
        (2, false, false, false) => HandType::TwoPairs,
        (_, _, _, _) => HandType::HighCard,
    }
}

fn get_hand_type_from_cards_considering_jacks_as_wild(cards: &[u16; 5]) -> HandType {
    let mut max_hand_type = get_hand_type_from_cards(&cards);

    // Check if there are any jacks in the hand
    if cards.contains(&11) {
        let wild_jack_indices: Vec<usize> = cards
            .iter()
            .enumerate()
            .filter(|(_, &card)| card == 11)
            .map(|(index, _)| index)
            .collect();

        // Generate all possible permutations of wild card replacements
        let permutations = generate_permutations(wild_jack_indices.len(), 14);

        // Iterate through each permutation
        for permutation in permutations {
            let mut candidate_cards = cards.clone();

            // Replace the wild jacks with the cards from the permutation
            for (i, &index) in wild_jack_indices.iter().enumerate() {
                candidate_cards[index] = permutation[i];
            }

            // Get the hand type for the candidate cards
            let candidate_hand_type = get_hand_type_from_cards(&candidate_cards);

            // Update the max hand type if necessary
            if candidate_hand_type > max_hand_type {
                max_hand_type = candidate_hand_type;
            }
        }
    }

    max_hand_type
}

fn generate_permutations(length: usize, max_value: u16) -> Vec<Vec<u16>> {
    (1..=length)
        .map(|_| 1..=max_value)
        .multi_cartesian_product()
        .collect()
}

fn get_fallback_rank_from_cards(cards: &[u16; 5], jacks_are_wild: bool) -> u32 {
    let card_string = cards
        .iter()
        .map(|&card| {
            // if the card is 11, it is a jack, so it has value 0
            if card == 11 && jacks_are_wild {
                return format!("{:X}", 0);
            }
            format!("{:X}", card)
        })
        .collect::<String>();
    u32::from_str_radix(&card_string, 16).unwrap()
}

impl PokerHand {
    fn new(cards: [u16; 5], bid: u16, jacks_are_wild: bool) -> PokerHand {
        let hand_type = {
            if jacks_are_wild {
                get_hand_type_from_cards_considering_jacks_as_wild(&cards)
            } else {
                get_hand_type_from_cards(&cards)
            }
        };

        PokerHand {
            hand_type: hand_type,
            fallback_rank: get_fallback_rank_from_cards(&cards, jacks_are_wild),
            bid,
            cards,
        }
    }
}

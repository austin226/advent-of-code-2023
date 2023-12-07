use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::common::get_input;

const JOKER_VALUE: usize = 0;
const CARD_NAMES: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfKind = 3,
    FullHouse = 4,
    FourOfKind = 5,
    FiveOfKind = 6,
}

#[derive(Debug, Eq)]
struct Hand {
    hand_values: Vec<usize>,
    bid: u32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let my_hand_type = self.hand_type();
        let other_hand_type = other.hand_type();
        if my_hand_type == other_hand_type {
            self.hand_values.cmp(&other.hand_values)
        } else {
            my_hand_type.cmp(&other_hand_type)
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_values == other.hand_values
    }
}

impl Hand {
    fn card_freqs(&self) -> HashMap<usize, u8> {
        self.hand_values
            .iter()
            .copied()
            .fold(HashMap::new(), |mut map, val| {
                *map.entry(val).or_default() += 1;
                map
            })
    }

    fn hand_type(&self) -> HandType {
        let card_freqs = self.card_freqs();
        let n_jokers = *card_freqs.get(&JOKER_VALUE).unwrap_or(&0);

        let mut freqs = self
            .card_freqs()
            .values()
            .sorted()
            .map(|f| *f)
            .collect_vec();

        for _ in 0..n_jokers {
            // Add a joker to the most frequent card's count, and remove the least frequent card.
            let last_freq_idx = freqs.len() - 1;
            freqs[last_freq_idx] += 1;

            if freqs[0] == 1 {
                freqs.remove(0);
            } else {
                freqs[0] -= 1;
            }
        }

        match freqs[..] {
            [5] => HandType::FiveOfKind,
            [1, 4] => HandType::FourOfKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }
}

pub fn run() {
    let input = get_input("src/day7/input0.txt");

    let card_values_map: HashMap<char, usize> = CARD_NAMES
        .iter()
        .enumerate()
        .map(|(i, c)| (*c, i))
        .collect();

    let mut hands = Vec::new();
    for line in input {
        let split = line.split_ascii_whitespace().collect_vec();
        let hand_values = split[0].chars().map(|c| card_values_map[&c]).collect_vec();
        let bid = split[1].parse::<u32>().unwrap();
        hands.push(Hand { hand_values, bid });
    }
    hands.sort();

    let total_winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = i as u32 + 1;
            hand.bid * rank
        })
        .sum();

    println!("{:?}", total_winnings);
}

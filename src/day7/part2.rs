use std::cmp::Ordering;
use std::collections::HashMap;

use itertools::Itertools;

use crate::common::get_input;

const JOKER_VALUE: usize = 0;
const CARD_NAMES: [char; 13] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A',
];

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
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

    fn non_joker_freqs(&self) -> Vec<u8> {
        self.card_freqs()
            .into_iter()
            .filter(|f| f.0 != JOKER_VALUE)
            .sorted_by(|&a, &b| a.0.cmp(&b.0))
            .map(|f| f.1)
            .collect_vec()
    }

    fn hand_type(&self) -> HandType {
        let card_freqs = self.card_freqs();
        let n_jokers = *card_freqs.get(&JOKER_VALUE).unwrap_or(&0);

        // Collect the frequencies of non-joker cards
        let mut freqs = self.non_joker_freqs();
        if n_jokers == 0 {
            debug_assert_eq!(freqs.len(), self.card_freqs().len());
        } else if n_jokers == 1 {
            debug_assert_eq!(freqs.len(), self.card_freqs().len() - 1);
        }

        println!("Hand: {:?} - freqs is {:?}", self, freqs);
        for _ in 0..n_jokers {
            if freqs.len() == 0 {
                // hand is only jokers
                freqs.push(1);
            } else {
                // Add a joker to the most frequent non-joker card's count.
                *freqs.last_mut().unwrap() += 1;
            }
            println!("Processed joker - freqs is now {:?}", freqs);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_freqs() {
        for (hand_values, freqs) in [
            (vec![0, 0, 0, 0, 0], HashMap::from([(0, 5)])),
            (vec![0, 0, 0, 0, 1], HashMap::from([(0, 4), (1, 1)])),
            (vec![0, 0, 0, 1, 1], HashMap::from([(0, 3), (1, 2)])),
            (vec![0, 0, 0, 1, 2], HashMap::from([(0, 3), (1, 1), (2, 1)])),
        ] {
            let hand = Hand {
                hand_values,
                bid: 1,
            };
            assert_eq!(
                hand.card_freqs(),
                freqs,
                "hand={:?}, expected freqs={:?}",
                hand,
                freqs
            );
        }
    }

    #[test]
    fn test_non_joker_freqs() {
        for (hand_values, freqs) in [
            (vec![0, 0, 0, 0, 0], vec![]),
            (vec![0, 0, 0, 0, 1], vec![1]),
            (vec![0, 0, 0, 1, 1], vec![2]),
            (vec![0, 0, 0, 1, 2], vec![1, 1]),
        ] {
            let hand = Hand {
                hand_values,
                bid: 1,
            };
            assert_eq!(
                hand.non_joker_freqs(),
                freqs,
                "hand={:?}, expected freqs={:?}",
                hand,
                freqs
            );
        }
    }

    #[test]
    fn test_hand_type() {
        for (hand_values, hand_type) in [
            (vec![0, 0, 0, 0, 0], HandType::FiveOfKind),
            (vec![0, 0, 0, 0, 1], HandType::FiveOfKind),
            (vec![0, 0, 0, 1, 1], HandType::FiveOfKind),
            (vec![0, 0, 0, 1, 2], HandType::FourOfKind),
        ] {
            let hand = Hand {
                hand_values,
                bid: 1,
            };
            assert_eq!(
                hand.hand_type(),
                hand_type,
                "hand={:?}, expected type={:?}",
                hand,
                hand_type
            );
        }
    }
}

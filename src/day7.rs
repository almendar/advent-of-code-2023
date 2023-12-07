use std::{cmp::Ordering, collections::HashMap};

use crate::common::fold_on_each_line1;

#[derive(Debug)]
struct Hand {
    cards: String,
    // cards_value: i64,
    hand_type_val: i64,
    bid: i64,
    // counts: HashMap<char, i64>,
}

fn hand_counts(hand: &str) -> HashMap<char, i64> {
    let mut map = HashMap::new();
    hand.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    map
}

fn ranks(with_joker: bool) -> HashMap<char, i64> {
    let mut ranks =
        if with_joker {
            vec![
                'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J',
            ]
        } else {
            vec![
                'A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2',
            ]
        };

    ranks.reverse();
    ranks
        .iter()
        .enumerate()
        .map(|x| (*x.1, x.0 as i64))
        .collect()
}

fn card_to_type(cards: &str) -> i64 {
    let counts = hand_counts(&cards);
    match counts.keys().len() {
        5 => 0, //High card
        4 => 1, //One pair
        1 => 6, //Five of a kind
        2 => match counts.values().max().unwrap() {
            4 => 5, //four of a count
            3 => 4, //full house
            _ => panic!("Wrong 2")
        },
        3 => match counts.values().max().unwrap() {
            3 => 3, // three of a kind
            2 => 2, // two pair
            _ => panic!("wrong 3")
        }
        _ => panic!("Shit")
    }
}


fn folder(line: String) -> Hand {
    match line.split_once(" ") {
        Some((cards, bid)) => {
            let bid = bid.parse().expect("works");
            let cards = cards.to_owned();
            let hand_type_val = card_to_type(&cards);
            Hand { bid, cards, hand_type_val }
        }
        None => panic!("Nope"),
    }
}

fn compare_hands(h1: &Hand, h2: &Hand) -> Ordering {
    {
        let ranks = ranks(false);
        match h1.hand_type_val.cmp(&h2.hand_type_val) {
            Ordering::Equal => {
                for i in 0..5 {
                    let left = h1.cards.chars().nth(i).unwrap();
                    let right = h2.cards.chars().nth(i).unwrap();
                    let left_val = ranks.get(&left).unwrap();
                    let right_val = ranks.get(&right).unwrap();
                    match left_val.cmp(right_val) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                }
                panic!("Shit");
            }
            other => other,
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut cards = fold_on_each_line1(input, folder).expect("Should be able to parse");
    cards.sort_by(compare_hands);
    cards.iter().enumerate().map(|(idx, h)| {
        (idx as i64 + 1) * h.bid
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day7() {
        // parse_card("QQQJA");
        let part1_sample = part1("input/day7_sample.txt");
        assert_eq!(6440, part1_sample);
        let part1_input = part1("input/day7_input.txt");

        assert_eq!(250120186, part1_input);
        println!("day7 part1 sample:{}, input: {}", part1_sample, part1_input);

        // let part2_sample = part2("input/day7_sample.txt");
        // assert_eq!(71503, part2_sample);
        // let part2_input = part2("input/day7_input.txt");
        // assert_eq!(45128024, part2_input);
        // println!("day7 part2 sample {}, input: {}", part2_sample, part2_input);
    }
}

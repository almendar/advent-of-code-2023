use std::{cmp::Ordering, collections::HashMap};

use crate::common::fold_on_each_line1;

#[derive(Debug)]
struct Hand {
    cards: String,
    hand_type_val: i64,
    bid: i64,
}

fn hand_counts(hand: &str) -> HashMap<char, i64> {
    let mut map = HashMap::new();
    hand.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    map
}

fn ranks(with_joker: bool) -> HashMap<char, i64> {
    let mut ranks = if with_joker {
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

fn card_to_type(cards: &str, joker: bool) -> i64 {
    let mut counts = hand_counts(&cards);
    if joker {
        if let Some(js) = counts.get(&'J').map(|x| x.clone()) {
            if 0 < js && js < 5 {
                counts.remove(&'J');
                let (key, _) = counts.iter().max_by_key(|x| x.1).unwrap();
                let key = key.clone();
                *counts.get_mut(&key).unwrap() += js;
            }
        }
    }

    match counts.keys().len() {
        5 => 0, //High card
        4 => 1, //One pair
        1 => 6, //Five of a kind
        2 => match counts.values().max().unwrap() {
            4 => 5, //four of a count
            3 => 4, //full house
            err => panic!("Wrong 2 {} {} {:?} ", err, cards, counts),
        },
        3 => match counts.values().max().unwrap() {
            3 => 3, // three of a kind
            2 => 2, // two pair
            err => panic!("wrong 3 {} {} {:?}", err, cards, counts),
        },
        _ => panic!("This should neven happen"),
    }
}

fn folder(line: String, with_joker: bool) -> Hand {
    match line.split_once(" ") {
        Some((cards, bid)) => {
            let bid = bid.parse().expect("works");
            let cards = cards.to_owned();
            let hand_type_val = card_to_type(&cards, with_joker);
            Hand {
                bid,
                cards,
                hand_type_val,
            }
        }
        None => panic!("Nope folder"),
    }
}

fn compare_hands(h1: &Hand, h2: &Hand, with_joker: bool) -> Ordering {
    {
        let ranks = ranks(with_joker);
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
                panic!("No expecting this");
            }
            other => other,
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut cards =
        fold_on_each_line1(input, |l| folder(l, false)).expect("Should be able to parse");
    cards.sort_by(|c1, c2| compare_hands(c1, c2, false));
    cards
        .iter()
        .enumerate()
        .map(|(idx, h)| (idx as i64 + 1) * h.bid)
        .sum()
}

pub fn part2(input: &str) -> i64 {
    let mut cards =
        fold_on_each_line1(input, |l| folder(l, true)).expect("Should be able to parse");
    cards.sort_by(|c1, c2| compare_hands(c1, c2, true));
    cards
        .iter()
        .enumerate()
        .map(|(idx, h)| (idx as i64 + 1) * h.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_day7() {
        let part1_sample = part1("input/day7_sample.txt");
        assert_eq!(6440, part1_sample);
        let part1_input = part1("input/day7_input.txt");
        assert_eq!(250120186, part1_input);
        println!("day7 part1 sample:{}, input: {}", part1_sample, part1_input);

        let part2_sample = part2("input/day7_sample.txt");
        assert_eq!(5905, part2_sample);
        let part2_input = part2("input/day7_input.txt");
        assert_eq!(250665248, part2_input);
        println!("day7 part2 sample {}, input: {}", part2_sample, part2_input);
    }
}

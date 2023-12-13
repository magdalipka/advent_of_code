use itertools::Itertools;
use std::{cmp::Ordering, iter::zip};

fn card_type(v: char) -> u32 {
    return match v {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'T' => 10,
        'J' => 0,
        x => x.to_digit(10).unwrap(),
    };
}

fn hand_type(v: &String) -> u32 {
    let chars = v.chars().counts();

    if chars.len() == 1 {
        // five of a kind
        return 7;
    };
    if chars.len() == 2 && chars.values().contains(&4) {
        // four of a kind
        return 6;
    }
    if chars.len() == 2 && chars.values().contains(&3) {
        // full house
        return 5;
    }
    if chars.len() == 3 && chars.values().contains(&3) {
        // three of a kind
        return 4;
    }
    if chars.len() == 3 {
        // three of a kind
        return 3;
    }
    if chars.len() == 4 {
        // three of a kind
        return 2;
    }
    return 1;
}

fn hand_type_with_joker(v: &String) -> u32 {
    todo!();
}

fn compare(a: &String, b: &String) -> Ordering {
    if hand_type_with_joker(a) > hand_type_with_joker(b) {
        return Ordering::Greater;
    };
    if hand_type_with_joker(b) > hand_type_with_joker(a) {
        return Ordering::Less;
    };
    for (va, vb) in zip(a.chars(), b.chars()) {
        if card_type(va) > card_type(vb) {
            return Ordering::Greater;
        };
        if card_type(vb) > card_type(va) {
            return Ordering::Less;
        };
    }
    return Ordering::Equal;
}

pub fn process(input: &str) -> u32 {
    let mut bids = input
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|bid| (bid.0.to_owned(), bid.1.parse::<u32>().unwrap()))
        .collect::<Vec<_>>();
    bids.sort_by(|a, b| compare(&a.0, &b.0));

    return bids.iter().enumerate().fold(0, |acc, (index, value)| {
        acc + ((index as u32 + 1) * value.1)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process(input), 5905);
    }
}

use camel_cards::{Hand, Rank};
use std::fs::read_to_string;

#[test]
fn test_sample_hand_1() {
    let hand = Hand::from("32T3K 765");
    let expected = Hand {
        hand: String::from("32T3K"),
        bid: 765,
        rank: Rank::OnePair,
    };
    assert_eq!(hand, expected);
}

#[test]
fn test_sample_races() {
    let input = read_to_string("./sample.txt").expect("Invalid sample file");
    let mut hands: Vec<Hand> = input.lines().map(|line| Hand::from(line)).collect();
    hands.sort();

    let mut sum: u32 = 0;
    for i in 0..hands.len() {
        sum += ((i as u32) + 1) * (hands.get(i).unwrap()).bid;
    }

    assert_eq!(sum, 6440);
}

#[test]
fn test_sort_cards() {
    let hand = Hand::from("KKQK6 0");
    let expected = Hand {
        hand: String::from("KKKQ6"),
        bid: 0,
        rank: Rank::ThreeOfAKind,
    };

    assert_eq!(hand, expected);
}

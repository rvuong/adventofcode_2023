use scratchpads::Card;
use std::fs::read_to_string;

#[test]
fn test_example_card_1() {
    let card: Card =
        Card::from("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53").expect("REASON");
    let mut expected_winning_numbers = vec![41, 48, 83, 86, 17];
    expected_winning_numbers.sort();
    let mut expected_numbers_you_have = vec![83, 86, 6, 31, 17, 9, 48, 53];
    expected_numbers_you_have.sort();
    let mut expected_winning_numbers_you_have = vec![48, 83, 17, 86];
    expected_winning_numbers_you_have.sort();

    assert_eq!(card.get_number(), 1);
    assert_eq!(card.get_winning_numbers(), &expected_winning_numbers);
    assert_eq!(card.get_numbers_you_have(), &expected_numbers_you_have);
    assert_eq!(
        card.get_winning_numbers_you_have(),
        &expected_winning_numbers_you_have
    );
    assert_eq!(card.get_points(), 8);
}

#[test]
fn test_example_card_2() {
    let card: Card =
        Card::from("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19").expect("REASON");

    assert_eq!(card.get_points(), 2);
}

#[test]
fn test_example_card_3() {
    let card: Card =
        Card::from("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1").expect("REASON");

    assert_eq!(card.get_points(), 2);
}

#[test]
fn test_example_card_4() {
    let card: Card =
        Card::from("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83").expect("REASON");

    assert_eq!(card.get_points(), 1);
}

#[test]
fn test_example_card_5() {
    let card: Card =
        Card::from("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36").expect("REASON");

    assert_eq!(card.get_points(), 0);
}

#[test]
fn test_example_card_6() {
    let card: Card =
        Card::from("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11").expect("REASON");

    assert_eq!(card.get_points(), 0);
}

#[test]
fn test_input_card_1() {
    let card: Card =
        Card::from("Card   1: 58 96 35 20 93 34 10 27 37 30 | 99 70 93 11 63 41 37 29  7 28 34 10 40 96 38 35 27 30 20 21  4 51 58 39 56").expect("REASON");

    assert_eq!(card.get_points(), 512);
}


#[test]
fn test_example_card_all() {
    let input = read_to_string("./sample.txt").expect("Invalid input file");
    let lines: Vec<&str> = input.lines().collect();

    let sum: u32 = lines
        .iter()
        .map(|line| {
            let card: Card = Card::from(line).expect("REASON");

            card.get_points()
        })
        .sum();

    assert_eq!(sum, 13);
}

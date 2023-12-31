use camel_cards::Hand;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("Invalid sample file");
    let mut hands: Vec<Hand> = input.lines().map(Hand::from).collect();
    hands.sort();
    // println!("{:#?}", hands);

    let mut sum: u32 = 0;
    for i in 0..hands.len() {
        sum += ((i as u32) + 1) * (hands.get(i).unwrap()).bid;
    }

    println!("[ Sum ]: {}", sum);
}

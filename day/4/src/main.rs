use scratchpads::{get_scratchpads, Card};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("Invalid input file");
    let lines: Vec<&str> = input.lines().collect();

    let sum: u32 = lines
        .iter()
        .map(|line| {
            let card: Card = Card::from(line).expect(line);

            card.get_points()
        })
        .sum();
    println!("Step 1: sum of points: {:?}", sum);

    // let scratchpads: Vec<u32> = get_scratchpads(&lines);
    // println!(
    //     "Step 2: number of total scratchpads: {:?}",
    //     scratchpads.len()
    // );
}

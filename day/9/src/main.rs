use mirage_maintenance::OASIS;
use std::fs::read_to_string;

fn main() {
    let oasis = OASIS::new();
    let input = read_to_string("./input.txt").expect("Invalid sample file");
    let lines: Vec<&str> = input.lines().collect();
    let prediction: i32 = lines.iter().map(|x| oasis.get_prediction_from(x)).sum();

    println!("The sum of predictions should be {}.", prediction);
}

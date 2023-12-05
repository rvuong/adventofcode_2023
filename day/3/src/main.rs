use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./full-engine-schematic.txt").expect("Invalid input file");
    let lines: Vec<&str> = input.lines().collect();
    let part_numbers: Vec<u32> = gondola_lift::get_part_numbers(&lines);

    println!(
        "part_numbers.iter().sum::<u32>(): {:?}",
        part_numbers.iter().sum::<u32>()
    );
}

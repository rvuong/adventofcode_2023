use haunted_wasteland::Map;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input.txt").expect("Invalid sample file");
    let map = Map::from(input.as_str());

    // println!("It takes {} steps to get out.", map.get_steps());

    println!(
        "If you were a ghost, it would take you {} steps to get out.",
        map.get_ghost_steps()
    );
}

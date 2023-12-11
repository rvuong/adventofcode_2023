use std::fs::read_to_string;
use wait_for_it::Race;

fn main() {
    let input = read_to_string("./input.txt").expect("Invalid sample file");
    let lines: Vec<&str> = input.lines().collect();

    let durations: Vec<&str> = lines.first().unwrap().split_whitespace().collect();
    let record_distances: Vec<&str> = lines.get(1).unwrap().split_whitespace().collect();

    let mut races: Vec<Race> = vec![];
    for i in 1..durations.len() {
        let d: &str = *durations.get(i).unwrap();
        let rd: &str = *record_distances.get(i).unwrap();
        races.push(Race {
            duration: d.parse::<u32>().unwrap(),
            record_distance: rd.parse::<u32>().unwrap(),
        });
    }

    let number_of_ways_to_win_multiplied: u32 = races
        .iter()
        .map(|race| race.get_number_of_ways_to_win())
        .product();
}

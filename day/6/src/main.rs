use std::fs::read_to_string;
use wait_for_it::Race;

fn main() {
    let input = read_to_string("./input.txt").expect("Invalid sample file");
    let lines: Vec<&str> = input.lines().collect();

    let durations: Vec<&str> = lines.first().unwrap().split_whitespace().collect();
    let record_distances: Vec<&str> = lines.get(1).unwrap().split_whitespace().collect();

    let mut races: Vec<Race> = vec![];
    for i in 1..durations.len() {
        let d: &str = durations.get(i).unwrap();
        let rd: &str = record_distances.get(i).unwrap();
        races.push(Race {
            duration: d.parse::<u64>().unwrap(),
            record_distance: rd.parse::<u64>().unwrap(),
        });
    }

    let number_of_ways_to_win_multiplied: u64 = races
        .iter()
        .map(|race| race.get_number_of_ways_to_win())
        .product();
    println!(
        "number_of_ways_to_win_multiplied: {:?}",
        number_of_ways_to_win_multiplied
    );

    // Part 2
    let concatenated_durations: Vec<&str> = durations
        .into_iter()
        .filter(|s| s.parse::<u64>().is_ok())
        .collect();
    let concatenated_duration = concatenated_durations.join("").parse::<u64>().unwrap();
    let concatenated_record_distances: Vec<&str> = record_distances
        .into_iter()
        .filter(|s| s.parse::<u64>().is_ok())
        .collect();
    let concatenated_record_distance = concatenated_record_distances
        .join("")
        .parse::<u64>()
        .unwrap();
    let full_race = Race {
        duration: concatenated_duration,
        record_distance: concatenated_record_distance,
    };
    println!(
        "full_race.get_number_of_ways_to_win(): {:?}",
        full_race.get_number_of_ways_to_win()
    );
}

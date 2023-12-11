use std::fs::read_to_string;
use wait_for_it::Race;

#[test]
fn test_sample_races() {
    let input = read_to_string("./sample.txt").expect("Invalid sample file");
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

    let race_1 = races.first().unwrap();
    assert_eq!(race_1.get_duration(), 7);
    assert_eq!(race_1.get_record_distance(), 9);

    let race_2 = races.get(1).unwrap();
    assert_eq!(race_2.get_duration(), 15);
    assert_eq!(race_2.get_record_distance(), 40);

    let race_3 = races.get(2).unwrap();
    assert_eq!(race_3.get_duration(), 30);
    assert_eq!(race_3.get_record_distance(), 200);
}

#[test]
fn test_sample_distance_by_charge_time() {
    let input = read_to_string("./sample.txt").expect("Invalid sample file");
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

    let race_1 = races.first().unwrap();

    assert_eq!(race_1.get_distance(0), 0);
    assert_eq!(race_1.get_distance(1), 6);
    assert_eq!(race_1.get_distance(2), 10);
    assert_eq!(race_1.get_distance(3), 12);
    assert_eq!(race_1.get_distance(4), 12);
    assert_eq!(race_1.get_distance(5), 10);
    assert_eq!(race_1.get_distance(6), 6);
    assert_eq!(race_1.get_distance(7), 0);
}

#[test]
fn test_sample_number_of_ways_to_win() {
    let input = read_to_string("./sample.txt").expect("Invalid sample file");
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

    let race_1 = races.first().unwrap();
    assert_eq!(race_1.get_number_of_ways_to_win(), 4);

    let race_2 = races.get(1).unwrap();
    assert_eq!(race_2.get_number_of_ways_to_win(), 8);

    let race_3 = races.get(2).unwrap();
    assert_eq!(race_3.get_number_of_ways_to_win(), 9);
}

#[test]
fn test_sample_margin_error() {
    let input = read_to_string("./sample.txt").expect("Invalid sample file");
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
    assert_eq!(number_of_ways_to_win_multiplied, 288);
}

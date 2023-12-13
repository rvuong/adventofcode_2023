use haunted_wasteland::Map;
use std::fs::read_to_string;

#[test]
fn test_sample_map() {
    let input = vec![
        "LLR",
        "",
        "AAA = (BBB, BBB)",
        "BBB = (AAA, ZZZ)",
        "ZZZ = (ZZZ, ZZZ)",
    ];
    let input = input.join("\n");
    let map = Map::from(input.as_str());

    assert_eq!(map.get_steps(), 6);
}

#[test]
fn test_input_map() {
    let input = read_to_string("./input.txt").expect("Invalid sample file");
    let map = Map::from(input.as_str());

    assert_eq!(map.get_steps(), 20777);
}

#[test]
fn test_ghost_sample_map() {
    let input = vec![
        "LR",
        "",
        "11A = (11B, XXX)",
        "11B = (XXX, 11Z)",
        "11Z = (11B, XXX)",
        "22A = (22B, XXX)",
        "22B = (22C, 22C)",
        "22C = (22Z, 22Z)",
        "22Z = (22B, 22B)",
        "XXX = (XXX, XXX)",
    ];

    let input = input.join("\n");
    let map = Map::from(input.as_str());

    assert_eq!(map.get_ghost_steps(), 6);
}

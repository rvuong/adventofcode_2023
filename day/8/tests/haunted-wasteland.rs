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

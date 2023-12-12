use haunted_wasteland::Map;

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

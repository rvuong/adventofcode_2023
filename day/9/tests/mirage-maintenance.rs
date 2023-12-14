use mirage_maintenance::OASIS;

#[test]
fn test_sample_row_1() {
    let oasis = OASIS::new();
    let prediction = oasis.get_prediction_from("0 3 6 9 12 15");

    assert_eq!(prediction, 18);
}

#[test]
fn test_sample_row_2() {
    let oasis = OASIS::new();
    let prediction = oasis.get_prediction_from("1 3 6 10 15 21");

    assert_eq!(prediction, 28);
}

#[test]
fn test_sample_row_3() {
    let oasis = OASIS::new();
    let prediction = oasis.get_prediction_from("10 13 16 21 30 45");

    assert_eq!(prediction, 68);
}

#[test]
fn test_sample_all_rows() {
    let oasis = OASIS::new();
    let input = vec!["0 3 6 9 12 15", "1 3 6 10 15 21", "10 13 16 21 30 45"];
    let prediction: i32 = input.iter().map(|x| oasis.get_prediction_from(x)).sum();
    assert_eq!(prediction, 114);
}

#[test]
fn test_sample_negative_row() {
    let oasis = OASIS::new();
    let prediction = oasis.get_prediction_from("2 -1 -1 5 35 140 429 1101 2482 5067 9586 17171 29829 51664 91688 169675 327410 648943 1295161 2560233 4960367");
    assert_eq!(prediction, 9368956);
}

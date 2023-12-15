#[test]
fn test_calibration_value_0() {
    let input_word = "1abc2";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 12);
}

#[test]
fn test_calibration_value_1() {
    let input_word = "pqr3stu8vwx";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 38);
}

#[test]
fn test_calibration_value_2() {
    let input_word = "a1b2c3d4e5f";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 15);
}

#[test]
fn test_calibration_value_3() {
    let input_word = "treb7uchet";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 77);
}

#[test]
fn test_calibration_value_4() {
    let input_word = "two1nine";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 29);
}

#[test]
fn test_calibration_value_5() {
    let input_word = "eightwothree";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 83);
}

#[test]
fn test_calibration_value_6() {
    let input_word = "abcone2threexyz";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 13);
}

#[test]
fn test_calibration_value_7() {
    let input_word = "xtwone3four";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 24);
}

#[test]
fn test_calibration_value_8() {
    let input_word = "4nineeightseven2";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 42);
}

#[test]
fn test_calibration_value_9() {
    let input_word = "zoneight234";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 14);
}

#[test]
fn test_calibration_value_10() {
    let input_word = "7pqrstsixteen";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 76);
}

#[test]
#[ignore]
fn test_input_row_value_0() {
    let input_word = "fivepqxlpninevh2xxsnsgg63pbvdnqptmg";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 53);
}

#[test]
#[ignore]
fn test_input_row_value_1() {
    let input_word = "eight8zlctbmsixhrvbpjb84nnmlcqkzrsix";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 86);
}

#[test]
#[ignore]
fn test_input_row_value_2() {
    let input_word = "hkxqfrqmsixfplbkpkdfzzszjxmdjtdkjlprrvr3gghlrqckqtbng";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 63);
}

#[test]
fn test_input_row_value_3() {
    let input_word = "4dtlmkfnm";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 44);
}

#[test]
fn test_input_row_value_4() {
    let input_word = "fiveeightwo";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 52);
}

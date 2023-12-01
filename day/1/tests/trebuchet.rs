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
#[ignore]
fn test_input_row_value_0() {
    let input_word = "fivepqxlpninevh2xxsnsgg63pbvdnqptmg";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 23);
}

#[test]
#[ignore]
fn test_input_row_value_1() {
    let input_word = "eight8zlctbmsixhrvbpjb84nnmlcqkzrsix";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 84);
}

#[test]
#[ignore]
fn test_input_row_value_2() {
    let input_word = "hkxqfrqmsixfplbkpkdfzzszjxmdjtdkjlprrvr3gghlrqckqtbng";
    let result = trebuchet::get_two_digit_number(input_word);

    assert_eq!(result, 33);
}

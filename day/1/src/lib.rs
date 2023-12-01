use std::fs::read_to_string;

// order_asc: true=asc, false=desc
fn get_digit(input: &&str, order_asc: bool) -> Option<u32> {
    for i in 0..input.len() {
        let index = match order_asc {
            true => i,
            false => input.len() - i - 1,
        };

        let char = input.chars().nth(index).unwrap();

        if char.is_numeric() {
            return char.to_digit(10); // to_digit() returns an Option
        }
    }

    None
}

pub fn get_two_digit_number(input: &str) -> u32 {
    let first_digit = get_digit(&input, true);
    let last_digit = get_digit(&input, false);

    if first_digit.is_none() || last_digit.is_none() {
        return 0;
    }

    first_digit.unwrap() * 10 + last_digit.unwrap()
}

pub fn get_input_sum(file: &str) -> u32 {
    let content = read_to_string(file).expect("Invalid input file");
    let lines: Vec<&str> = content.lines().collect();
    let line_sums: Vec<u32> = lines.into_iter().map(get_two_digit_number).collect();

    line_sums.into_iter().sum::<u32>()
}

use std::fs::read_to_string;

enum Order {
    Start,
    End,
}

// Convert string numbers into int values
// eg "[two]1[nine]" should be converted into "[2]1[9]"
fn numerize_string_numbers(input: String) -> String {
    let result = numerize_starting_string_numbers(input);

    numerize_ending_string_numbers(result)
}

fn numerize_starting_string_numbers(input: String) -> String {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for i in 0..(input.len()) {
        for number in numbers {
            let number_len = number.len();
            let substring = input.get(i..number_len + i);
            if substring.is_some() && substring.unwrap().eq(number) {
                let digit = match number {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => "",
                };

                let result = format!(
                    "{}{}{}",
                    input.get(..i).unwrap(),
                    digit,
                    input.get(number_len + i..).unwrap()
                );

                return result;
            }
        }
    }

    input
}

fn numerize_ending_string_numbers(input: String) -> String {
    let numbers = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for i in (0..input.len()).rev() {
        for number in numbers {
            let number_len = number.len();
            let end = i + 1;
            let start = if i >= number_len - 1 {
                i + 1 - number_len
            } else {
                0
            };

            let substring = input.get(start..end);
            if substring.is_some() && substring.unwrap().eq(number) {
                let digit = match number {
                    "one" => "1",
                    "two" => "2",
                    "three" => "3",
                    "four" => "4",
                    "five" => "5",
                    "six" => "6",
                    "seven" => "7",
                    "eight" => "8",
                    "nine" => "9",
                    _ => "",
                };

                let result = format!(
                    "{}{}{}",
                    input.get(..start).unwrap(),
                    digit,
                    input.get(end..).unwrap()
                );

                return result;
            }
        }
    }

    input
}

pub fn get_two_digit_number(input: &str) -> u32 {
    let tmp = numerize_string_numbers(String::from(input));

    let chars_vec: Vec<char> = tmp.chars().collect();
    let first_digit_index = digit_index(&chars_vec, Order::Start);
    let first_char = chars_vec.get(first_digit_index);
    let last_digit_index = digit_index(&chars_vec, Order::End);
    let last_char = chars_vec.get(last_digit_index);

    if first_char.is_none() || last_char.is_none() {
        return 0;
    }

    let first_digit = char2int(first_char.unwrap());
    let last_digit = char2int(last_char.unwrap());

    first_digit * 10 + last_digit
}

fn char2int(input: &char) -> u32 {
    let str = format!("{}", input);

    str.parse::<u32>().unwrap()
}

fn digit_index(input: &Vec<char>, order: Order) -> usize {
    match order {
        Order::Start => {
            for i in 0..input.len() {
                let c = input.get(i).unwrap();
                if c.is_ascii_digit() {
                    return i;
                }
            }
        }
        Order::End => {
            for i in (0..input.len()).rev() {
                let c = input.get(i).unwrap();
                if c.is_ascii_digit() {
                    return i;
                }
            }
        }
    }

    0_usize
}

pub fn get_input_sum(file: &str) -> u32 {
    let content = read_to_string(file).expect("Invalid input file");
    let lines: Vec<&str> = content.lines().collect();
    let line_sums: Vec<u32> = lines.into_iter().map(get_two_digit_number).collect();

    line_sums.into_iter().sum::<u32>()
}

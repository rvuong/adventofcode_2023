use std::fmt;
use std::fs::read_to_string;

enum Number {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Number::One => write!(f, "one"),
            Number::Two => write!(f, "two"),
            Number::Three => write!(f, "three"),
            Number::Four => write!(f, "four"),
            Number::Five => write!(f, "five"),
            Number::Six => write!(f, "six"),
            Number::Seven => write!(f, "seven"),
            Number::Eight => write!(f, "eight"),
            Number::Nine => write!(f, "nine"),
        }
    }
}

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

// Convert string numbers into int values
// eg "[two]1[nine]" should be converted into "[2]1[9]"
fn numerize_string_numbers(input: String) -> String {
    // Can't use ".replace()" because the replace is not starting by left of the String,
    // but by the pattern searched.
    // input
    //     .replace("one", "1")
    //     .replace("two", "2")
    //     .replace("three", "3")
    //     .replace("four", "4")
    //     .replace("five", "5")
    //     .replace("six", "6")
    //     .replace("seven", "7")
    //     .replace("eight", "8")
    //     .replace("nine", "9")
    // result:      "eightwothree"  -> "eigh2three" (2) -> "eigh23"
    // expected:    "eightwothree"  -> "8wothree" (8)   -> "8wo3"
    // So, we must instead parse the String from left to right, and replace case by case.

    let mut output: String = input;
    let mut output_len = output.len();
    let numbers = vec![
        Number::One,
        Number::Two,
        Number::Three,
        Number::Four,
        Number::Five,
        Number::Six,
        Number::Seven,
        Number::Eight,
        Number::Nine,
    ];

    let mut i = 0;
    while i < output_len {
        // Match each substring starting at that index, with number as words
        for n in numbers.iter() {
            let number = n.to_string();
            let number_length = number.len();

            unsafe {
                let end = i + number_length;
                let slice = output.get_unchecked(i..end);

                if slice == number {
                    let before = output.get_unchecked(0..i);
                    let after = output.get_unchecked(end..);
                    let number_value = match n {
                        Number::One => 1,
                        Number::Two => 2,
                        Number::Three => 3,
                        Number::Four => 4,
                        Number::Five => 5,
                        Number::Six => 6,
                        Number::Seven => 7,
                        Number::Eight => 8,
                        Number::Nine => 9,
                    };

                    output = format!("{}{}{}", before, number_value, after);
                    output_len = output.len();
                }
            }
        }

        i += 1;
    }

    output
}

pub fn get_two_digit_number(input: &str) -> u32 {
    let input = numerize_string_numbers(input.to_string());
    let input = input.as_str();

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

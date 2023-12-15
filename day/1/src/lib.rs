use regex::{ Captures, Regex };
use std::fmt;
use std::fs::read_to_string;

enum Number {
    // One,
    // Two,
    // Three,
    // Four,
    // Five,
    // Six,
    // Seven,
    // Eight,
    // Nine,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match self {
        //     Number::One => write!(f, "one"),
        //     Number::Two => write!(f, "two"),
        //     Number::Three => write!(f, "three"),
        //     Number::Four => write!(f, "four"),
        //     Number::Five => write!(f, "five"),
        //     Number::Six => write!(f, "six"),
        //     Number::Seven => write!(f, "seven"),
        //     Number::Eight => write!(f, "eight"),
        //     Number::Nine => write!(f, "nine"),
        // }
        write!(f, "one")
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
    // println!("$> input: {:?}", input);

    let output: String = input;

    // Replace 1st occurrence of number patterns in the string:
    let re: Regex = Regex::new(r"(?<first>one|two|three|four|five|six|seven|eight|nine|ten)").unwrap();
    let replaced = re.replace(&output, |caps: &Captures| {
        let captured_number = &caps[1];

        match captured_number {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            _ => "9",
        }
    });
    let output = format!("{}", replaced);

    // Replace last occurrence of number patterns in the string:
    let reverse: String = output.chars().rev().collect();
    let rev_re: Regex = Regex::new(r"(?<last>eno|owt|eerht|ruof|evif|xis|neves|thgie|enin|net)").unwrap();
    let rev_replaced = rev_re.replace(&reverse, |caps: &Captures| {
        let captured_number = &caps[1];

        match captured_number {
            "eno" => "1",
            "owt" => "2",
            "eerht" => "3",
            "ruof" => "4",
            "evif" => "5",
            "xis" => "6",
            "neves" => "7",
            "thgie" => "8",
            _ => "9",
        }
    });
    let reordered: String = rev_replaced.chars().rev().collect();

    reordered
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

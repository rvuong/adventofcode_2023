use regex::Regex;

fn get_start_and_end_y(row_index: usize, input_len: usize) -> (usize, usize) {
    let start_y = if row_index > 0 {
        row_index - 1
    } else {
        row_index
    };

    let end_y = if row_index < input_len - 1 {
        row_index + 1
    } else {
        row_index
    };

    (start_y, end_y)
}

fn get_start_and_end_x(number_str: &str, row: &str) -> (usize, usize) {
    let number_index = row.find(number_str).unwrap();

    let start_x = if number_index > 0 {
        number_index - 1
    } else {
        number_index
    };

    let end_x = if number_index + number_str.len() < row.len() - 1 {
        number_index + number_str.len()
    } else {
        number_index + number_str.len() - 1
    };

    (start_x, end_x)
}

fn is_part_number(number_str: &str, input: &Vec<&str>, row_index: usize) -> bool {
    let row = input.get(row_index).unwrap();
    let (start_y, end_y) = get_start_and_end_y(row_index, input.len());
    let (start_x, end_x) = get_start_and_end_x(number_str, row);

    for y in start_y..=end_y {
        let r = input.get(y).expect("KO");
        for x in start_x..=end_x {
            let c = r.chars().nth(x).unwrap();

            match c {
                '.' | '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {}
                _ => {
                    // println!("{} is a part number ({})", number_str, c);
                    return true;
                }
            };
        }
    }

    false
}

pub fn get_part_numbers(input: &Vec<&str>) -> Vec<u32> {
    let mut part_numbers_str: Vec<&str> = vec![];

    let re: Regex = Regex::new(r"([0-9]+)").unwrap();

    for (index, row) in input.iter().enumerate() {
        // Find numbers
        let matches: Vec<_> = re.find_iter(row).map(|m| m.as_str()).collect();
        // Filter part numbers
        for m in matches {
            // Check if there is an adjacent symbol:
            if is_part_number(m, input, index) {
                part_numbers_str.push(m);
            }
        }
    }

    // Cast into Vec<u32>
    part_numbers_str
        .iter()
        .map(|&x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

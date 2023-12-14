pub struct OASIS {}

impl OASIS {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_prediction_from(&self, input: &str) -> i32 {
        let mut sequences: Vec<Vec<i32>> = vec![];

        // Get numbers from the input &str
        let numbers: Vec<i32> = input
            .split_whitespace()
            .map(|s: &str| s.parse::<i32>().unwrap())
            .collect();
        sequences.push(numbers);
        let mut depth = 1;

        // Get the following sequences, calculated by the difference
        let mut sum: i32 = 1; // Any value but 0
        let max_depth = 1000000000;
        while sum != 0 && depth.le(&max_depth) {
            let sequence: &Vec<i32> = &sequences[depth - 1];
            // println!("sequence: {:?}", sequence);
            let differences = self.get_differences(sequence);
            sum = differences.iter().sum::<i32>();
            sequences.push(differences);
            depth += 1;
        }

        // Get the sequences next value
        for i in (1..(sequences.len())).rev() {
            let current_sequence = &sequences[i];
            let current_sequence_last_value = current_sequence[current_sequence.len() - 1];
            let previous_sequence = &mut sequences[i - 1];
            let previous_sequence_last_value = previous_sequence[previous_sequence.len() - 1];
            previous_sequence.push(current_sequence_last_value + previous_sequence_last_value);
        }

        sequences[0][sequences[0].len() - 1]
    }

    fn get_differences(&self, input: &Vec<i32>) -> Vec<i32> {
        let mut differences: Vec<i32> = vec![];

        for i in 1..input.len() {
            let x = input.get(i).unwrap() - input.get(i - 1).unwrap();
            differences.push(x);
        }

        differences
    }
}

impl Default for OASIS {
    fn default() -> Self {
        Self::new()
    }
}

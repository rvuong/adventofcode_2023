use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    instructions: String,
    network: HashMap<String, (String, String)>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let data: Vec<&str> = input.lines().collect();

        let mut network = HashMap::new();
        let lines = &data[2..data.len()]; // Exclude rows #0 and #1
        for line in lines {
            let key: String = (line[..3]).to_string();
            let left: String = (line[7..10]).to_string();
            let right: String = (line[12..15]).to_string();
            network.insert(key, (left, right));
        }

        Map {
            instructions: String::from(*data.first().unwrap()),
            network,
        }
    }
}

impl Map {
    pub fn get_steps(&self) -> u32 {
        let mut steps: usize = 0;

        // Initialisation
        let mut key: &str = "AAA";
        let max = 1000000;

        while !key.eq("ZZZ") && steps < max {
            // Get instruction
            let i = steps % self.instructions.len();
            let instruction = &self.instructions[i..(i + 1)];

            // Get element
            let node = self.network.get(key);
            key = match node {
                Some((left, right)) => match instruction {
                    "L" => left,
                    "R" => right,
                    _ => "",
                },
                None => "",
            };

            steps += 1;
        }

        steps as u32
    }
}

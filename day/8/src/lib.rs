use std::collections::HashMap;

#[derive(Debug)]
pub struct Map {
    instructions: String,
    network: HashMap<String, (String, String)>,
    ghost_start_steps: Vec<String>,
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let data: Vec<&str> = input.lines().collect();

        let mut network = HashMap::new();
        let mut ghost_start_steps: Vec<String> = vec![];

        let lines = &data[2..data.len()]; // Exclude rows #0 and #1
        for line in lines {
            let key: String = (line[..3]).to_string();
            let left: String = (line[7..10]).to_string();
            let right: String = (line[12..15]).to_string();
            network.insert(key.clone(), (left, right));

            let key_suffix = (key[2..3]).to_string();
            if key_suffix.eq(&String::from("A")) {
                ghost_start_steps.push(key);
            }
        }

        Map {
            instructions: String::from(*data.first().unwrap()),
            network,
            ghost_start_steps,
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

    pub fn get_ghost_steps(&self) -> u32 {
        let mut steps: usize = 0;

        // Initialisation
        let mut keys: Vec<&str> = vec![];
        for step in &self.ghost_start_steps {
            keys.push(step);
        }
        // let mut keys: Vec<&str> = vec!["JQA"];
        // let mut keys: Vec<&str> = vec!["NHA"];
        // let mut keys: Vec<&str> = vec!["AAA"];
        // let mut keys: Vec<&str> = vec!["FSA"];
        // let mut keys: Vec<&str> = vec!["LLA"];
        // let mut keys: Vec<&str> = vec!["MNA"];
        // println!("-> Starting from {:?}...", keys);

        let mut all_steps_are_at_end = false;
        let max = 1000000000;

        while !all_steps_are_at_end && steps < max {
            // Get instruction
            let i = steps % self.instructions.len();
            let instruction = &self.instructions[i..(i + 1)];

            let mut tmp_keys: Vec<&str> = vec![];
            for key in keys {
                // Get element
                let node = self.network.get(key);
                let processed_key = match node {
                    Some((left, right)) => match instruction {
                        "L" => left,
                        "R" => right,
                        _ => "",
                    },
                    None => "",
                };
                tmp_keys.push(processed_key);
            }

            keys = tmp_keys;

            let end: Vec<bool> = keys.iter().map(|step| self.is_ghost_end(step)).collect();
            all_steps_are_at_end = end.iter().all(|e| *e); // .fold(true, |acc, e| acc && *e);

            steps += 1;
        }

        if !all_steps_are_at_end {
            return 0 as u32;
        }

        steps as u32
    }

    fn is_ghost_end(&self, key: &str) -> bool {
        let key_suffix = (key[2..3]).to_string();

        key_suffix.eq(&String::from("Z"))
    }
}

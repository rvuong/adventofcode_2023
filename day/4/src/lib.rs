use regex::Regex;
use std::collections::HashSet;

pub struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
    winning_numbers_you_have: Vec<u32>,
}

impl Card {
    pub fn from(input: &str) -> Option<Self> {
        // let re: Regex = Regex::new(r"^Card (?<number>[0-9]+)\: (?<winning_numbers>([0-9]+[ ]+)*)\|(?<numbers_you_have>([ ]+[0-9]+)*)").unwrap();
        let re: Regex = Regex::new(r"^Card (?<number>[0-9]+)\: (?<numbers>.*)$").unwrap();
        let matches = re.captures(input);
        match matches {
            Some(m) => {
                let number: &str = &m["number"];
                let number: u32 = number.parse().unwrap();

                let mut numbers: Vec<&str> = m["numbers"].split(" | ").collect();

                let mut winning_numbers: Vec<u32> = numbers.get(0).unwrap()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x: &str| x.parse::<u32>().unwrap())
                    .collect();
                winning_numbers.sort();

                // let mut numbers_you_have: Vec<u32> = numbers.get(1).unwrap()
                //     .split(' ')
                //     .map(|x: &str| x.parse::<u32>().unwrap())
                //     .collect();
                // numbers_you_have.sort();

                // println!("numbers_you_have: {:?}", numbers.get(1).unwrap());
                let mut numbers_you_have: Vec<u32> = numbers.get(1).unwrap()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x: &str| x.parse::<u32>().unwrap())
                    .collect();
                // println!("numbers_you_have: {:?}", numbers_you_have);

                // let mut numbers_you_have = vec![0];
                // // let mut numbers_you_have: Vec<u32> = m["numbers_you_have"].split(' ')
                // //     .map(|s| s.parse().unwrap())
                // //     .collect();
                numbers_you_have.sort();

                let winning_numbers_you_have = vectors_intersect(&winning_numbers, &numbers_you_have);

                Some(Self {
                    number,
                    winning_numbers: winning_numbers.to_vec(),
                    numbers_you_have,
                    winning_numbers_you_have,
                })
            }
            None => None,
        }
    }

    pub fn get_number(&self) -> u32 {
        self.number
    }

    pub fn get_winning_numbers(&self) -> &Vec<u32> {
        &self.winning_numbers
    }

    pub fn get_numbers_you_have(&self) -> &Vec<u32> {
        &self.numbers_you_have
    }

    pub fn get_winning_numbers_you_have(&self) -> &Vec<u32> {
        &self.winning_numbers_you_have
    }

    pub fn get_points(&self) -> u32 {
        let len = self.winning_numbers_you_have.len();

        match len {
            0 => 0,
            1 => 1,
            _ => 2_u32.pow((len - 1) as u32),
        }
    }
}

/**
 * Uses HashSets in order to compute the intersection of the 2 Vecs
 */
fn vectors_intersect(v1: &[u32], v2: &[u32]) -> Vec<u32> {
    let mut hs1: HashSet<u32> = HashSet::new();
    let mut hs2: HashSet<u32> = HashSet::new();

    for v1_item in v1.iter() {
        hs1.insert(*v1_item);
    }

    for v2_item in v2.iter() {
        hs2.insert(*v2_item);
    }

    let intersection: HashSet<&u32> = hs1.intersection(&hs2).collect();
    let mut v: Vec<u32> = Vec::from_iter(intersection)
        .into_iter()
        .copied()
        .collect();
    v.sort();

    v
}

use regex::Regex;
use std::cmp::Ordering;
use std::collections::HashSet;

#[derive(Debug, Clone, Eq)]
pub struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    numbers_you_have: Vec<u32>,
    winning_numbers_you_have: Vec<u32>,
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.number.cmp(&other.number)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.number == other.number
    }
}

impl Card {
    pub fn from(input: &str) -> Option<Self> {
        let re: Regex = Regex::new(r"^Card[ ]+(?<number>[0-9]+)\: (?<numbers>.*)$").unwrap();
        let matches = re.captures(input);
        match matches {
            Some(m) => {
                let number: &str = &m["number"];
                let number: u32 = number.parse().unwrap();

                let numbers: Vec<&str> = m["numbers"].split(" | ").collect();

                let mut winning_numbers: Vec<u32> = numbers
                    .first()
                    .unwrap()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x: &str| x.parse::<u32>().unwrap())
                    .collect();
                winning_numbers.sort();

                let mut numbers_you_have: Vec<u32> = numbers
                    .get(1)
                    .unwrap()
                    .split(' ')
                    .filter(|x| !x.is_empty())
                    .map(|x: &str| x.parse::<u32>().unwrap())
                    .collect();
                numbers_you_have.sort();

                let winning_numbers_you_have =
                    vectors_intersect(&winning_numbers, &numbers_you_have);

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
    let mut v: Vec<u32> = Vec::from_iter(intersection).into_iter().copied().collect();
    v.sort();

    v
}

pub fn get_scratchpads(lines: &[&str]) -> Vec<u32> {
    // Get original Cards
    let original_cards: Vec<Card> = lines.iter().map(|line| Card::from(line).unwrap()).collect();

    // Get processed Cards
    let mut cards: Vec<Card> = lines.iter().map(|line| Card::from(line).unwrap()).collect();

    // Add copies of Cards
    let mut i = 0;
    while i < cards.len() && i < 50000 {
        let card = cards.get(i).unwrap();
        let number_of_winning_numbers_you_have = card.get_winning_numbers_you_have().len() as u32;
        if number_of_winning_numbers_you_have > 0 {
            // Get numbers of cards to be copied, add then to the Vec
            for n in
                (card.get_number() + 1)..=(card.get_number() + number_of_winning_numbers_you_have)
            {
                let new_card = original_cards.get(n as usize - 1).unwrap();
                cards.push(Card::clone(new_card));
            }
            cards.sort();
        }

        i += 1;
    }

    cards.iter().map(|c| c.get_number()).collect()
}

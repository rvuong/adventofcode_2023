use std::cmp::{Ord, Ordering};
use std::convert::From;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Rank {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl From<Rank> for u32 {
    fn from(rank: Rank) -> Self {
        match rank {
            Rank::FiveOfAKind => 6,
            Rank::FourOfAKind => 5,
            Rank::FullHouse => 4,
            Rank::ThreeOfAKind => 3,
            Rank::TwoPair => 2,
            Rank::OnePair => 1,
            Rank::HighCard => 0,
        }
    }
}

impl Rank {
    fn get(cards: &[Card]) -> Self {
        if cards.first().unwrap() == cards.get(1).unwrap()
            && cards.get(1).unwrap() == cards.get(2).unwrap()
            && cards.get(2).unwrap() == cards.get(3).unwrap()
            && cards.get(3).unwrap() == cards.get(4).unwrap()
        {
            Rank::FiveOfAKind
        } else if (cards.first().unwrap() == cards.get(1).unwrap()
            && cards.get(1).unwrap() == cards.get(2).unwrap()
            && cards.get(2).unwrap() == cards.get(3).unwrap())
            || (cards.get(1).unwrap() == cards.get(2).unwrap()
                && cards.get(2).unwrap() == cards.get(3).unwrap()
                && cards.get(3).unwrap() == cards.get(4).unwrap())
        {
            Rank::FourOfAKind
        } else if (cards.first().unwrap() == cards.get(1).unwrap()
            && cards.get(1).unwrap() == cards.get(2).unwrap()
            && cards.get(3).unwrap() == cards.get(4).unwrap())
            || (cards.first().unwrap() == cards.get(1).unwrap()
                && cards.get(2).unwrap() == cards.get(3).unwrap()
                && cards.get(3).unwrap() == cards.get(4).unwrap())
        {
            Rank::FullHouse
        } else if (cards.first().unwrap() == cards.get(1).unwrap()
            && cards.get(1).unwrap() == cards.get(2).unwrap())
            || (cards.get(1).unwrap() == cards.get(2).unwrap()
                && cards.get(2).unwrap() == cards.get(3).unwrap())
            || (cards.get(2).unwrap() == cards.get(3).unwrap()
                && cards.get(3).unwrap() == cards.get(4).unwrap())
        {
            Rank::ThreeOfAKind
        } else if (cards.first().unwrap() == cards.get(1).unwrap()
            && cards.get(2).unwrap() == cards.get(3).unwrap())
            || (cards.get(1).unwrap() == cards.get(2).unwrap()
                && cards.get(3).unwrap() == cards.get(4).unwrap())
        {
            Rank::TwoPair
        } else if cards.first().unwrap() == cards.get(1).unwrap()
            || cards.get(1).unwrap() == cards.get(2).unwrap()
            || cards.get(2).unwrap() == cards.get(3).unwrap()
            || cards.get(3).unwrap() == cards.get(4).unwrap()
        {
            Rank::OnePair
        } else {
            Rank::HighCard
        }
    }
}

#[derive(Debug, Eq)]
pub struct Hand {
    pub hand: String,
    pub bid: u32,
    pub rank: Rank,
}

impl From<&str> for Hand {
    fn from(input: &str) -> Self {
        let segments = input.split_whitespace().collect::<Vec<&str>>();
        let hand = String::from(*segments.first().unwrap());

        let mut cards: Vec<Card> = hand.chars().map(Card::from).collect();
        cards.sort();
        cards.reverse();
        // println!("Cards: {:#?}", cards);
        let sorted_hand: Vec<&str> = cards.iter().map(|card| card.card.as_str()).collect();
        let sorted_hand: String = sorted_hand.join("");
        // println!("Hand: {:#?}", h);

        let rank = Rank::get(&cards);

        Self {
            hand: sorted_hand,
            bid: segments.get(1).unwrap().parse::<u32>().unwrap(),
            rank,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match (other.rank as u32).cmp(&(self.rank as u32)) {
            Ordering::Greater => Ordering::Greater,
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                // Compare High Card

                let mut self_cards: Vec<Card> = self.hand.chars().map(Card::from).collect();
                self_cards.sort();
                self_cards.reverse();
                // println!("-> self_cards: {:#?}\n", self_cards);

                let mut other_cards: Vec<Card> = other.hand.chars().map(Card::from).collect();
                other_cards.sort();
                other_cards.reverse();
                // println!("> Self hand: {:#?} ; Other hand: {:#?}", self_cards, other_cards);

                for i in 0..self_cards.len() {
                    let self_card = self_cards.get(i).unwrap();
                    let other_card = other_cards.get(i).unwrap();

                    match self_card.cmp(other_card) {
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Equal => {}
                    };
                }

                Ordering::Equal
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

#[derive(Eq, Debug)]
struct Card {
    card: String,
    weight: u32,
}

impl From<char> for Card {
    fn from(card: char) -> Self {
        let weight = match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => 0,
        };

        Card {
            card: String::from(card),
            weight,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

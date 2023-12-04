use regex::Regex;
use std::cmp;
use std::fs::read_to_string;

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<Set>,
}

#[derive(Debug, PartialEq)]
pub struct Set {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    pub fn from(line: &str) -> Option<Game> {
        let re: Regex = Regex::new(r"^Game (?<id>[0-9]+)\: (?<sets>.*)$").unwrap();
        let Some(matches) = re.captures(line) else { return None; };

        let game_id: u32 = (matches["id"]).parse().unwrap();

        let sets = (matches["sets"]).split("; ");
        let mut game_sets: Vec<Set> = vec![];
        for set in sets {
            if let Some(set) = Set::from(set) {
                game_sets.push(set);
            }
        }

        Some(Game {
            id: game_id,
            sets: game_sets,
        })
    }
}

impl Set {
    fn from(input_set: &str) -> Option<Set> {
        let re: Regex = Regex::new(r"(\,\ )?(?<number>[0-9]+) (?<color>green|blue|red)").unwrap();

        let mut result = Set {
            red: 0,
            green: 0,
            blue: 0,
        };

        let colors: Vec<&str> = input_set.split(", ").collect();
        for color in colors {
            let Some(matches) = re.captures(color) else { return None; };
            match &matches["color"] {
                "red" => {
                    result.red = (matches["number"]).parse::<u32>().unwrap();
                }
                "green" => {
                    result.green = (matches["number"]).parse::<u32>().unwrap();
                }
                "blue" => {
                    result.blue = (matches["number"]).parse::<u32>().unwrap();
                }
                _ => {}
            }
        }

        Some(result)
    }

    pub fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

pub fn get_game_from_line(line: &str) -> Option<Game> {
    Game::from(line)
}

pub fn is_possible(line: &str, boundaries: &Set) -> bool {
    let game_result = Game::from(line);
    if game_result.is_none() {
        return false;
    }

    let game: Game = game_result.unwrap();
    for set in game.sets {
        if set.red > boundaries.red || set.green > boundaries.green || set.blue > boundaries.blue {
            return false;
        }
    }

    true
}

pub fn get_possible_games_ids_sum(file: &str) -> u32 {
    let boundaries = Set {
        red: 12,
        green: 13,
        blue: 14,
    };

    let content = read_to_string(file).expect("Invalid input file");
    let games_lines: Vec<&str> = content.lines().collect();

    let possible_games: Vec<u32> = games_lines
        .iter()
        .map(|&gl| {
            if !is_possible(gl, &boundaries) {
                return 0;
            }

            let game = Game::from(gl);

            match game {
                Some(g) => g.id,
                None => 0,
            }
        })
        .collect();

    possible_games.iter().sum::<u32>()
}

pub fn get_fewest_cubes(line: &str) -> Option<Set> {
    let game_result = Game::from(line);
    game_result.as_ref()?;

    let mut fewests: Set = Set {
        red: 0,
        green: 0,
        blue: 0,
    };
    let game = game_result.unwrap();
    for i in game.sets {
        fewests.red = cmp::max(i.red, fewests.red);
        fewests.green = cmp::max(i.green, fewests.green);
        fewests.blue = cmp::max(i.blue, fewests.blue);
    }

    Some(fewests)
}

pub fn get_powers_sum(file: &str) -> u32 {
    let content = read_to_string(file).expect("Invalid input file");
    let games_lines: Vec<&str> = content.lines().collect();

    let game_powers: Vec<u32> = games_lines
        .iter()
        .map(|&gl| {
            // let game = Game::from(gl);
            let result_set = get_fewest_cubes(gl);
            match result_set {
                Some(s) => s.power(),
                None => 0,
            }
        })
        .collect();

    game_powers.iter().sum::<u32>()
}

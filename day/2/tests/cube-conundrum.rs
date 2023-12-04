use cube_conundrum::{Game, Set};

fn get_mock() -> Game {
    Game {
        id: 1,
        sets: vec![
            Set {
                red: 4,
                green: 0,
                blue: 3,
            },
            Set {
                red: 1,
                green: 2,
                blue: 6,
            },
            Set {
                red: 0,
                green: 2,
                blue: 0,
            },
        ],
    }
}

fn get_boundaries() -> Set {
    Set {
        red: 12,
        green: 13,
        blue: 14,
    }
}

#[test]
fn test_example_game_1() {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let result = cube_conundrum::get_game_from_line(line);
    let expected = get_mock();

    match result {
        Some(r) => assert_eq!(r, expected),
        None => (),
    }
}

#[test]
fn test_is_example_1_possible() {
    let boundaries = get_boundaries();
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let result = cube_conundrum::is_possible(line, &boundaries);

    assert!(result);
}

#[test]
fn test_is_example_2_possible() {
    let boundaries = get_boundaries();
    let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    let result = cube_conundrum::is_possible(line, &boundaries);

    assert!(result);
}

#[test]
fn test_is_example_3_possible() {
    let boundaries = get_boundaries();
    let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let result = cube_conundrum::is_possible(line, &boundaries);

    assert!(!result);
}

#[test]
fn test_is_example_4_possible() {
    let boundaries = get_boundaries();
    let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
    let result = cube_conundrum::is_possible(line, &boundaries);

    assert!(!result);
}

#[test]
fn test_is_example_5_possible() {
    let boundaries = get_boundaries();
    let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let result = cube_conundrum::is_possible(line, &boundaries);

    assert!(result);
}

#[test]
fn test_get_possible_examples_sum_of_ids() {
    let boundaries = get_boundaries();
    let games_lines = vec![
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
        "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
        "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
        "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    ];

    // Get ids of possible games, as a Vector
    let possible_games: Vec<u32> = games_lines
        .iter()
        .map(|&gl| {
            if !cube_conundrum::is_possible(gl, &boundaries) {
                return 0;
            }

            let game = Game::from(gl);

            match game {
                Some(g) => g.id,
                None => 0,
            }
        })
        .collect();

    assert_eq!(possible_games.iter().sum::<u32>(), 8);
}

fn test_fewest_number_of_cubes(line: &str, expected: Set, expected_power: u32) {
    let result_set = cube_conundrum::get_fewest_cubes(line);
    match result_set {
        Some(s) => {
            assert_eq!(s, expected);
            assert_eq!(s.power(), expected_power);
        }
        None => assert!(false),
    }
}

#[test]
fn test_fewest_number_of_cubes_1() {
    let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
    let expected = Set {
        red: 4,
        green: 2,
        blue: 6,
    };
    let expected_power: u32 = 48;

    test_fewest_number_of_cubes(line, expected, expected_power);
}

#[test]
fn test_fewest_number_of_cubes_2() {
    let line = "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
    let expected = Set {
        red: 1,
        green: 3,
        blue: 4,
    };
    let expected_power: u32 = 12;

    test_fewest_number_of_cubes(line, expected, expected_power);
}

#[test]
fn test_fewest_number_of_cubes_3() {
    let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
    let expected = Set {
        red: 20,
        green: 13,
        blue: 6,
    };
    let expected_power: u32 = 1560;

    test_fewest_number_of_cubes(line, expected, expected_power);
}

#[test]
fn test_fewest_number_of_cubes_4() {
    let line = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
    let expected = Set {
        red: 14,
        green: 3,
        blue: 15,
    };
    let expected_power: u32 = 630;

    test_fewest_number_of_cubes(line, expected, expected_power);
}

#[test]
fn test_fewest_number_of_cubes_5() {
    let line = "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    let expected = Set {
        red: 6,
        green: 3,
        blue: 2,
    };
    let expected_power: u32 = 36;

    test_fewest_number_of_cubes(line, expected, expected_power);
}

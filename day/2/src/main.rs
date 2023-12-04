use cube_conundrum::{get_possible_games_ids_sum, get_powers_sum};

fn main() {
    println!(
        "Sum of IDs for possible games: {}",
        get_possible_games_ids_sum("./input.txt")
    );

    println!(
        "Sum of powers for listed games: {}",
        get_powers_sum("./input.txt")
    );
}

use cube_conundrum::get_possible_games_ids_sum;

fn main() {
    println!(
        "Sum of IDs for possible games: {}",
        get_possible_games_ids_sum("./input.txt")
    );
}

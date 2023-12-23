#[tauri::command]
pub fn puzzle7(input: String) -> u64 {
    let mut result: u64 = 0;
    let game_info: Vec<&str> = input.split("\n").collect();
    let mut hands: Vec<(&str, u64)> = Vec::new();
    for game in game_info {
        let current_game: Vec<&str> = game.split_whitespace().collect();
        let mut hand: (&str, u64) = (current_game[0], current_game[1].parse::<u64>().unwrap());
    }
    return result;
}

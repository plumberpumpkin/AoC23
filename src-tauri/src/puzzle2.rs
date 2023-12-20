

#[tauri::command]
pub fn puzzle2(input: String, blue_cubes: u8, red_cubes: u8, green_cubes: u8) -> u16 {
    let mut result: u16 = 0;
    let raw_games: Vec<&str> = input.split("\n").collect();
    //println!("Input is: gc {} , bc {}, rc {}", green_cubes, blue_cubes, red_cubes);
    let _games: Vec<Game> = Vec::new();
    for raw_game in raw_games {
        let colon_index = raw_game.find(":").unwrap();
        let game_id = raw_game[5..colon_index].parse::<u16>().unwrap();
        //gather existing sets
        let raw_game_sets: Vec<&str> = raw_game[colon_index + 2..].split(";").collect();
        let mut sets: Vec<(u8, u8, u8)> = Vec::new();
        for raw_game_set in raw_game_sets {
            let raw_set: Vec<&str> = raw_game_set.split(",").collect();
            let mut set: (u8, u8, u8) = (0, 0, 0);
            for colour in raw_set {
                if colour.contains("red") {
                    let mut red_count = colour.replace("red", "");
                    red_count = red_count.replace(" ", "");
                    set.0 = red_count.parse::<u8>().unwrap();
                }
                if colour.contains("green") {
                    let mut green_count = colour.replace("green", "");
                    green_count = green_count.replace(" ", "");
                    set.1 = green_count.parse::<u8>().unwrap();
                }
                if colour.contains("blue") {
                    let mut blue_count = colour.replace("blue", "");
                    blue_count = blue_count.replace(" ", "");
                    set.2 = blue_count.parse::<u8>().unwrap();
                }
            }
            sets.push(set);
        }
        let game: Game = Game { game_id, sets };
        println!("Game {} has {} sets", game.game_id, game.sets.len());
        //check if game is valid based on input
        let mut valid = true;
        //check game sets
        for set in game.sets {
            if set.0 > red_cubes || set.1 > green_cubes || set.2 > blue_cubes {
                valid = false;
                break;
            }
        }
        if valid {
            result += game_id;
        }
    }
    return result;
}

#[tauri::command]
pub fn puzzle2advanced(input: String) {
    let _result: u16 = 0;
    let _raw_games: Vec<&str> = input.split("\n").collect();
}

struct Game {
    game_id: u16,
    sets: Vec<(u8, u8, u8)>,
}

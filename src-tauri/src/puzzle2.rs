#[tauri::command]
pub fn puzzle2(input: String, blue_cubes: u32, red_cubes: u32, green_cubes: u32) -> u16 {
    let mut result: u16 = 0;
    let raw_games: Vec<&str> = input.split("\n").collect();
    //println!("Input is: gc {} , bc {}, rc {}", green_cubes, blue_cubes, red_cubes);
    let games: Vec<Game> = get_games(raw_games);
    for game in games {
        let mut valid = true;
        //check game sets
        for set in game.sets {
            if set.0 > red_cubes || set.1 > green_cubes || set.2 > blue_cubes {
                valid = false;
                break;
            }
        }
        if valid {
            result += game.game_id;
        }
    }

    return result;
}

#[tauri::command]
pub fn puzzle2advanced(input: String) -> u32 {
    let mut result: u32 = 0;
    let raw_games: Vec<&str> = input.split("\n").collect();
    let games: Vec<Game> = get_games(raw_games);
    for game in games {
        let mut min_ball_sets: (u32, u32, u32) = (0, 0, 0);
        for set in game.sets {
            if set.0 > min_ball_sets.0 {
                min_ball_sets.0 = set.0;
            }
            if set.1 > min_ball_sets.1 {
                min_ball_sets.1 = set.1;
            }
            if set.2 > min_ball_sets.2 {
                min_ball_sets.2 = set.2;
            }
        }
        println!(
            "Game {} has min ball sets: {:?}",
            game.game_id, min_ball_sets
        );
        let set_power = min_ball_sets.0 * min_ball_sets.1 * min_ball_sets.2;
        result += set_power;
        println!("Current result {}", result);
    }
    return result;
}

struct Game {
    game_id: u16,
    sets: Vec<(u32, u32, u32)>,
}

fn get_games(input: Vec<&str>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for raw_game in input {
        let colon_index = raw_game.find(":").unwrap();
        let game_id = raw_game[5..colon_index].parse::<u16>().unwrap();
        //gather existing sets
        let raw_game_sets: Vec<&str> = raw_game[colon_index + 2..].split(";").collect();
        let mut sets: Vec<(u32, u32, u32)> = Vec::new();
        for raw_game_set in raw_game_sets {
            let raw_set: Vec<&str> = raw_game_set.split(",").collect();
            let mut set: (u32, u32, u32) = (0, 0, 0);
            for colour in raw_set {
                if colour.contains("red") {
                    let mut red_count = colour.replace("red", "");
                    red_count = red_count.replace(" ", "");
                    set.0 = red_count.parse::<u32>().unwrap();
                }
                if colour.contains("green") {
                    let mut green_count = colour.replace("green", "");
                    green_count = green_count.replace(" ", "");
                    set.1 = green_count.parse::<u32>().unwrap();
                }
                if colour.contains("blue") {
                    let mut blue_count = colour.replace("blue", "");
                    blue_count = blue_count.replace(" ", "");
                    set.2 = blue_count.parse::<u32>().unwrap();
                }
            }
            sets.push(set);
        }
        let game: Game = Game { game_id, sets };
        games.push(game);
    }
    return games;
}

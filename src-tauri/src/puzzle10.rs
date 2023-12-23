use std::collections::HashMap;
use uuid::Uuid;

pub fn puzzle10(input: String) -> u64 {
    let raw_input: Vec<String> = input.split("\n").collect();
    let mut tiles: HashMap<Uuid, Tile> = HashMap::new();
    for i in 0..raw_input.len()-1{
        let tiles_in_line:Vec<char> = raw_input[i].chars().collect();
        for j in 0..tiles_in_line.len()-1{
            let mut tile: Tile = Tile::new();
            let raw_tile = tiles_in_line[j].to_string();
            tile.tile = raw_tile;
            if raw_tile == "S"{
                println!("Starting point found!");

            }
        }
    }
    let mut distance: u64 = 0;



    return distance;
}
struct Tile {
    tile_id: uuid,
    has_pipe: bool,
    tile: String,
    x_y_position: (usize, usize),
    distance_from_start: u64,
    is_connected: bool
}

impl Tile {
    fn new() -> Tile {
        Tile {
            tile_id: Uuid::new_v4(),
            has_pipe: false,
            tile: "X".to_string(),
            x_y_position: (0,0),
            distance_from_start: 0,
            is_connected: false
        }
    }

}
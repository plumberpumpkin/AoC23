use std::collections::HashMap;
use std::env::current_exe;

#[tauri::command]
pub fn puzzle8(input: String) -> u64 {
    let mut number_of_steps: u64 = 0;
    let raw_input: Vec<&str> = input.split("\n").collect();
    let moving_directions: String = String::from(raw_input[0]);
    let mut network_nodes: HashMap<String, (String, String)> = HashMap::new();
    for i in 2..raw_input.len() {
        let entry: String = String::from(raw_input[i]);
        let current_origin: String = String::from(&entry[0..3]);
        let destination_right: String = String::from(&entry[7..10]);
        let destination_left: String = String::from(&entry[12..15]);

        network_nodes.insert(current_origin, (destination_right, destination_left));
    }
    let mut current_node: String = String::from("AAA");
    let mut current_step: usize = 0;
    while current_node != "ZZZ" {
        if current_step == moving_directions.len() {
            println!("Current step: {}, resetting", current_step);
            current_step = 0;
        }
        let current_instruction = moving_directions.chars().nth(current_step).unwrap();
        let current_node_data = network_nodes.get(&current_node).unwrap();
        if current_instruction == 'L' {
            current_node = String::from(&current_node_data.0)
        } else {
            current_node = String::from(&current_node_data.1)
        }
        number_of_steps += 1;
        current_step += 1;
    }
    return number_of_steps;
}

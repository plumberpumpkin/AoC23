#[tauri::command]
pub fn puzzle3(input: String) -> u32{
    let mut result: u32 = 0;
    let intermediate_input: Vec<&str> = input.split("\n").collect();
    let mut machine_schematic: Vec<(&str, usize)> = Vec::new();
    for line in intermediate_input{
        let entry = (line, machine_schematic.len());
        machine_schematic.push(entry);
    }
    println!("Machine schematic: {:?}", machine_schematic);


    return result;
}
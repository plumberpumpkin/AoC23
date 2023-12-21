#[tauri::command]
pub fn puzzle6(input: String) -> u32{
    let mut result: u32 = 1;
    let raw_input:Vec<&str> = input.split("\n").collect();
    let raw_times_string: String = raw_input[0].replace("Time:", "");
    let raw_times:Vec<&str> = raw_times_string.split_whitespace().collect();
    let raw_distances_string: String = raw_input[1].replace("Distance:", "");
    let raw_distances: Vec<&str> = raw_distances_string.split_whitespace().collect();
    let mut races: Vec<(u32,u32)> = Vec::new();
    let total_races = raw_times.len();
    for i in 0..total_races{
        let race = (raw_times[i].parse::<u32>().unwrap(), raw_distances[i].parse::<u32>().unwrap());
        races.push(race);
    }
    for race in races{
        let mut number_of_winnable_solutions = 0;
        for i in 0..race.0{
            let velocity = i*1;
            let distance = velocity * (race.0 - i);
            if distance > race.1{
                println!("Found a winnable solution: {}, {}", velocity, distance);
                number_of_winnable_solutions += 1;
            }
        }
        println!("Number of winnable solutions: {}", number_of_winnable_solutions);
        result *= number_of_winnable_solutions;
    }
    return result;
}

#[tauri::command]
pub fn puzzle6advanced(input: String) ->u32 {
    let result: u32 = 0;


    return result;
}
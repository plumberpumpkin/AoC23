#[tauri::command]
pub fn puzzle6(input: String) -> u128 {
    let mut result: u128 = 1;
    let raw_input: Vec<&str> = input.split("\n").collect();
    let raw_times_string: String = raw_input[0].replace("Time:", "");
    let raw_times: Vec<&str> = raw_times_string.split_whitespace().collect();
    let raw_distances_string: String = raw_input[1].replace("Distance:", "");
    let raw_distances: Vec<&str> = raw_distances_string.split_whitespace().collect();
    let mut races: Vec<(u128, u128)> = Vec::new();
    let total_races = raw_times.len();
    for i in 0..total_races {
        let race = (
            raw_times[i].parse::<u128>().unwrap(),
            raw_distances[i].parse::<u128>().unwrap(),
        );
        races.push(race);
    }
    for race in races {
        let mut number_of_winnable_solutions = 0;
        for i in 0..race.0 {
            let velocity = i * 1;
            let distance = velocity * (race.0 - i);
            if distance > race.1 {
                println!("Found a winnable solution: {}, {}", velocity, distance);
                number_of_winnable_solutions += 1;
            }
        }
        println!(
            "Number of winnable solutions: {}",
            number_of_winnable_solutions
        );
        result *= number_of_winnable_solutions;
    }
    return result;
}

#[tauri::command]
pub fn puzzle6advanced(input: String) -> u128 {
    let raw_input: Vec<&str> = input.split("\n").collect();
    let time: u128 = raw_input[0]
        .replace("Time:", "")
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();
    let distance: u128 = raw_input[1]
        .replace("Distance:", "")
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();
    let mut lower_bound: u128 = 0;
    //find lower bound
    for i in 0..time {
        let velocity: u128 = i * 1;
        let current_distance: u128 = velocity * (time - i);
        if current_distance > distance {
            println!(
                "Found a lower bound winnable solution: {}, {}",
                velocity, distance
            );
            lower_bound = velocity;
            break;
        }
    }
    //find upper bound
    let mut upper_bound: u128 = 0;
    for i in 0..time {
        let velocity: u128 = time - i;
        let current_distance: u128 = velocity * (time - velocity);
        if current_distance > distance {
            println!(
                "Found an upper bound winnable solution: {}, {}",
                velocity, distance
            );
            upper_bound = velocity;
            break;
        }
    }
    //Plus 1 to include the upper bound
    let result: u128 = upper_bound - lower_bound + 1;

    return result;
}

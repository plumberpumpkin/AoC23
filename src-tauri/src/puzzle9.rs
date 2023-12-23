#[tauri::command]
pub fn puzzle9(input: String) -> i128 {
    let mut result: i128 = 0;
    let mut raw_input: Vec<&str> = input.split("\n").collect();
    let mut sequences: Vec<Vec<i128>> = Vec::new();
    for entry in raw_input{
        let mut raw_sequence: Vec<&str> = entry.split_whitespace().collect();
        let mut sequence: Vec<i128> = Vec::new();
        for number in raw_sequence{
            sequence.push(number.parse::<i128>().unwrap());
        }
        sequences.push(sequence);
    }
    println!("Sequences {:?}", sequences);
    for sequence in sequences{
        let mut intermediate_sum: i128 = 0;
        let mut last_entries: Vec<i128> = Vec::new();
        let mut intermediate_sequences: Vec<Vec<i128>> = Vec::new();
        intermediate_sequences.push(sequence);
        let mut sequence_is_zeroes: bool = false;
        let mut current_sub_sequence_number: usize = 0;
        while !sequence_is_zeroes {
            let mut intermediate_sequence: Vec<i128> = Vec::new();
            let current_sub_sequence = &intermediate_sequences[current_sub_sequence_number];
            for i in 0..current_sub_sequence.len(){
                if i == current_sub_sequence.len()-1{
                    break;
                } else {
                    let difference = current_sub_sequence[&i+1] - current_sub_sequence[*&i];
                    intermediate_sequence.push(difference);
                }
            }
            current_sub_sequence_number += 1;
            sequence_is_zeroes = is_sequence_just_zeroes(&intermediate_sequence);
            println!("Sequence {:?}", &intermediate_sequence);
            intermediate_sequences.push(intermediate_sequence);
        }
        //Here my sequences are defined
        for i in 0..intermediate_sequences.len(){
            let current_sequence_selector = intermediate_sequences.len()-i-1;
            let current_sequence = &intermediate_sequences[current_sequence_selector];
            if is_sequence_just_zeroes(current_sequence) {
                last_entries.push(0);
            } else {
                let last_entry = last_entries[last_entries.len()-1] + current_sequence[current_sequence.len()-1];
                println!("Last extrapolated entry {:?}", &last_entry);
                last_entries.push(last_entry);
            }
        }
        result += last_entries[last_entries.len()-1];

    }
    return result;
}

fn is_sequence_just_zeroes(sequence: &Vec<i128>) -> bool{
    if sequence.len() == 0{
        return false;
    } else{
        for &entry in sequence{
            if entry != 0{
                return false;
            }
        }
    }
    return true;
}

fn sum_vector(vector: Vec<i128>) -> i128 {
    let mut sum: i128 = 0;
    for entry in vector{
        sum += entry;
    }
    return sum;
}
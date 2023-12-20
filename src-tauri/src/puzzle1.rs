#[tauri::command]
pub fn puzzle1(input: String) -> u16
{
    let mut result: u16 = 0;
    let entries: Vec<&str> = input.split_whitespace().collect();
    for entry in entries{
        //store all numbers in new object
        let mut numbers: Vec<char> = Vec::new();
        for char in entry.chars(){
            if char.is_digit(10){
                numbers.push(char);
            }
        }
        let mut digits: String = String::from("");
        digits.push(numbers[0]);
        digits.push(numbers[numbers.len()-1]);
        let number: u16 = digits.parse().unwrap();
        result += number;
    }
    return result;
}

#[tauri::command]
pub fn puzzle1advanced(input: String) -> u16
{
    let mut result: u16 = 0;
    let entries: Vec<&str> = input.split_whitespace().collect();
    for entry in entries{
        //store all numbers in new object
        let mut number_tuples: Vec<(String, usize)> = Vec::new();
        //first get numbers from entry
        for i in 0..entry.len(){
            let char = entry.chars().nth(i).unwrap();
            if char.is_digit(10){
                let number_tuple: (String, usize) = (char.to_string(), i);
                number_tuples.push(number_tuple);
            }
        }
        //now search for written numbers
        if entry.contains("one") {
            let number_tuple: (String, usize) = ("1".to_string(), entry.find("one").unwrap());
            number_tuples.push(number_tuple);
        }


    }
    return result;
}

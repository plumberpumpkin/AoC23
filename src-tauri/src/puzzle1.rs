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

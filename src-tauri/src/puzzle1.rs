#[tauri::command]
pub fn puzzle1(input: String) -> u16
{
    let mut result: u16 = 0;
    let entries: Vec<&str> = input.split(" ").collect();
    for entry in entries{

        //store all numbers in new object
        //build number from first and last digit
        //add number to result
    }
    return result;
}

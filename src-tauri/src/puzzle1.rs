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
            //Finds the first occurence of the pattern in the string
            let number_tuple1: (String, usize) = ("1".to_string(), entry.find("one").unwrap());
            //Finds the last occurence of the pattern in the string
            let number_tuple2: (String, usize) = ("1".to_string(), entry.rfind("one").unwrap());
            //Store them both to vector -> sorting by index value guarantees that double numbers are next to each other with the same index and the number builds correctly
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }
        if entry.contains("two") {
            let number_tuple1: (String, usize) = ("2".to_string(), entry.find("two").unwrap());
            let number_tuple2: (String, usize) = ("2".to_string(), entry.rfind("two").unwrap());
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }
        if entry.contains("three") {
            let number_tuple1: (String, usize) = ("3".to_string(), entry.rfind("three").unwrap());
            let number_tuple2: (String, usize) = ("3".to_string(), entry.find("three").unwrap());
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }
        if entry.contains("four") {
            let number_tuple1: (String, usize) = ("4".to_string(), entry.rfind("four").unwrap());
            let number_tuple2: (String, usize) = ("4".to_string(), entry.find("four").unwrap());
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }
        if entry.contains("five") {
            let number_tuple1: (String, usize) = ("5".to_string(), entry.rfind("five").unwrap());
            let number_tuple2: (String, usize) = ("5".to_string(), entry.find("five").unwrap());
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }
        if entry.contains("six") {
            let number_tuple1: (String, usize) = ("6".to_string(), entry.rfind("six").unwrap());
            let number_tuple2: (String, usize) = ("6".to_string(), entry.find("six").unwrap());
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }
        if entry.contains("seven") {
            let number_tuple1: (String, usize) = ("7".to_string(), entry.rfind("seven").unwrap());
            let number_tuple2: (String, usize) = ("7".to_string(), entry.find("seven").unwrap());
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }
        if entry.contains("eight") {
            let number_tuple1: (String, usize) = ("8".to_string(), entry.rfind("eight").unwrap());
            let number_tuple2: (String, usize) = ("8".to_string(), entry.find("eight").unwrap());
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }
        if entry.contains("nine") {
            let number_tuple1: (String, usize) = ("9".to_string(), entry.rfind("nine").unwrap());
            let number_tuple2: (String, usize) = ("9".to_string(), entry.find("nine").unwrap());
            number_tuples.push(number_tuple1);
            number_tuples.push(number_tuple2);
        }

        //now sort the tuples by their index
        number_tuples.sort_by(|a, b| a.1.cmp(&b.1));
        println!("{:?}", number_tuples);
       // number_tuples.reverse();
        //now build the number
        let mut number: String = String::from("");
        number.push(number_tuples[0].0.chars().nth(0).unwrap());
        number.push(number_tuples[number_tuples.len()-1].0.chars().nth(0).unwrap());
        let number: u16 = number.parse().unwrap();
        result += number;

    }
    return result;
}

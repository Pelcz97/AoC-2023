use std::collections::HashMap;

fn main() {
    let input = include_str!("./Input1.txt");
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let modified_line = replace_str_with_digit(String::from(line));
        let num1 = find_first_digit(String::from(&modified_line)).unwrap();
        let num2 = find_first_digit(String::from(&modified_line).chars().rev().collect::<String>()).unwrap();
        result += num1 * 10 + num2;
    }
    print!("{}", result);
}

fn find_first_digit(line : String) -> Option<u32> {
    return line.chars()
    .find(|c| c.is_digit(10))
    .map(|c| c.to_digit(10).unwrap());
}

fn replace_str_with_digit(line: String) -> String {
    let mut result = String::new();
    let strings = HashMap::from([("one", "1"), ("two", "2"), ("three","3"), ("four", "4"), ("five", "5"), ("six", "6"), ("seven", "7"), ("eight", "8"), ("nine", "9")]);
    for i in 0..line.len() {
       let substring = &line[i..];
       for (key, value) in strings.iter() {
            if substring.starts_with(key) {
                result.insert_str(result.len(), value);
                break;
            } 
            if substring.starts_with(value) {
                result.insert(result.len(), line.chars().nth(i).unwrap());
                break;
            }
       }
    }
    return result;
}
fn main() {
    let input = include_str!("./Input1.txt");
    let lines = input.lines();
    let mut result = 0;
    for line in lines {
        let num1 = find_first_digit(String::from(line)).unwrap();
        let num2 = find_first_digit(line.chars().rev().collect::<String>()).unwrap();
        result += num1 * 10 + num2;
    }
    print!("{}", result);
}

fn find_first_digit(line : String) -> Option<u32> {
    for elem in line.chars() {
        if elem.is_digit(10) {
            return elem.to_digit(10);
        }
    }
    return Option::None;
}
struct Coordinates {
    pub row: i32,
    pub start_index: i32,
    pub end_index: i32,
    pub value: i32,
}

fn main() {
    let input = include_str!("./Input3.txt");
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let mut coordinates: Vec<Coordinates> = vec![];
    let mut result: i32 = 0;

    for (line_index, line) in lines.clone().into_iter().enumerate() {
        let mut start_index = 0;
        let mut current_value = String::new();
        let mut is_same_number = false;
        for (index, character) in line.char_indices() {
            if character.is_ascii_digit() {
                if !is_same_number {
                    is_same_number = true;
                    start_index = index;
                }
                current_value.push(character);
            } else {
                if is_same_number {
                    coordinates.push(Coordinates {
                        row: line_index as i32,
                        start_index: start_index as i32,
                        end_index: (index - 1) as i32,
                        value: current_value.parse::<i32>().unwrap(),
                    });
                    is_same_number = false;
                    current_value = String::new();
                }
            }
        }
        if is_same_number {
            coordinates.push(Coordinates {
                row: line_index as i32,
                start_index: start_index as i32,
                end_index: (lines[0].len() - 1) as i32,
                value: current_value.parse::<i32>().unwrap(),
            });
        }
    }
    let mut results_values = String::new();
    for coordinate in coordinates {
        if check_coordinate(&coordinate, &lines.clone()) {
            results_values.push_str(&coordinate.value.to_string());
            results_values.push(',');
        result += coordinate.value;
        }
    }
    print!("{}", results_values);
    print!("{}", result);
}

fn check_coordinate(coordinate: &Coordinates, input: &Vec<String>) -> bool {
    let mut result = false;
    for i in coordinate.start_index ..=coordinate.end_index {
        result |=  check(&input, i - 1, coordinate.row)
                || check(&input, i + 1, coordinate.row)
                || check(&input, i, coordinate.row - 1)
                || check(&input, i, coordinate.row + 1)
                || check(&input, i - 1, coordinate.row - 1)
                || check(&input, i - 1, coordinate.row + 1)
                || check(&input, i + 1, coordinate.row - 1)
                || check(&input, i + 1, coordinate.row + 1);
        if result == true { return true;}
    }
    return result;
}

fn check(input: &Vec<String>, x: i32, y: i32) -> bool {
    if y < 0 {
        return false;
    }
    if x < 0 {
        return false;
    }
    if y >= input.len() as i32 {
        return false;
    }
    let y_as_usize = y as usize;
    let x_as_usize = x as usize;
    if x_as_usize >= input[y_as_usize].len()  {
        return false;
    }
    let char_at_position = input[y_as_usize]
    .chars()
    .nth(x as usize)
    .expect(format!("Could not find at position {} {}", x, y).as_str());
    return !(char_at_position.is_ascii_digit() || char_at_position == '.');
}

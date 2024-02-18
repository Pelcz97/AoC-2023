pub struct Game {
    pub id: i32,
    pub plays: Vec<Play>,
}

impl Game {
    pub fn from_line(line: String) -> Game {
        let game = split_and_collect(&line, ":");
        let id = split_and_collect(&game[0], " ")[1]
            .parse::<i32>()
            .expect("Could not parse number");
        let play_strings = split_and_collect(&game[1], ";");
        let mut plays: Vec<Play> = vec![];
        for play_string in play_strings {
            let draws = split_and_collect(&play_string, ",");
            let trimmed_draws = draws.iter().map(|s| s.trim()).collect::<Vec<&str>>();
            let mut play: Play = Play {
                red: 0,
                blue: 0,
                green: 0,
            };

            for draw in trimmed_draws {
                let color = split_and_collect(&draw.to_string(), " ");
                match color[1].as_str() {
                    "red" => play.red = parse_and_expect(&color[0]),
                    "blue" => play.blue = parse_and_expect(&color[0]),
                    "green" => play.green = parse_and_expect(&color[0]),
                    _ => panic!("Could not find color"),
                }
            }
            plays.push(play);
        }
        return Game {
            id: id,
            plays: plays,
        };
    }
}

fn split_and_collect(line: &String, split_char: &str) -> Vec<String> {
    return line
        .split(split_char)
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
}

fn parse_and_expect(line: &String) -> u32 {
    return line.parse::<u32>().expect("Could not parse number");
}

pub struct Play {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

use std::collections::HashMap;
use crate::structs::game::Game;

pub mod structs;

fn main() {
    let input = include_str!("./Input2.txt");
    let lines = input.lines();
    let mut result = 0;
    
    for line in lines {
        let game = Game::from_line(String::from(line));
        let max_red = game.plays.iter().map(|play|play.red).max().unwrap();
        let max_green = game.plays.iter().map(|play|play.green).max().unwrap();
        let max_blue = game.plays.iter().map(|play|play.blue).max().unwrap();
        result += max_red * max_blue * max_green;
    }
    print!("{}", result);
}
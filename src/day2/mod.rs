// https://adventofcode.com/2023/day/2

use std::path::Path;

use crate::common::read_lines;

const N_RED: u32 = 12;
const N_GREEN: u32 = 13;
const N_BLUE: u32 = 14;

#[derive(Debug)]
struct Game {
    id: u32,
    turns: Vec<Turn>,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            turns: Vec::new(),
        }
    }

    fn add_turn(&mut self, turn: Turn) {
        self.turns.push(turn);
    }

    fn is_possible(&self) -> bool {
        let (red, green, blue) = self.turns.iter().fold((0, 0, 0), |sum, turn| {
            (sum.0 + turn.red, sum.1 + turn.green, sum.2 + turn.blue)
        });
        red <= N_RED && green <= N_GREEN && blue <= N_BLUE
    }
}

#[derive(Debug)]
struct Turn {
    red: u32,
    green: u32,
    blue: u32,
}

impl Turn {
    fn parse(summary: &str) -> Self {
        let colors_str: Vec<&str> = summary.split(", ").collect();

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for color_str in colors_str.iter() {
            let c: Vec<&str> = color_str.split(" ").collect();
            let n: u32 = c[0].to_string().parse().unwrap();
            let color = c[1];
            match color {
                "red" => {
                    red += n;
                }
                "green" => {
                    green += n;
                }
                "blue" => {
                    blue += n;
                }
                _ => {
                    panic!("unknown color {}", color);
                }
            };
        }

        Self { red, green, blue }
    }
}

pub fn run() {
    let path = Path::new("src/day2/input1.txt");
    let mut sum = 0;
    match read_lines(path) {
        Ok(lines) => {
            for line in lines {
                match line {
                    Ok(line) => {
                        let s: Vec<&str> = line.split(": ").collect();

                        let game_and_id: Vec<&str> = s[0].split(" ").collect();
                        let game_id: u32 = game_and_id[1].to_string().parse().unwrap();

                        let mut game = Game::new(game_id);

                        let turn_summaries: Vec<&str> = s[1].split("; ").collect();
                        for turn_summary in turn_summaries.iter() {
                            let turn = Turn::parse(&turn_summary);
                            game.add_turn(turn);
                        }

                        if game.is_possible() {
                            sum += game.id;
                        }
                    }
                    Err(err) => {
                        panic!("Cannot read line in {} - {}", path.display(), err);
                    }
                }
            }
        }
        Err(err) => {
            panic!("Cannot read file {} - {}", path.display(), err);
        }
    }

    // Determine which games would have been possible if the bag had been loaded with only 12 red cubes, 13 green cubes, and 14 blue cubes. What is the sum of the IDs of those games?
    println!("{}", sum);
}

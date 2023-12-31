// https://adventofcode.com/2023/day/2

use std::{cmp::max, path::Path};

use crate::common::read_lines;

#[derive(Debug, Default)]
struct CubeCounts {
    red: Option<u32>,
    green: Option<u32>,
    blue: Option<u32>,
}

impl CubeCounts {
    fn default() -> Self {
        Self {
            red: None,
            green: None,
            blue: None,
        }
    }

    fn update_min(&mut self, other: CubeCounts) {
        self.red = max_or_take(self.red, other.red);
        self.green = max_or_take(self.green, other.green);
        self.blue = max_or_take(self.blue, other.blue);
    }

    fn power(&self) -> u64 {
        self.red.unwrap_or(0) as u64
            * self.green.unwrap_or(0) as u64
            * self.blue.unwrap_or(0) as u64
    }
}

#[derive(Debug)]
struct Game {
    cube_counts: CubeCounts,
}

impl Game {
    fn new() -> Self {
        Self {
            cube_counts: CubeCounts::default(),
        }
    }

    fn add_turn(&mut self, turn: Turn) {
        self.cube_counts.update_min(turn.cube_counts);
    }
}

#[derive(Debug)]
struct Turn {
    cube_counts: CubeCounts,
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

        Self {
            cube_counts: CubeCounts {
                red: none_if_zero(red),
                green: none_if_zero(green),
                blue: none_if_zero(blue),
            },
        }
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
                        let mut game = Game::new();

                        let turn_summaries: Vec<&str> = s[1].split("; ").collect();
                        for turn_summary in turn_summaries.iter() {
                            let turn = Turn::parse(&turn_summary);
                            game.add_turn(turn);
                        }

                        println!("{:?} (power={})", game, game.cube_counts.power());
                        sum += game.cube_counts.power();
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

fn none_if_zero(n: u32) -> Option<u32> {
    if n == 0 {
        None
    } else {
        Some(n)
    }
}

fn max_or_take(mine: Option<u32>, theirs: Option<u32>) -> Option<u32> {
    if let Some(theirs) = theirs {
        Some(if let Some(mine) = mine {
            max(mine, theirs)
        } else {
            theirs
        })
    } else {
        mine
    }
}

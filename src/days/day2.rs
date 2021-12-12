use core::fmt;
use std::{fs::File, io::BufRead};
use std::io::BufReader;

struct Command {
    dir: Direction,
    mag: i32,
}

impl Command {
    fn from_string(string: String) -> Command {
        let tokens: Vec<&str> = string.splitn(2, ' ').collect();
        let dir_string = tokens[0];
        let mag = tokens[1].parse::<i32>().unwrap();
        let dir = match dir_string {
            "forward" => Direction::Forward,
            "up" => Direction::Up,
            "down" => Direction::Down,
            _ => { panic!() },
        };
        Command { dir: dir, mag: mag }
    }
}

enum Direction {
    Forward,
    Up,
    Down,
}

struct Coord {
    x: i32,
    y: i32,
    aim: i32,
}

impl Coord {
    fn new() -> Coord {
        Coord { x: 0, y: 0, aim: 0 }
    }

    fn augment(&mut self, com: Command) {
        match com.dir {
            Direction::Forward => { self.x += com.mag },
            Direction::Down => { self.y += com.mag },
            Direction::Up => { self.y -= com.mag },
        }
    }

    fn aim(&mut self, com: Command) {
        match com.dir {
            Direction::Forward => {
                self.x += com.mag;
                self.y += com.mag * self.aim;
            },
            Direction::Down => { self.aim += com.mag },
            Direction::Up => { self.aim -= com.mag },
        }
    }
}

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let product = self.x * self.y;
        write!(f, "{} x {} = {}", self.x, self.y, product)
    }
}

pub fn part1(input_file: BufReader<File>) {
    let mut coord = Coord::new();
    for line in input_file.lines() {
        coord.augment(Command::from_string(line.unwrap()));
    }
    println!("{}", coord);
}

pub fn part2(input_file: BufReader<File>) {
    let mut coord = Coord::new();
    for line in input_file.lines() {
        coord.aim(Command::from_string(line.unwrap()));
    }
    println!("{}", coord);
}
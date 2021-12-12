use std::{path::PathBuf, fs::File};
use structopt::StructOpt;
use std::io::BufReader;

pub mod days;

#[derive(Debug, StructOpt)]
#[structopt(name = "advent", about = "run advent of code exercises")]
enum Advent {
    D1 {
        #[structopt(short, long)]
        part: Option<u32>,
        input_file: PathBuf,
    },
    D2 {
        #[structopt(short, long)]
        part: Option<u32>,
        input_file: PathBuf,
    },
    D3 {
        #[structopt(short, long)]
        part: Option<u32>,
        input_file: PathBuf,
    },
}

fn day1(input_file: BufReader<File>, part: u32) {
    match part {
        1 => {
            days::day1::part1(input_file);
        }
        2 => {
            days::day1::part2(input_file);
        }
        _ => {
            println!("part DNE");
        }
    }
}

fn day2(input_file: BufReader<File>, part: u32) {
    match part {
        1 => {
            days::day2::part1(input_file);
        }
        2 => {
            days::day2::part2(input_file);
        }
        _ => {
            println!("part DNE");
        }
    }
}

fn day3(input_file: BufReader<File>, part: u32) {
    match part {
        1 => {
            days::day3::part1(input_file);
        }
        2 => {
            days::day3::part2(input_file);
        }
        _ => {
            println!("part DNE");
        }
    }
}

fn main() {
    let opt = Advent::from_args();
    match opt {
        Advent::D1{ input_file, part } => {
            let file = BufReader::new(File::open(input_file).unwrap());
            day1(file, part.unwrap_or(1));
        }
        Advent::D2{ input_file, part } => {
            let file = BufReader::new(File::open(input_file).unwrap());
            day2(file, part.unwrap_or(1));
        }
        Advent::D3{ input_file, part } => {
            let file = BufReader::new(File::open(input_file).unwrap());
            day3(file, part.unwrap_or(1));
        }
    }
}

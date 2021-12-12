use std::{fs::File, io::BufRead};
use std::io::BufReader;

pub fn part1(input_file: BufReader<File>) {
    let mut acc: u32 = 0;
    let mut prev_line: Option<u32> = None;
    for line in input_file.lines() {
        let int_val = line.unwrap().trim().parse::<u32>().unwrap();
        match prev_line {
            None => {},
            Some(prev_val) => {
                if int_val > prev_val {
                    acc += 1;
                }
            }
        }
        prev_line = Some(int_val);
    }
    println!("{}", acc);
}

pub fn part2(input_file: BufReader<File>) {
    let mut window: [u32;3] = [0, 0, 0];
    let mut window_pop = 0;
    let mut acc: u32 = 0;
    let mut prev_sum: u32 = 0;
    for line in input_file.lines() {
        let int_val = line.unwrap().trim().parse::<u32>().unwrap();
        if window_pop < 2 {
            window[window_pop] = int_val;
            window_pop += 1;
            continue;
        } else if window_pop == 2 {
            window[window_pop] = int_val;
            window_pop += 1;
            prev_sum = window[0] + window[1] + window[2];
        } else {
            window[0] = window[1];
            window[1] = window[2];
            window[2] = int_val;
        }
        let new_sum = window[0] + window[1] + window[2];
        if prev_sum < new_sum {
            acc += 1;
        }
        prev_sum = new_sum;
    }
    println!("{}", acc);
}
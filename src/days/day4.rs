use std::{fs::File, io::BufRead};
use std::io::BufReader;

struct Board {
    values: [[usize; 5]; 5],
    ticks: [[bool; 5]; 5],
}

impl Board {
    fn from_strings(strs: Vec<String>) -> Board {
        let mut values = [[0;5];5];
        for (line, val_row) in strs.iter().zip(values.iter_mut()) {
            let tokens: Vec<&str> = line.split_whitespace().collect();
            for (token, val_ptr) in tokens.into_iter().zip(val_row.iter_mut()) {
                *val_ptr = token.parse().unwrap();
            }
        }
        Board { values: values, ticks: [[false;5];5] }
    }

    fn check(& self) -> bool {
        //let ldiag = [[0,0],[1,1],[2,2],[3,3],[4,4]];
        //let rdiag = [[0,4],[1,3],[2,2],[3,1],[4,0]];
        for row in self.ticks {
            if row.iter().all(|bool| *bool) {
                return true;
            }
        }
        let cols: Vec<Vec<bool>> = (0..self.ticks[0].len()).map(|j|{
            self.ticks.iter().map(|row | row[j]).collect()
        }).collect();
        for col in cols {
            if col.iter().all(|bool| *bool) {
                return true;
            }
        }
        /*
        if ldiag.iter().map(|c| self.ticks[c[0]][c[1]]).all(|bool| bool) {
            return true;
        }
        if rdiag.iter().map(|c| self.ticks[c[0]][c[1]]).all(|bool| bool) {
            return true;
        }
        */
        false
    }

    fn tick(&mut self, pick: usize) {
        for (val_row, tick_row) in self.values.iter().zip(self.ticks.iter_mut()) {
            for (val, tick) in val_row.iter().zip(tick_row.iter_mut()) {
                if *val == pick {
                    *tick = true;
                }
            }
        }
    }

    fn score(& self, pick: usize) -> usize {
        let mut acc: usize = 0;
        for (val_row, tick_row) in self.values.iter().zip(self.ticks.iter()) {
            for (val, tick) in val_row.iter().zip(tick_row.iter()) {
                if !*tick {
                    acc += val;
                }
            }
        }
        acc * pick
    }

    fn print_crappy(& self) {
        for (row, tick_row) in self.values.iter().zip(self.ticks.iter()) {
            println!("{:?} {:?}", row, tick_row);
        }
    }
}

pub fn part1(input_file: BufReader<File>) {
    let lines: Vec<String> = input_file.lines().map(|l| l.unwrap()).collect();
    let first_line = &lines[0];
    let board_lines = &lines[1..];
    let mut index: usize = 0;
    let mut board_index: usize = 0;
    let mut raw_board: Vec<String> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    while board_lines.len() > index {
        if board_lines[index] == "" {
            index += 1;
            continue;
        }
        raw_board.push(board_lines[index].clone());
        index += 1;
        board_index += 1;
        if board_index == 5 {
            boards.push(Board::from_strings(raw_board.clone()));
            board_index = 0;
            raw_board = Vec::new();
        }
    }
    let ticks: Vec<usize> = first_line.split(',').map(|s| s.parse().unwrap()).collect();
    for tick in ticks {
        for board in boards.iter_mut() {
            board.tick(tick);
            if board.check() {
                board.print_crappy();
                println!("{} => {}", tick, board.score(tick));
                return;
            }
        }
    }
}

pub fn part2(input_file: BufReader<File>) {
    let lines: Vec<String> = input_file.lines().map(|l| l.unwrap()).collect();
    let first_line = &lines[0];
    let board_lines = &lines[1..];
    let mut index: usize = 0;
    let mut board_index: usize = 0;
    let mut raw_board: Vec<String> = Vec::new();
    let mut boards: Vec<Board> = Vec::new();
    while board_lines.len() > index {
        if board_lines[index] == "" {
            index += 1;
            continue;
        }
        raw_board.push(board_lines[index].clone());
        index += 1;
        board_index += 1;
        if board_index == 5 {
            boards.push(Board::from_strings(raw_board.clone()));
            board_index = 0;
            raw_board = Vec::new();
        }
    }
    let ticks: Vec<usize> = first_line.split(',').map(|s| s.parse().unwrap()).collect();
    let mut boards_won: usize = 0;
    let mut winning_boards: Vec<bool> = boards.iter().map(|_| false).collect();
    for tick in ticks {
        let num_boards = boards.len();
        for (board, winning) in boards.iter_mut().zip(winning_boards.iter_mut()) {
            board.tick(tick);
            if board.check() {
                if !*winning {
                    boards_won += 1;
                    *winning = true;
                }
                if boards_won == num_boards {
                    board.print_crappy();
                    println!("{} => {}", tick, board.score(tick));
                    return;
                }
            }
        }
    }
}
use core::fmt;
use std::{fs::File, io::BufRead};
use std::io::BufReader;
use std::convert::TryInto;

struct Collector {
    size: usize,
    sums: Vec<i32>,
}

impl Collector {

    fn addval(&mut self, val: &Value) {
        if val.bits.len() != self.size {
            panic!();
        }
        for (i, bit) in val.bits.iter().enumerate() {
            match bit {
                false => { self.sums[i] -= 1 },
                true => { self.sums[i] += 1 },
            }
        }
    }

    fn from_values(vals: &Vec<Value>) -> Collector {
        let mut sums_vec = Vec::new();
        let rows = vals[0].bits.len();
        sums_vec.resize(rows, 0); 
        let mut coll = Collector {
            size: rows,
            sums: sums_vec,
        };
        for val in vals {
            coll.addval(val);
        }
        coll
    }

    fn to_value(& self) -> Value {
        Value { bits: self.sums.iter().map(|count| {
            if count >= &0 {
                return true;
            }
            return false;
        }).collect()}
    }
    
    fn get_gamma(& self) -> usize {
        self.to_value().to_number()
    }

    fn get_epsilon(& self) -> usize {
        self.to_value().flipped().to_number()
    }
}

#[derive(Clone)]
struct Value {
    bits: Vec<bool>,
}

impl Value {
    fn from_line(line: String) -> Value {
        Value { bits: line.chars().map(|char| {
            match char {
                '0' => false,
                '1' => true,
                _ => panic!(),
            }
        }).collect()}
    }

    fn to_number(& self) -> usize {
        let mut acc = 0;
        for (i, bit) in self.bits.iter().enumerate() {
            if *bit {
                acc += (2 as usize).pow((self.bits.len() - i - 1).try_into().unwrap());
            }
        }
        acc
    }

    fn flipped(& self) -> Value {
        Value { bits: self.bits.iter().map(|bit| !bit).collect() }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binary = self.bits.iter().map(|bit| {
            match bit {
                true => '1',
                false => '0',
            }
        }).collect::<String>();
        write!(f, "{:?} => {}", binary, self.to_number())
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binary = self.bits.iter().map(|bit| {
            match bit {
                true => '1',
                false => '0',
            }
        }).collect::<String>();
        write!(f, "{}", binary)
    }
}

pub fn part1(input_file: BufReader<File>) {
    let lines = input_file.lines();
    let vals = lines.map(|line| {
        Value::from_line(line.unwrap())
    }).collect();
    // TODO: from_values_iter
    let coll = Collector::from_values(&vals);
    println!("Gamma: {}, Epsilon: {}", coll.get_gamma(), coll.get_epsilon());
    println!("{}", coll.get_epsilon() * coll.get_gamma());
}

fn get_o2(vals: &Vec<Value>) -> usize {
    let mut index = 0;
    let mut in_vals: Vec<Value> = vals.to_vec();
    while in_vals.len() > 1 {
        let coll_val = Collector::from_values(&in_vals).to_value();
        let new_vals = in_vals.into_iter().filter(|val| val.bits[index] == coll_val.bits[index]).collect();
        in_vals = new_vals;
        index += 1;
    }
    in_vals[0].to_number()
}

fn get_co2(vals: &Vec<Value>) -> usize {
    let mut index = 0;
    let mut in_vals: Vec<Value> = vals.to_vec();
    while in_vals.len() > 1 {
        println!("{}", in_vals.len());
        let coll_val = Collector::from_values(&in_vals).to_value();
        let new_vals = in_vals.into_iter().filter(|val| val.bits[index] != coll_val.bits[index]).collect();
        in_vals = new_vals;
        index += 1;
    }
    println!("{}", in_vals[0]);
    in_vals[0].to_number()
}

pub fn part2(input_file: BufReader<File>) {
    let lines = input_file.lines();
    let vals: Vec<Value> = lines.map(|line| {
        Value::from_line(line.unwrap())
    }).collect();
    let o2 = get_o2(&vals);
    let co2 = get_co2(&vals);
    println!("O2: {}, CO2: {}, {}", o2, co2, o2 * co2);
}
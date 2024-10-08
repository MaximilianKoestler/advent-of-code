//! Advent of code 2015 day 23 part 1

use std::io::BufRead;

use day_23_1::{Instruction, Processor, Register};

fn main() {
    let file = std::fs::File::open("input/input.txt").unwrap();
    let reader = std::io::BufReader::new(file);
    let instructions: Vec<_> = reader
        .lines()
        .map(|line| { Instruction::try_from(line.unwrap().as_ref()) }.unwrap())
        .collect();

    let mut processor = Processor::new(&instructions);
    processor.run();

    println!("Register B: {}", processor.get_register(Register::B));
}

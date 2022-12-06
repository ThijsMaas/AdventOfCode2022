use std::env;

mod solutions;
use crate::solutions::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Expected a argument like 'day1'");
    }
    match args[1].as_str() {
        "day1" => day1::solution(),
        "day2" => day2::solution(),
        "day3" => day3::solution(),
        "day4" => day4::solution(),
        "day5" => day5::solution(),
        "day6" => day6::solution(),
        _ => panic!("Argument not supported"),
    }
}

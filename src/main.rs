use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;

fn main() {
    let day: u8 = env::args().nth(1).expect("Day needs to be passed.")
                             .parse::<u8>().expect("Day was not a number.");
	let input = &fs::read_to_string(format!("./src/day{}/input.txt", day)).expect("Problem reading input.");

    match day {
		1 => day1::solve(input),
        2 => day2::solve(input),
        3 => day3::solve(input),
        _ => panic!("Day {} not found", day),
    }
}

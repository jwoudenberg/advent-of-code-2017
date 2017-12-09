use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

fn to_digit(n: char) -> u32 {
    return n.to_digit(10).unwrap();
}

fn get_input() -> Result<String, Box<Error>>{
    let mut f = File::open("input.txt")?;
    let mut input = String::new();
    f.read_to_string(&mut input)?;
    return Ok(input);
}

fn calculate_sum(input: String) -> u32 {
    let numbers = input.trim().chars().map(to_digit);
    let shifted = numbers.clone().cycle().skip(1);
    let pairs = numbers.zip(shifted);
    return pairs.map(|(x, y)| if x == y { x } else { 0 }).sum();
}

fn main() {
    match get_input() {
        Ok(input) => println!("{}", calculate_sum(input)),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}

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

fn calculate_sum(input: String, displace: usize) -> u32 {
    let numbers = input.trim().chars().map(to_digit);
    let shifted = numbers.clone().cycle().skip(displace);
    let pairs = numbers.zip(shifted);
    return pairs.map(|(x, y)| if x == y { x } else { 0 }).sum();
}

fn part1() -> Result<(), Box<Error>> {
    let input = get_input()?;
    println!("Sum, part 1: {}", calculate_sum(input, 1));
    Ok(())
}

fn part2() -> Result<(), Box<Error>> {
    let input = get_input()?;
    let half_size = input.len() / 2;
    println!("Sum, part 2: {}", calculate_sum(input, half_size));
    Ok(())
}

fn main() {
    match part1() {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err),
    }
    match part2() {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err),
    }
}

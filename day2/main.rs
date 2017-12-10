use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;

fn get_input() -> Result<Vec<Vec<u32>>, Box<Error>>{
    let mut f = File::open("input.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    let input = buffer.lines().map(parse_line).collect();
    return Ok(input);
}

fn parse_line(line: &str) -> Vec<u32> {
    return line.split('\t').map(parse_int).collect();
}

fn parse_int(str: &str) -> u32 {
    return str.parse().unwrap();
}

fn checksum(input: Vec<Vec<u32>>) -> u32 {
    return input.iter().map(line_checksum).sum();
}

fn line_checksum(line: &Vec<u32>) -> u32 {
    let min = line.iter().min().unwrap();
    let max = line.iter().max().unwrap();
    return max - min;
}

fn main() {
    match get_input() {
        Ok(input) => println!("{}", checksum(input)),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}

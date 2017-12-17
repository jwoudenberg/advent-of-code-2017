use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter::Iterator;
use std::iter;

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

fn part1(input: Vec<Vec<u32>>) -> u32 {
    return input.iter().map(line_checksum).sum();
}

fn line_checksum(line: &Vec<u32>) -> u32 {
    let min = line.iter().min().unwrap();
    let max = line.iter().max().unwrap();
    return max - min;
}

fn part2(input: Vec<Vec<u32>>) -> u32 {
    return input.iter().map(line_checksum_divide).sum();
}

fn line_checksum_divide (xs: &Vec<u32>) -> u32 {
    let iter = xs.iter();
    let repeats = iter::repeat(iter.clone());
    let (x, y) = iter.zip(repeats)
            .flat_map(|(x, ys)| iter::repeat(x).zip(ys))
            .find(|&(x, y)| x != y && x % y == 0)
            .unwrap();
    x / y
}

fn main() {
    match get_input() {
        Ok(input) => println!("Part1: {}", part1(input)),
        Err(e) => println!("Error: {}", e.to_string()),
    }
    match get_input() {
        Ok(input) => println!("Part2: {}", part2(input)),
        Err(e) => println!("Error: {}", e.to_string()),
    }
}

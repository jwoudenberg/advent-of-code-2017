#[macro_use] extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use regex::Regex;

fn main() {
    match part1() {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err),
    }
}

#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    weight: u32,
    supporting: Vec<&'a str>
}

fn part1() -> Result<(), Box<Error>> {
    let input = read_input()?;
    let programs: Vec<Program> = input.lines().map(parse_line).collect();
    let names: HashSet<&&str> = programs.iter().map(|p| &p.name).collect();
    let supported: HashSet<&&str> = programs.iter().flat_map(|p| p.supporting.iter()).collect();
    let root = names.difference(&supported).next().unwrap();
    println!("Root: {}", root);
    Ok(())
}

fn parse_line(line: &str) -> Program {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([a-z]+) \((\d+)\)(?: -> ([a-z, ]+))?$").unwrap();
    }
    let cap = RE.captures_iter(line).nth(0).unwrap();
    let name = cap.get(1).unwrap().as_str();
    let weight = cap.get(2).unwrap().as_str().parse().unwrap();
    let supporting: Vec<&str> = match cap.get(3) {
        None => vec![],
        Some(str) => str.as_str().split(", ").collect(),
    };
    Program {
        name: name,
        weight: weight,
        supporting: supporting,
    }
}

fn read_input() -> Result<String, Box<Error>> {
    let mut f = File::open("input.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

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

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Memory {
    banks: Vec<u32>
}

impl Memory {
    fn from_vec(vec: Vec<u32>) -> Memory {
        Memory { banks: vec }
    }
}

fn part1() -> Result<(), Box<Error>> {
    let input = read_input()?;
    let mut mem = Memory::from_vec(parse_line(&input));
    let cycles = redistribute_until_cycle(&mut mem);
    println!("Loop detected after: {}", cycles);
    Ok(())
}

fn redistribute_until_cycle(mem: &mut Memory) -> u32 {
    let mut seen: HashSet<Memory> = HashSet::new();
    let mut count = 0;
    while seen.insert(mem.clone()) {
        count += 1;
        redistribute(mem);
    }
    count
}

fn part2() -> Result<(), Box<Error>> {
    let input = read_input()?;
    let mut mem = Memory::from_vec(parse_line(&input));
    let cycles = redistribute_until_cycle_part_2(&mut mem);
    println!("Loop length: {}", cycles);
    Ok(())
}

fn redistribute_until_cycle_part_2(mem: &mut Memory) -> u32 {
    let mut seen: HashMap<Memory, u32> = HashMap::new();
    let mut count = 0;
    loop {
        match seen.insert(mem.clone(), count) {
            None => (),
            Some(x) => return count - x,
        }
        count += 1;
        redistribute(mem);
    }
}

fn redistribute(mem: &mut Memory) -> () {
    let banks = mem.banks.clone();
    let mut blocks = *banks.iter().max().unwrap();
    let mut pos = banks.iter().position(|x| x == &blocks).unwrap();
    mem.banks[pos] = 0;
    while blocks > 0 {
        pos = (pos + 1) % banks.len();
        blocks -= 1;
        mem.banks[pos] += 1;
    }
}

fn read_input() -> Result<String, Box<Error>> {
    let mut f = File::open("input.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse_line(line: &str) -> Vec<u32> {
    return line.split('\t').map(parse_int).collect();
}

fn parse_int(str: &str) -> u32 {
    return str.trim().parse().unwrap();
}

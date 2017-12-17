use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
extern crate itertools;

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

fn part1() -> Result<(), Box<Error>> {
    let input = read_input()?;
    let instructions: Vec<i32> = input.lines().map(parse_int).collect();
    let initial = Zipper::from_vec(instructions).unwrap();
    let iter = itertools::unfold(initial, step);
    let jumps = iter.count() + 1;
    println!("Jumps, part1: {}", jumps);
    Ok(())
}

fn part2() -> Result<(), Box<Error>> {
    let input = read_input()?;
    let instructions: Vec<i32> = input.lines().map(parse_int).collect();
    let initial = Zipper::from_vec(instructions).unwrap();
    let iter = itertools::unfold(initial, step_part_2);
    let jumps = iter.count() + 1;
    println!("Jumps, part2: {}", jumps);
    Ok(())
}

struct Zipper<T> {
    items: Vec<T>,
    focus: usize
}

impl<T> Zipper<T> {
    fn from_vec(vec: Vec<T>) -> Option<Zipper<T>> {
        if vec.len() == 0 {
            None
        } else {
            Some(
                Zipper {
                    items: vec,
                    focus: 0,
                }
            )
        }
    }

    fn get(&self) -> &T {
        self.items.iter().nth(self.focus).unwrap()
    }

    fn set(&mut self, x: T) -> () {
        self.items[self.focus] = x;
    }

    fn step(&mut self, n: i32) -> Option<usize> {
        let new_focus = (self.focus as i32) + n;
        if new_focus < 0 || (new_focus as usize) >= self.items.len() {
            None
        } else {
            self.focus = new_focus as usize;
            Some(new_focus as usize)
        }
    }
}

fn step(instructions: &mut Zipper<i32>) -> Option<usize> {
    let instruction = instructions.get().clone();
    instructions.set(instruction + 1);
    instructions.step(instruction)
}

fn step_part_2(instructions: &mut Zipper<i32>) -> Option<usize> {
    let instruction = instructions.get().clone();
    let new_instruction = if instruction >= 3 { instruction - 1 } else { instruction + 1 };
    instructions.set(new_instruction);
    instructions.step(instruction)
}

fn read_input() -> Result<String, Box<Error>> {
    let mut f = File::open("input.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse_int(str: &str) -> i32 {
    return str.parse().unwrap();
}

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn read_input() -> Result<String, Box<Error>> {
    let mut f = File::open("input.txt")?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse<'a>(input: &'a str) -> Box<Iterator<Item=Vec<&str>> + 'a> {
    Box::new(input.lines().map(parse_line))
}

fn parse_line(line: &str) -> Vec<&str> {
    line.split(" ").collect()
}

fn valid(phrase: &Vec<&str>) -> bool {
    let set: HashSet<&&str> = phrase.iter().collect();
    set.len() == phrase.len()
}

fn sort(word: &&str) -> Vec<char> {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort();
    chars
}

fn valid_anagram(phrase: &Vec<&str>) -> bool {
    let set: HashSet<Vec<char>> = phrase.iter().map(sort).collect();
    set.len() == phrase.len()
}

fn valid_phrase_count() -> Result<(), Box<Error>> {
    let input = read_input()?;
    println!("Valid phrases, method 1: {}", parse(&input).filter(valid).count());
    println!("Valid phrases, method 2: {}", parse(&input).filter(valid_anagram).count());
    Ok(())
}

fn main() {
    match valid_phrase_count() {
        Ok(()) => (),
        Err(err) => println!("Error: {}", err),
    }
}

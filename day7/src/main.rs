#[macro_use] extern crate lazy_static;
extern crate regex;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::hash::Hash;
use regex::Regex;

fn main() {
    match solution() {
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

fn solution() -> Result<(), Box<Error>> {
    let input = read_input()?;
    let programs: HashMap<&str, Program> = input.lines().map(parse_line).map(|p| (p.name, p)).collect();
    let root = find_root(&programs);
    println!("Root: {}", root.name);
    let mut inbalanced: Vec<&Program> = programs.values()
        .filter(|p| is_inbalanced(p, &programs))
        .collect();
    inbalanced.sort_by_key(|p| height(p, &programs));
    let highest_inbalance = inbalanced.iter().next().unwrap();
    let corrected_weight = balance(highest_inbalance, &programs);
    println!("Corrected weight: {}", corrected_weight);
    Ok(())
}


fn balance(program: &Program, programs: &HashMap<&str, Program>) -> u32 {
    let mut weights: HashMap<u32, Vec<&Program>> = HashMap::new();
    for c in children(program, &programs).iter() {
        add_to(&mut weights, weight(c, programs), c);
    }
    let (wrong_weight, wrong_child) = weights.iter().min_by_key(|&(_w, ps)| ps.len()).unwrap();
    let (correct_weight, _) = weights.iter().max_by_key(|&(_w, ps)| ps.len()).unwrap();
    wrong_child.iter().next().unwrap().weight + correct_weight - wrong_weight
}

fn height(program: &Program, programs: &HashMap<&str, Program>) -> u32 {
    match parent(program, programs) {
        None => 0,
        Some(p) => height(p, programs),
    }
}

fn parent<'a>(program: &Program<'a>, programs: &'a HashMap<&str, Program<'a>>) -> Option<&'a Program<'a>> {
    programs.values().find(|p| p.supporting.contains(&program.name))
}

fn find_root<'a>(programs: &'a HashMap<&str, Program>) -> &'a Program<'a> {
    let names: HashSet<&&str> = programs.keys().collect();
    let supported: HashSet<&&str> = programs.values().flat_map(|p| p.supporting.iter()).collect();
    let root = names.difference(&supported).next().unwrap();
    programs.get(*root).unwrap()
}

fn is_inbalanced(program: &Program, programs: &HashMap<&str, Program>) -> bool {
    let mut weights = HashMap::new();
    children(program, programs).iter()
        .map(|c| weight(c, programs))
        .for_each(|w| increment_key(&mut weights, w));
    weights.len() > 1
}

fn increment_key<T>(map: &mut HashMap<T, u32>, key: T) -> () where T: Eq + Hash {
    let count = map.entry(key).or_insert(0);
    *count += 1;
}

fn add_to<K, V>(map: &mut HashMap<K, Vec<V>>, key: K, value: V) -> () where K: Eq + Hash {
    let vec: &mut Vec<V> = map.entry(key).or_insert(vec![]);
    vec.push(value);
}

fn children<'a>(program: &Program<'a>, programs: &'a HashMap<&str, Program<'a>>) -> Vec<&'a Program<'a>> {
    program.supporting.iter()
        .map(|n| programs.get(n).unwrap())
        .collect()
}

fn weight(program: &Program, programs: &HashMap<&str, Program>) -> u32 {
    let child_weight: u32 = children(program, programs).iter().map(|p| weight(p, programs)).sum();
    program.weight + child_weight
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

use std::iter;
use std::iter::Take;
use std::iter::Repeat;
use std::collections::HashMap;

const INPUT: i32 = 361527;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

#[derive(Clone, Copy)]
struct Side {
    ln: i32,
    dir: Direction,
}

fn side_iter(side: Side)  -> Take<Repeat<Direction>> {
    iter::repeat(side.dir).take(side.ln as usize)
}

fn next_side(side: Side) -> Side {
    match side {
        Side { dir: Direction::Right, ln: n } => Side { dir: Direction::Up, ln: n },
        Side { dir: Direction::Up, ln: n } => Side { dir: Direction::Left, ln: n + 1 },
        Side { dir: Direction::Left, ln: n } => Side { dir: Direction::Down, ln: n },
        Side { dir: Direction::Down, ln: n } => Side { dir: Direction::Right, ln: n + 1 },
    }
}

struct Spiral {
    current: Side
}

impl Spiral {
    fn new() -> Spiral {
        let start_side = Side { ln: 1, dir: Direction::Right };
        Spiral { current: start_side }
    }
}

impl Iterator for Spiral {
    type Item = Side;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.current.clone();
        self.current = next_side(self.current);
        Some(current)
    }
}

fn step(coords: (i32, i32), direction: Direction) -> (i32, i32) {
    let (x, y) = coords;
    match direction {
        Direction::Right => (x + 1, y),
        Direction::Up => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Down => (x, y - 1),
    }
}

fn distance(coords: (i32, i32)) -> i32 {
    let (x, y) = coords;
    x + y
}

fn part1() {
    let steps = INPUT - 1; // We start with index 1.
    let start = (0, 0);
    let coords = Spiral::new().flat_map(side_iter).take(steps as usize).fold(start, step);
    println!("Distance: {}", distance(coords));
}

struct Grid {
    location: (i32, i32),
    grid: HashMap<(i32, i32), i32>,
}

impl Grid {
    fn new() -> Grid {
        let mut grid = HashMap::new();
        let start = (0, 0);
        grid.insert(start, 1);
        Grid {
            location: start,
            grid: grid,
        }
    }

    fn step(&mut self, dir: Direction) -> i32 {
        self.location = step(self.location, dir);
        let (x, y) = self.location;
        let surrounding = [
            self.get((x-1, y-1)),
            self.get((x-1, y)),
            self.get((x-1, y+1)),
            self.get((x, y-1)),
            self.get((x, y+1)),
            self.get((x+1,y-1)),
            self.get((x+1, y)),
            self.get((x+1, y+1)),
        ].iter().sum();
        self.set(surrounding);
        return surrounding;
    }

    fn set(&mut self, value: i32) -> &mut Grid {
        self.grid.insert(self.location, value);
        self
    }

    fn get(&self, coords: (i32, i32)) -> i32 {
        match self.grid.get(&coords) {
            Some(x) => *x,
            None => 0,
        }
    }
}


fn part2() {
    let mut grid = Grid::new();
    let result = Spiral::new()
        .flat_map(side_iter)
        .map(|dir| grid.step(dir))
        .find(|v| v > &INPUT);
    match result {
        Some(output) => println!("Output: {}", output),
        None => println!("Could not calculate output.!"),
    }
}

fn main() {
    part1();
    part2()
}

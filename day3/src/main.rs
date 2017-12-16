use std::iter;
use std::iter::Take;
use std::iter::Repeat;

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

fn main() {
    let n = 361527;
    let steps = n - 1; // We start with index 1.
    let start = (0, 0);
    let coords = Spiral::new().flat_map(side_iter).take(steps as usize).fold(start, step);
    println!("Distance: {}", distance(coords));
}

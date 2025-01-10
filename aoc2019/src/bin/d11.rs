use itertools::Itertools;
use std::collections::HashMap;
use std::io::stdin;
use std::iter::once;
use std::sync::mpsc;
use utils::{Direction, Grid, Intcode, Point};

struct State {
    painted: HashMap<Point, isize>,
    pos: Point,
    dir: Direction,
}

impl State {
    fn new() -> Self {
        Self { painted: Default::default(), pos: Point(0, 0), dir: Direction::UP }
    }

    fn color(&self) -> isize {
        self.painted.get(&self.pos).copied().unwrap_or_default()
    }

    fn advance(&mut self, col: isize, rot: isize) {
        assert!(col == 0 || col == 1);
        assert!(rot == 0 || rot == 1);
        self.painted.insert(self.pos, col);
        self.dir = if rot == 0 { self.dir.counter_rotate() } else { self.dir.rotate() };
        self.pos = self.pos + self.dir;
    }

    fn draw(&self) -> Grid<char> {
        let Point(x, y) = self.pos;
        let minx = self.painted.keys().map(|p| p.0).chain(once(x)).min().unwrap() - 1;
        let maxx = self.painted.keys().map(|p| p.0).chain(once(x)).max().unwrap() + 1;
        let miny = self.painted.keys().map(|p| p.1).chain(once(y)).min().unwrap() - 1;
        let maxy = self.painted.keys().map(|p| p.1).chain(once(y)).max().unwrap() + 1;
        let offset = Direction::new(-minx, -miny);
        let mut grid = Grid::new('.', (maxx - minx + 1) as usize, (maxy - miny + 1) as usize);
        for (&p, &t) in self.painted.iter() {
            if t == 1 {
                grid[p + offset] = '#';
            }
        }
        grid[self.pos + offset] = match self.dir {
            Direction::UP => '^',
            Direction::RIGHT => '>',
            Direction::DOWN => 'v',
            Direction::LEFT => '<',
            d => panic!("Unexpected direction {d:?}"),
        };
        grid
    }
}

fn generate(program: &Intcode, init_tile: isize) {
    let mut program = program.clone();
    let mut state = State::new();
    let (tx, rx) = mpsc::channel();
    tx.send(init_tile).unwrap();
    for (c, r) in program.run(rx).tuples() {
        state.advance(c, r);
        tx.send(state.color()).unwrap();
    }
    println!("{}", state.draw());
    println!("Painted tiles: {}\n", state.painted.len());
}

fn main() {
    let program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    generate(&program, 0);
    generate(&program, 1);
}

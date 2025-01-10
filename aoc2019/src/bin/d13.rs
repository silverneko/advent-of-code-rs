use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::stdin;
use utils::{Direction, Grid, Intcode, Point};

#[derive(Default)]
struct State {
    entities: HashMap<Point, isize>,
    score: isize,
}

impl State {
    fn update(&mut self, x: isize, y: isize, t: isize) {
        if (x, y) == (-1, 0) {
            self.score = t;
        } else {
            self.entities.insert((y, x).into(), t);
        }
    }

    fn ball(&self) -> Point {
        *self.entities.iter().find(|(_, &t)| t == 4).unwrap().0
    }

    fn bar(&self) -> Point {
        *self.entities.iter().find(|(_, &t)| t == 3).unwrap().0
    }

    fn print(&self) {
        let minx = self.entities.keys().map(|p| p.0).min().unwrap();
        let maxx = self.entities.keys().map(|p| p.0).max().unwrap();
        let miny = self.entities.keys().map(|p| p.1).min().unwrap();
        let maxy = self.entities.keys().map(|p| p.1).max().unwrap();
        let offset = Direction::new(-minx, -miny);
        let mut grid = Grid::new(' ', (maxx - minx + 1) as usize, (maxy - miny + 1) as usize);
        for (&p, &t) in self.entities.iter() {
            grid[p + offset] = match t {
                0 => ' ',
                1 => '#',
                2 => '*',
                3 => '=',
                4 => '@',
                _ => panic!("Unexpected entity {t}"),
            };
        }
        println!("{grid}Score: {}", self.score);
    }
}

struct InputController<'a> {
    state: &'a RefCell<State>,
}

impl Iterator for InputController<'_> {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.state.borrow();
        state.print();
        Some(state.ball().1.cmp(&state.bar().1) as isize)
    }
}

fn part1(program: &Intcode) -> usize {
    program.clone().run(std::iter::empty()).tuples().filter(|&(_, _, t)| t == 2).count()
}

fn main() {
    let mut program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    dbg!(part1(&program));

    program.code[0] = 2;
    let state = RefCell::new(State::default());
    for (x, y, t) in program.run(InputController { state: &state }).tuples() {
        state.borrow_mut().update(x, y, t);
    }
    state.borrow().print();
}

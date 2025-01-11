use itertools::Itertools;
use std::cell::RefCell;
use std::collections::HashMap;
use std::io::{stdin, stdout};
use utils::{move_up_and_clear_lines, Direction, Grid, Intcode, Point};

#[derive(Default)]
struct State {
    entities: HashMap<Point, isize>,
    score: isize,
    last_print_lines: usize,
}

impl State {
    fn update(&mut self, x: isize, y: isize, t: isize) {
        if (x, y) == (-1, 0) {
            self.score = t;
        } else {
            self.entities.insert((y, x).into(), t);
        }
    }

    fn should_move(&self) -> isize {
        let ball = self.entities.iter().find(|(_, &t)| t == 4).unwrap().0 .1;
        let bar = self.entities.iter().find(|(_, &t)| t == 3).unwrap().0 .1;
        (ball - bar).signum()
    }

    fn print(&mut self) {
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
        move_up_and_clear_lines(stdout(), self.last_print_lines as u32);
        println!("{grid}Score: {}", self.score);
        self.last_print_lines = grid.h + 1;
        std::thread::sleep(std::time::Duration::from_millis(2));
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
    let joystick = std::iter::repeat_with(|| state.borrow().should_move());
    for (x, y, t) in program.run(joystick).tuples() {
        let mut state = state.borrow_mut();
        state.update(x, y, t);
        state.print();
    }
}

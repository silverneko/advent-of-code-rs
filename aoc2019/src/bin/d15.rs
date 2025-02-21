use itertools::Itertools;
use std::collections::{HashMap, VecDeque};
use std::io::{stdin, stdout};
use utils::{terminal, Deferred, Direction, Grid, Intcode, Point};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Entity {
    Empty = b'.',
    Tank = b'O',
    Wall = b'#',
}

struct Drone<'a>(Deferred<'a>);

impl Drone<'_> {
    fn try_move(&mut self, d: isize) -> Entity {
        self.0.send(d);
        match self.0.iter().next() {
            Some(0) => Entity::Wall,
            Some(1) => Entity::Empty,
            Some(2) => Entity::Tank,
            err => panic!("Unexpected signal {err:?}"),
        }
    }
}

struct State<'a> {
    drone: Drone<'a>,
    entities: HashMap<Point, Entity>,
    pos: Point,
}

impl<'a> State<'a> {
    fn new(drone: Drone<'a>) -> Self {
        Self { drone, entities: HashMap::from([(Point(0, 0), Entity::Empty)]), pos: Point(0, 0) }
    }

    fn print(&self) {
        let (minx, maxx) = self.entities.keys().map(|p| p.0).minmax().into_option().unwrap();
        let (miny, maxy) = self.entities.keys().map(|p| p.1).minmax().into_option().unwrap();
        let offset = Direction::new(minx, miny);
        let mut grid = Grid::new(' ', (maxx - minx + 1) as usize, (maxy - miny + 1) as usize);
        for (&p, &t) in self.entities.iter() {
            grid[p - offset] = t as u8 as char;
        }
        grid[self.pos - offset] = '@';
        terminal::home(stdout());
        println!("{grid}");
        std::thread::sleep(std::time::Duration::from_millis(1));
    }

    fn put(&mut self, p: Point, e: Entity) {
        self.entities.insert(p, e);
    }

    fn get(&self, p: Point) -> Option<Entity> {
        self.entities.get(&p).copied()
    }

    fn slam(&mut self) {
        self.print();
        for ((d, dv), (b, bv)) in [
            ((1, Direction::UP), (2, Direction::DOWN)),
            ((2, Direction::DOWN), (1, Direction::UP)),
            ((3, Direction::LEFT), (4, Direction::RIGHT)),
            ((4, Direction::RIGHT), (3, Direction::LEFT)),
        ] {
            let np = self.pos + dv;
            if self.get(np).is_none() {
                let e = self.drone.try_move(d);
                self.put(np, e);
                if matches!(e, Entity::Empty | Entity::Tank) {
                    self.pos = np;
                    self.slam();
                    self.drone.try_move(b);
                    self.pos = np + bv;
                }
            }
        }
        self.print();
    }

    fn bfs(&self) -> (usize, usize) {
        let s: Point = *self.entities.iter().find(|(_, &v)| v == Entity::Tank).unwrap().0;
        let mut q = VecDeque::from([(0, s)]);
        let mut visited = HashMap::new();
        while let Some((d, p)) = q.pop_front() {
            if visited.contains_key(&p) {
                continue;
            }
            visited.insert(p, d);
            for dv in Direction::cardinals() {
                let np = p + dv;
                if matches!(self.get(np), Some(Entity::Empty | Entity::Tank)) {
                    q.push_back((d + 1, np));
                }
            }
        }
        (visited[&Point(0, 0)], visited.into_values().max().unwrap())
    }
}

fn main() {
    let mut program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    let mut state = State::new(Drone(program.deferred_run()));
    terminal::clear(stdout());
    state.slam();
    assert_eq!(state.pos, Point(0, 0));
    dbg!(state.bfs());
}

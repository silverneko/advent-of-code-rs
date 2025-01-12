use itertools::{iproduct, Itertools};
use std::collections::HashMap;
use std::io::{stdin, stdout, BufRead};
use utils::{terminal, BatchLines, Direction, Grid, Intcode, Point};

#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(u8)]
enum Entity {
    Space = b'.',
    Scaffold = b'#',
}

struct State {
    grid: Grid<Entity>,
}

impl State {
    fn parse(reader: impl BufRead) -> Self {
        let grid = reader
            .lines()
            .map(|e| {
                e.unwrap()
                    .bytes()
                    .map(|c| match c {
                        b'.' => Entity::Space,
                        b'#' | b'<' | b'>' | b'^' | b'v' => Entity::Scaffold,
                        b => panic!("Unexpected input {b}"),
                    })
                    .collect()
            })
            .collect::<Vec<_>>()
            .into();
        Self { grid }
    }

    fn part1(&self) -> usize {
        iproduct!(0..self.grid.h, 0..self.grid.w)
            .filter(|&p| {
                self.grid[p] == Entity::Scaffold
                    && Direction::cardinals()
                        .into_iter()
                        .all(|dv| self.grid.get(Point::from(p) + dv) == Some(&Entity::Scaffold))
            })
            .map(|(y, x)| x * y)
            .sum()
    }
}

fn main() {
    let mut program: Intcode = stdin().lines().next().unwrap().unwrap().parse().unwrap();
    let output: String = program.clone().run(std::iter::empty()).map(|s| s as u8 as char).collect();
    let state = State::parse(output.trim_ascii().as_bytes());
    dbg!(state.part1());

    let prompt_response = HashMap::from([
        ("Main:", "A,B,A,C,B,C,A,B,A,C\n"),
        ("Function A:", "R,10,L,8,R,10,R,4\n"),
        ("Function B:", "L,6,L,6,R,10\n"),
        ("Function C:", "L,6,R,12,R,12,R,10\n"),
        ("Continuous video feed?", "y\n"),
    ]);
    program.code[0] = 2;
    let deferred = program.deferred_run();

    let mut first_frame = true;
    let mut new_frame = false;
    let mut output = deferred.iter().peekable();
    let lines = output
        .by_ref()
        .peeking_take_while(|&i| i < 256 && (i as u8).is_ascii())
        .map(|a| a as u8 as char)
        .batch_lines();
    for line in lines {
        let resp = prompt_response.get(line.trim_ascii());
        if !first_frame && new_frame && resp.is_none() {
            std::thread::sleep(std::time::Duration::from_millis(5));
            terminal::home(stdout());
        }
        println!("{line}");
        if let Some(resp) = resp {
            deferred.send_seq(resp.bytes().map(|b| b.into()));
            println!("{resp}");
            terminal::clear(stdout());
            first_frame = false;
        }
        new_frame = line.is_empty();
    }
    println!("Part2 answer: {}", output.next().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"
..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..
"
        .trim_ascii();
        assert_eq!(State::parse(input.as_bytes()).part1(), 76);
    }
}

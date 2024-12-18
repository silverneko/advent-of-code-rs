use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::io::{stdin, BufRead};
use utils::{Direction, Grid, Point};

fn parse_input(reader: impl BufRead) -> Grid<u8> {
    reader.lines().map(|e| e.unwrap().into()).collect::<Vec<Vec<u8>>>().into()
}

// A node in our graph is a position (Point) plus an orientation (Point).
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct Node(Point, Point);

// (weight, to, from)
#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct State(u32, Node, Option<Node>);

fn solve(grid: &Grid<u8>) -> (u32, u32) {
    let s = Node(
        (0..grid.h).cartesian_product(0..grid.w).find(|&p| grid[p] == b'S').unwrap().into(),
        Direction::RIGHT,
    );
    // Priority queue for Dijkstra algorithm from S -> E.
    let mut pq: BinaryHeap<Reverse<State>> = BinaryHeap::from([Reverse(State(0, s, None))]);
    // Back links for back tracking.
    let mut backtrack: HashMap<Node, Vec<Node>> = HashMap::new();
    // BFS queue for backtracking from E -> S.
    let mut bq: VecDeque<Node> = VecDeque::new();

    let mut shortest_path_length = None;
    while let Some(Reverse(State(dist, target, source))) = pq.pop() {
        if backtrack.contains_key(&target) {
            continue;
        }
        let mut source_nodes: Vec<Node> = match source {
            Some(node) => vec![node],
            _ => vec![],
        };
        while matches!(pq.peek(), Some(&Reverse(State(d, t, _))) if (d, t) == (dist, target)) {
            if let Some(Reverse(State(_, _, Some(node)))) = pq.pop() {
                source_nodes.push(node);
            }
        }
        backtrack.insert(target, source_nodes);

        let Node(p, d) = target;
        match grid[p] {
            b'E' => {
                if *shortest_path_length.get_or_insert(dist) == dist {
                    // Put all equally good path into queue.
                    bq.push_back(target);
                    continue;
                }
                break;
            }
            b'#' => continue,
            b'.' | b'S' => {}
            t => panic!("unexpected token: {:?}", char::from_u32(t.into())),
        }
        pq.push(Reverse(State(dist + 1, Node(p + d, d), Some(target))));
        pq.push(Reverse(State(dist + 1000, Node(p, d.rotate()), Some(target))));
        pq.push(Reverse(State(dist + 1000, Node(p, d.rotate().rotate().rotate()), Some(target))));
    }

    let mut area_set: HashSet<Point> = HashSet::new();
    while let Some(s) = bq.pop_front() {
        area_set.insert(s.0);
        let Some(back) = backtrack.get_mut(&s) else { unreachable!() };
        for &pv in back.iter() {
            bq.push_back(pv);
        }
        // We only need to backtrack once, so just clear the back link after visiting a node.
        back.clear();
    }

    (shortest_path_length.unwrap(), area_set.len() as u32)
}

fn main() {
    println!("{:?}", solve(&parse_input(stdin().lock())));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_sample_small() {
        let input = r"
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
"
        .trim();
        let reader = Cursor::new(input);
        let grid = parse_input(reader);
        assert_eq!(solve(&grid), (7036, 45));
    }

    #[test]
    fn test_sample_large() {
        let input = r"
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
"
        .trim();
        let reader = Cursor::new(input);
        let grid = parse_input(reader);
        assert_eq!(solve(&grid), (11048, 64));
    }
}

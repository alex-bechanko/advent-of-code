/*
Advent of Code solutions written in the Rust programming language
Copyright (C) 2025 Alexander Bechanko

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Wall,
    Space,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    fn rotate_clockwise(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn rotate_counterclockwise(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn into_tuple(self) -> (isize, isize) {
        match self {
            Self::Up => (-1, 0),
            Self::Down => (1, 0),
            Self::Left => (0, -1),
            Self::Right => (0, 1),
        }
    }

    fn all() -> Vec<Self> {
        vec![Self::Up, Self::Right, Self::Down, Self::Left]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Facing<T>(T, Direction);

struct Maze {
    contents: Vec<Vec<Cell>>,
}
impl Maze {
    fn get(&self, row: usize, col: usize) -> Option<&Cell> {
        self.contents.get(row).and_then(|r| r.get(col))
    }

    fn neighbors(&self, &Facing(point, direction): &Facing<Point>) -> Vec<Cost> {
        let mut ns = vec![];
        // left
        let d = direction.rotate_counterclockwise();
        let cost = 1001;
        if let Some(pt) = point.linear_move(d) {
            if let Some(Cell::Space) = self.get(pt.row, pt.col) {
                let n = Cost(Facing(pt, d), cost);
                ns.push(n);
            }
        }

        // right
        let d = direction.rotate_clockwise();
        let cost = 1001;
        if let Some(pt) = point.linear_move(d) {
            if let Some(Cell::Space) = self.get(pt.row, pt.col) {
                let n = Cost(Facing(pt, d), cost);
                ns.push(n);
            }
        }

        // forward
        let d = direction;
        let cost = 1;
        if let Some(pt) = point.linear_move(d) {
            if let Some(Cell::Space) = self.get(pt.row, pt.col) {
                let n = Cost(Facing(pt, d), cost);
                ns.push(n);
            }
        }

        ns
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    row: usize,
    col: usize,
}
impl Point {
    fn new(row: usize, col: usize) -> Point {
        Point { row, col }
    }

    fn linear_move(&self, dir: Direction) -> Option<Point> {
        let (dr, dc) = dir.into_tuple();
        let row = self.row.checked_add_signed(dr)?;
        let col = self.col.checked_add_signed(dc)?;
        Some(Point::new(row, col))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let row = self.row;
        let col = self.col;
        write!(f, "({row}, {col})")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Cost(Facing<Point>, u32);

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.1.cmp(&self.1))
    }
}

impl Ord for Cost {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.1.cmp(&self.1)
    }
}

type Node = Facing<Point>;
fn dijkstra(maze: &Maze, start: Facing<Point>) -> (HashMap<Node, u32>, HashMap<Node, Vec<Node>>) {
    use std::cmp::Ordering::{Equal, Greater, Less};

    let mut distances = HashMap::new();
    distances.insert(start, 0);

    let mut predecessors: HashMap<Facing<Point>, Vec<Facing<Point>>> = HashMap::new();

    let mut pq: BinaryHeap<Cost> = BinaryHeap::new();
    pq.push(Cost(start, 0));

    while let Some(current) = pq.pop() {
        let Cost(current_node @ Facing(current_point, direction), current_distance) = current;

        if &current_distance > distances.get(&current_node).unwrap_or(&u32::MAX) {
            continue;
        }

        let ns: Vec<_> = maze
            .neighbors(&current_node)
            .into_iter()
            .filter_map(|n| {
                let Cost(neighbor_node @ Facing(neighbor, neighbor_direction), weight) = n;
                let distance = current_distance + weight;
                let &neighbor_distance = distances.get(&neighbor_node).unwrap_or(&u32::MAX);

                match distance.cmp(&neighbor_distance) {
                    Greater => return None,
                    Less => {
                        distances.insert(neighbor_node, distance);
                        predecessors.insert(
                            Facing(neighbor, neighbor_direction),
                            vec![Facing(current_point, direction)],
                        );
                    }
                    Equal => {
                        if let Some(ps) =
                            predecessors.get_mut(&Facing(neighbor, neighbor_direction))
                        {
                            if !ps.contains(&current_node) {
                                ps.push(current_node);
                            }
                        }
                    }
                }

                Some(Cost(Facing(neighbor, neighbor_direction), distance))
            })
            .collect();

        pq.extend(ns);
    }

    (distances, predecessors)
}

fn shortest_paths(
    predecessors: &HashMap<Facing<Point>, Vec<Facing<Point>>>,
    distances: &HashMap<Facing<Point>, u32>,
    start: Point,
    start_direction: Direction,
    end: Point,
) -> Vec<Vec<Facing<Point>>> {
    let start = Facing(start, start_direction);
    let mut shortest = vec![];

    let shortest_distance = Direction::all()
        .into_iter()
        .filter_map(|d| distances.get(&Facing(end, d)).copied())
        .min()
        .expect("No shortest path");

    let mut stack: Vec<(Vec<Facing<Point>>, Facing<Point>)> = vec![];
    for d in Direction::all() {
        let e = Facing(end, d);
        if *distances.get(&e).unwrap_or(&u32::MAX) == shortest_distance {
            stack.push((vec![e], e));
        }
    }

    while let Some((current_path, current_node)) = stack.pop() {
        if current_node == start {
            let mut current_path = current_path.clone();
            current_path.reverse();
            shortest.push(current_path);
        } else if let Some(predecessor_nodes) = predecessors.get(&current_node) {
            for &n in predecessor_nodes {
                let mut path = current_path.clone();
                path.push(n);
                stack.push((path, n));
            }
        }
    }

    shortest
}

#[allow(dead_code)]
fn debug_maze(maze: &Maze, highlight: &HashSet<Point>) -> String {
    let height = maze.contents.len();
    let width = maze.contents[0].len();
    let mut s = String::with_capacity(height * (width + 1));
    for row in 0..maze.contents.len() {
        for col in 0..maze.contents[0].len() {
            let c = match (
                maze.get(row, col),
                highlight.contains(&Point::new(row, col)),
            ) {
                (Some(&Cell::Space), true) => 'O',
                (Some(&Cell::Space), false) => '.',
                (_, _) => '#',
            };
            s.push(c);
        }
        s.push('\n');
    }

    s
}

fn parse(input: &str) -> (Maze, Point, Point) {
    let mut contents = vec![];
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (i, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            let cell = match c {
                '#' => Cell::Wall,
                '.' => Cell::Space,
                'S' => {
                    start = Some(Point::new(i, j));
                    Cell::Space
                }
                'E' => {
                    end = Some(Point::new(i, j));
                    Cell::Space
                }
                c => panic!("Unexpected character {c:?}"),
            };

            row.push(cell);
        }
        contents.push(row);
    }

    let start = start.expect("No start found");
    let end = end.expect("No end found");
    let maze = Maze { contents };

    (maze, start, end)
}

/// # Panics
pub fn part1(input: &str) -> u32 {
    let (maze, start, end) = parse(input);
    let (distances, _) = dijkstra(&maze, Facing(start, Direction::Right));
    // now to see what the distances are for each possible facing of the end point
    Direction::all()
        .into_iter()
        .filter_map(|d| distances.get(&Facing(end, d)).copied())
        .min()
        .expect("No shortest path")
}

pub fn part2(input: &str) -> usize {
    let (maze, start, end) = parse(input);
    let (distances, predecessors) = dijkstra(&maze, Facing(start, Direction::Right));

    let nodes: HashSet<Point> =
        shortest_paths(&predecessors, &distances, start, Direction::Right, end)
            .into_iter()
            .flatten()
            .map(|Facing(p, _)| p)
            .collect();

    nodes.len()
}

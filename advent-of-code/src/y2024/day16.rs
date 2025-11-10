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

use crate::y2024::direction::CardinalDirection;
use crate::y2024::maze::Maze;
use std::collections::HashSet;

struct GridMaze {
    contents: Vec<Vec<char>>,
}
impl GridMaze {
    fn get(&self, position: Point) -> Option<&char> {
        self.contents
            .get(position.y)
            .and_then(|r| r.get(position.x))
    }
}

impl Maze for GridMaze {
    type Node = (CardinalDirection, Point);
    fn neighbors(&self, n: &Self::Node) -> Vec<(u32, Self::Node)> {
        let &(direction, position) = n;
        let mut ns = vec![];

        // left
        let d = direction.rotate_counterclockwise();
        let cost = 1001;
        if let Some(pt) = position.linear_move(d)
            && let Some('.') = self.get(pt)
        {
            ns.push((cost, (d, pt)));
        }

        // right
        let d = direction.rotate_clockwise();
        let cost = 1001;
        if let Some(pt) = position.linear_move(d)
            && let Some('.') = self.get(pt)
        {
            ns.push((cost, (d, pt)));
        }

        // forward
        let d = direction;
        let cost = 1;
        if let Some(pt) = position.linear_move(d)
            && let Some('.') = self.get(pt)
        {
            ns.push((cost, (d, pt)));
        }

        ns
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn linear_move(&self, dir: CardinalDirection) -> Option<Point> {
        let (dx, dy) = dir.into();
        let x = self.x.checked_add_signed(dx)?;
        let y = self.y.checked_add_signed(dy)?;
        Some(Point { x, y })
    }
}

#[allow(dead_code)]
fn debug_maze(maze: &GridMaze, highlight: &HashSet<Point>) -> String {
    let height = maze.contents.len();
    let width = maze.contents[0].len();
    let mut s = String::with_capacity(height * (width + 1));
    for row in 0..maze.contents.len() {
        for col in 0..maze.contents[0].len() {
            let pos = Point { y: row, x: col };
            let c = match (maze.get(pos), highlight.contains(&pos)) {
                (Some(&'.'), true) => 'O',
                (Some(&'.'), false) => '.',
                (_, _) => '#',
            };
            s.push(c);
        }
        s.push('\n');
    }

    s
}

fn parse(input: &str) -> (GridMaze, Point, Point) {
    let mut contents = vec![];
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                '#' => '#',
                '.' => '.',
                'S' => {
                    start = Some(Point { x, y });
                    '.'
                }
                'E' => {
                    end = Some(Point { x, y });
                    '.'
                }
                c => panic!("Unexpected character {c:?}"),
            };

            row.push(cell);
        }
        contents.push(row);
    }

    let start = start.expect("No start found");
    let end = end.expect("No end found");
    let maze = GridMaze { contents };

    (maze, start, end)
}

/// # Panics
pub fn part1(input: &str) -> u32 {
    let (maze, start, end) = parse(input);
    let (distances, _) = maze.dijkstra(&(CardinalDirection::Right, start));

    // now to see what the distances are for each possible facing of the end point
    CardinalDirection::all()
        .into_iter()
        .filter_map(|d| distances.get(&(d, end)).copied())
        .min()
        .expect("No shortest path")
}

/// # Panics
pub fn part2(input: &str) -> usize {
    let (maze, start, end) = parse(input);
    let start = (CardinalDirection::Right, start);
    let (dist, prev) = maze.dijkstra(&start);

    let paths = CardinalDirection::all().into_iter().flat_map(|d| {
        let end = (d, end);
        maze.shortest_paths(&dist, &prev, &start, &end)
    });

    let shortest = paths
        .clone()
        .map(|p| p.len())
        .min()
        .expect("No shortest path");

    let nodes: HashSet<Point> = paths
        .filter(|p| p.len() == shortest)
        .flatten()
        .map(|(_, pt)| pt)
        .collect();

    nodes.len()
}

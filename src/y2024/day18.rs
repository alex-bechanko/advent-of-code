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

use std::collections::{HashMap, HashSet, VecDeque};

fn parse(input: &str) -> Option<Vec<(usize, usize)>> {
    let mut pts = vec![];
    for l in input.lines() {
        let (a, b) = l.split_once(',')?;
        let a = a.parse().ok()?;
        let b = b.parse().ok()?;

        pts.push((a, b));
    }

    Some(pts)
}

#[allow(dead_code)]
fn debug_grid(blocks: &[(usize, usize)], path: &[(usize, usize)], width: usize, height: usize) {
    let mut s = String::with_capacity(height * (width + 1));
    for row in 0..height {
        for col in 0..width {
            let pt = (col, row);
            if path.contains(&pt) {
                s.push('O');
            } else if blocks.contains(&pt) {
                s.push('#');
            } else {
                s.push('.');
            }
        }
        s.push('\n');
    }
    println!("{s}");
}

fn shortest_path(
    corrupted_memory: &[(usize, usize)],
    goal: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    let start = (0, 0);

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut parent: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut queue = VecDeque::new();
    queue.push_back(start);

    while let Some(position @ (x, y)) = queue.pop_front() {
        if position == goal {
            break;
        }

        let offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        for (dx, dy) in offsets {
            let Some(x2) = x.checked_add_signed(dx) else {
                continue;
            };
            let Some(y2) = y.checked_add_signed(dy) else {
                continue;
            };
            if x2 > goal.0 || y2 > goal.1 {
                continue;
            }

            let p = (x2, y2);

            if visited.contains(&p) || corrupted_memory.contains(&p) {
                continue;
            }

            visited.insert(p);
            parent.insert(p, position);
            queue.push_back(p);
        }
    }

    // now to count the steps for the path
    let mut path = vec![goal];
    let mut end = goal;
    while end != start {
        end = *parent.get(&end)?;
        path.push(end);
    }

    path.reverse();
    // debug_grid(corrupted_memory, path, goal.0 + 1, goal.1 + 1);

    Some(path)
}

/// # Panics
pub fn part1(input: &str) -> usize {
    let goal = (70, 70);
    let num_bytes = 1024;

    let pts = parse(input).expect("Failed to parse input");
    let pts: &[(usize, usize)] = &pts[0..num_bytes];

    let path = shortest_path(pts, goal).expect("No path found");

    // subtract one to not count the starting position as a move
    path.len() - 1
}

/// # Panics
pub fn part2(input: &str) -> String {
    let goal = (70, 70);
    let pts = parse(input).expect("Failed to parse input");

    let mut path: Vec<(usize, usize)> =
        shortest_path(&[], goal).expect("No obstacles should mean there is a path");
    for i in 1..=pts.len() {
        // check if the additional point will require a recalculation of the shortest path
        if !path.contains(&pts[i - 1]) {
            continue;
        }

        let updated_path = shortest_path(&pts[0..i], goal);

        if let Some(p) = updated_path {
            path = p;
        } else {
            let block = pts[i - 1];
            return format!("{},{}", block.0, block.1);
        }
    }

    "Always a path".to_owned()
}

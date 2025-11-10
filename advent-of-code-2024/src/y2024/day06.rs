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

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Guard(Direction, usize, usize);

impl Guard {
    /// Moves the guard.
    /// If the guard is forced to move outside the maze then return None
    /// Changes in x, y, and direction are considered steps
    fn step(&self, maze: &Vec<Vec<bool>>) -> Option<Guard> {
        let &Guard(dir, gx, gy) = self;
        let (dx, dy) = dir.to_vector();

        let x = gx.checked_add_signed(dx)?;
        if x >= maze[0].len() {
            return None;
        }

        let y = gy.checked_add_signed(dy)?;
        if y >= maze.len() {
            return None;
        }

        if maze[y][x] {
            return Guard(dir.rotate(), gx, gy).step(maze);
        }

        Some(Guard(dir, x, y))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_vector(self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
            Self::Right => (1, 0),
        }
    }

    fn rotate(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

fn parse(input: &str) -> (Guard, Vec<Vec<bool>>) {
    let grid_width = input.find('\n').unwrap();
    let grid_height = input.chars().filter(|&c| c == '\n').count();

    let mut x: usize = 0;
    let mut y: usize = 0;
    let mut maze = vec![vec![false; grid_width]; grid_height];
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            if c == '#' {
                maze[row][col] = true;
            } else if c == '^' {
                (x, y) = (col, row);
            }
        }
    }

    (Guard(Direction::Up, x, y), maze)
}

pub fn part1(input: &str) -> usize {
    let (guard, maze) = parse(input);
    let positions = path(guard, &maze).expect("Guard is in a loop");

    let coords: HashSet<(usize, usize)> =
        HashSet::from_iter(positions.iter().map(|&Guard(_, x, y)| (x, y)));

    coords.len()
}

/// Computes the full path the guard will take.
/// If the guard is forced into a loop, return None.
fn path(mut guard: Guard, maze: &Vec<Vec<bool>>) -> Option<Vec<Guard>> {
    let mut positions: Vec<Guard> = Vec::with_capacity(5000);
    positions.push(guard);

    while let Some(next) = guard.step(maze) {
        guard = next;
        if positions.contains(&next) {
            return None;
        }

        positions.push(next);
    }

    Some(positions)
}

fn cast(mut guard: Guard, maze: &Vec<Vec<bool>>) -> Option<Guard> {
    let Guard(curr_direction, _, _) = guard;

    while let Some(next) = guard.step(maze) {
        let Guard(next_direction, _, _) = next;
        if next_direction != curr_direction {
            return Some(next);
        }

        guard = next;
    }

    None
}

fn walk(mut guard: Guard, maze: &Vec<Vec<bool>>) -> Option<HashSet<Guard>> {
    let mut positions = HashSet::with_capacity(200);
    positions.insert(guard);

    while let Some(next) = cast(guard, maze) {
        guard = next;
        if !positions.insert(next) {
            return None;
        }
    }

    Some(positions)
}

pub fn part2(input: &str) -> usize {
    let (guard, mut maze) = parse(input);
    let travelled = path(guard, &maze).expect("Starting grid is already a cycle");

    let blocks: HashSet<(usize, usize)> =
        HashSet::from_iter(travelled.iter().map(|&Guard(_, x, y)| (x, y)));

    let mut first_guard_positions = HashMap::new();
    for w in travelled.windows(2) {
        let past = w[0];
        let curr = w[1];
        let Guard(_, x, y) = curr;
        let pos = (x, y);

        first_guard_positions.entry(pos).or_insert(past);
    }

    let mut count = 0;
    for pos in blocks.iter() {
        let &(x, y) = pos;
        if let Some(&guard) = first_guard_positions.get(pos) {
            maze[y][x] = true;
            if walk(guard, &maze).is_none() {
                count += 1;
            }
            maze[y][x] = false;
        }
    }

    count
}

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

type Position<T> = (T, T);
type Velocity<T> = (T, T);
type Robot<T> = (Position<T>, Velocity<T>);

fn parse_pair(
    input: &str,
    prefix: &str,
    grid_width: usize,
    grid_height: usize,
) -> Option<(usize, usize)> {
    let pos = input.strip_prefix(prefix)?;
    let (x, y) = pos.split_once(',')?;
    let x: isize = x.parse().ok()?;
    let x: usize = if x < 0 {
        grid_width.checked_add_signed(x)?
    } else {
        usize::try_from(x).ok()?
    };

    let y: isize = y.parse().ok()?;
    let y: usize = if y < 0 {
        grid_height.checked_add_signed(y)?
    } else {
        usize::try_from(y).ok()?
    };

    Some((x, y))
}
fn parse_robot(input: &str, grid_width: usize, grid_height: usize) -> Option<Robot<usize>> {
    let (pos, vel) = input.split_once(' ')?;
    let pos = parse_pair(pos, "p=", grid_width, grid_height)?;
    let vel = parse_pair(vel, "v=", grid_width, grid_height)?;
    Some((pos, vel))
}

fn parse(input: &str, grid_width: usize, grid_height: usize) -> Vec<Robot<usize>> {
    input
        .lines()
        .map(|l| parse_robot(l, grid_width, grid_height).expect("Couldn't parse robot"))
        .collect()
}

pub fn part1(input: &str) -> usize {
    use std::cmp::Ordering::{Equal, Greater, Less};
    let grid_width = 101;
    let grid_height = 103;
    let half_width = grid_width / 2;
    let half_height = grid_height / 2;
    parse(input, grid_width, grid_height)
        .into_iter()
        .filter_map(|((x, y), (dx, dy))| {
            let x = (x + dx * 100) % grid_width;
            let y = (y + dy * 100) % grid_height;
            match (x.cmp(&half_width), y.cmp(&half_height)) {
                (Equal, _) | (_, Equal) => None,
                (Less, Less) => Some(1),
                (Greater, Less) => Some(2),
                (Less, Greater) => Some(3),
                (Greater, Greater) => Some(4),
            }
        })
        .fold(vec![0; 4], |mut quads, q| {
            quads[q - 1] += 1;
            quads
        })
        .into_iter()
        .product()
}

fn line(grid: &[Vec<bool>]) -> bool {
    let check: Vec<isize> = vec![-4, -3, -2, -1, 0, 1, 2, 3, 4];

    for row in 4..(grid.len() - 4) {
        // allowind this for now while I rethink how to restructure this check best
        #[allow(clippy::needless_range_loop)]
        for col in 0..grid[0].len() {
            if check
                .iter()
                .all(|&i| grid[row.checked_add_signed(i).unwrap()][col])
            {
                return true;
            }
        }
    }

    false
}

fn place_robots(robots: &[Robot<usize>], grid_width: usize, grid_height: usize) -> Vec<Vec<bool>> {
    let mut grid = vec![vec![false; grid_width]; grid_height];
    for robot in robots {
        grid[robot.0.1][robot.0.0] = true;
    }

    grid
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<bool>]) {
    let mut s = String::with_capacity(grid.len() * (grid[0].len() + 1));
    for row in 0..grid.len() {
        #[allow(clippy::needless_range_loop)]
        for col in 0..grid[0].len() {
            let x = grid[row][col];
            if x {
                s += "x";
            } else {
                s += ".";
            }
        }
        s += "\n";
    }

    println!("{s}");
}

pub fn part2(input: &str) -> usize {
    let grid_width = 101;
    let grid_height = 103;
    let mut robots = parse(input, grid_width, grid_height);

    let mut t = 0;
    while !line(&place_robots(&robots, grid_width, grid_height)) {
        for robot in &mut robots {
            let ((x, y), (dx, dy)) = *robot;
            let x = (x + dx) % grid_width;
            let y = (y + dy) % grid_height;

            *robot = ((x, y), (dx, dy));
        }
        t += 1;
    }

    t
}

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

use std::collections::HashSet;

fn parse(input: &str) -> (usize, usize, Vec<(usize, usize, char)>) {
    let antenna = input
        .lines()
        .enumerate()
        .flat_map(|(i, l)| l.chars().enumerate().map(move |(j, c)| (j, i, c)))
        .filter(|(_, _, c)| *c != '.')
        .collect();

    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    (width, height, antenna)
}

fn antinode(x1: usize, y1: usize, x2: usize, y2: usize, i: isize) -> (isize, isize) {
    let x1 = isize::try_from(x1).unwrap();
    let y1 = isize::try_from(y1).unwrap();

    let x2 = isize::try_from(x2).unwrap();
    let y2 = isize::try_from(y2).unwrap();

    let ax: isize = x2 - (x1 - x2).signum() * (x2 - x1).abs() * i;
    let ay: isize = y2 - (y1 - y2).signum() * (y2 - y1).abs() * i;

    (ax, ay)
}

pub fn part1(input: &str) -> usize {
    let (width, height, antenna) = parse(input);
    let width = isize::try_from(width).unwrap();
    let height = isize::try_from(height).unwrap();

    let mut nodes = HashSet::new();
    for i in 0..antenna.len() - 1 {
        for j in i + 1..antenna.len() {
            let (x1, y1, c1) = antenna[i];
            let (x2, y2, c2) = antenna[j];

            if c1 != c2 {
                continue;
            }

            let (ax, ay) = antinode(x1, y1, x2, y2, 1);
            if ax > -1 && ax < width && ay > -1 && ay < height {
                nodes.insert((usize::try_from(ax).unwrap(), usize::try_from(ay).unwrap()));
            }

            let (ax, ay) = antinode(x1, y1, x2, y2, -2);
            if ax > -1 && ax < width && ay > -1 && ay < height {
                nodes.insert((usize::try_from(ax).unwrap(), usize::try_from(ay).unwrap()));
            }
        }
    }
    nodes.len()
}

pub fn part2(input: &str) -> usize {
    let (width, height, antenna) = parse(input);
    let width = isize::try_from(width).unwrap();
    let height = isize::try_from(height).unwrap();

    let mut nodes = HashSet::new();
    for i in 0..antenna.len() - 1 {
        for j in i + 1..antenna.len() {
            let (x1, y1, c1) = antenna[i];
            let (x2, y2, c2) = antenna[j];

            if c1 != c2 {
                continue;
            }

            for ind in 0..width + height {
                let (ax, ay) = antinode(x1, y1, x2, y2, ind);
                if ax > -1 && ax < width && ay > -1 && ay < height {
                    nodes.insert((usize::try_from(ax).unwrap(), usize::try_from(ay).unwrap()));
                } else {
                    break;
                }
            }

            for ind in (-width * height..0).rev() {
                let (ax, ay) = antinode(x1, y1, x2, y2, ind);
                if ax > -1 && ax < width && ay > -1 && ay < height {
                    nodes.insert((usize::try_from(ax).unwrap(), usize::try_from(ay).unwrap()));
                } else {
                    break;
                }
            }
        }
    }

    nodes.len()
}

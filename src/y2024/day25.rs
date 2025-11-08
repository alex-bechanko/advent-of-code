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

type Locks = Vec<Vec<isize>>;
type Keys = Vec<Vec<isize>>;

fn parse(input: &str) -> (Locks, Keys) {
    let mut locks = vec![];
    let mut keys = vec![];

    for block in input.split("\n\n") {
        let is_lock = block.starts_with("#####");

        let block: Vec<Vec<char>> = block.split("\n").map(|l| l.chars().collect()).collect();

        let mut heights = vec![-1isize; 5];
        for row in block {
            for (i, c) in row.iter().copied().enumerate() {
                heights[i] += if c == '#' { 1 } else { 0 };
            }
        }

        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (locks, keys)
}

pub fn part1(input: &str) -> usize {
    let num_pins = 5;
    let (locks, keys) = parse(input);

    let mut used: HashSet<(usize, usize)> = HashSet::new();
    for (l, lock) in locks.iter().enumerate() {
        for (k, key) in keys.iter().enumerate() {
            let fits = lock.iter().zip(key).all(|(l, k)| l + k <= num_pins);

            if fits {
                used.insert((l, k));
            }
        }
    }

    used.len()
}

pub fn part2(_input: &str) -> String {
    "No puzzle".into()
}

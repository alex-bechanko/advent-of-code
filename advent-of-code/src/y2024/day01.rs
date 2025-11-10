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

fn parse_pair(line: &str) -> (u32, u32) {
    let (left, right) = line.split_once("   ").unwrap();

    (left.parse().unwrap(), right.parse().unwrap())
}

pub fn part1(input: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = input.lines().map(parse_pair).unzip();
    left.sort_unstable();
    right.sort_unstable();

    std::iter::zip(&left, &right)
        .map(|(l, &r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let (left, right): (Vec<u32>, Vec<u32>) = input.lines().map(parse_pair).unzip();
    let mut frequency = std::collections::HashMap::new();
    for n in &right {
        frequency.entry(n).and_modify(|x| *x += 1).or_insert(1);
    }

    left.iter()
        .fold(0, |acc, n| acc + n * frequency.get(&n).unwrap_or(&0))
}

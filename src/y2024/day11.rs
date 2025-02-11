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

use std::collections::HashMap;

fn parse(input: &str) -> Vec<u128> {
    input
        .split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn num_digits(stone: u128) -> u32 {
    stone.ilog10() + 1
}

fn num_stones_memo(cache: &mut HashMap<(u128, usize), usize>, stone: u128, blinks: usize) -> usize {
    if let Some(&num) = cache.get(&(stone, blinks)) {
        return num;
    }

    if blinks == 0 {
        return 1;
    }

    let ans = if stone == 0 {
        num_stones_memo(cache, 1, blinks - 1)
    } else if num_digits(stone) % 2 == 0 {
        let left = stone / 10u128.pow(num_digits(stone) / 2);
        let right = stone % 10u128.pow(num_digits(stone) / 2);

        num_stones_memo(cache, left, blinks - 1) + num_stones_memo(cache, right, blinks - 1)
    } else {
        num_stones_memo(cache, stone * 2024, blinks - 1)
    };

    cache.insert((stone, blinks), ans);
    ans
}

pub fn part1(input: &str) -> usize {
    let stones = parse(input);
    let mut memo = HashMap::new();
    stones
        .into_iter()
        .map(|s| num_stones_memo(&mut memo, s, 25))
        .sum()
}

pub fn part2(input: &str) -> usize {
    let stones = parse(input);
    let mut memo = HashMap::new();
    stones
        .into_iter()
        .map(|s| num_stones_memo(&mut memo, s, 75))
        .sum()
}

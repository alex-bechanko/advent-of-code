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

fn joltage(bank: &[usize], digits: usize) -> Option<usize> {
    if digits == 1 {
        return bank.iter().copied().max();
    }

    let lefts = &bank[..bank.len() - digits + 1];
    let (i, left) = lefts.iter().copied().enumerate().max_by(|a, b| {
        let elem_order = a.1.cmp(&b.1);
        if elem_order == std::cmp::Ordering::Equal {
            b.0.cmp(&a.0)
        } else {
            elem_order
        }
    })?;

    let rest = joltage(&bank[i + 1..], digits - 1)?;
    let ans = left * 10usize.pow(digits as u32 - 1) + rest;

    Some(ans)
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .iter()
        .filter_map(|bank| joltage(bank, 2))
        .sum()
}

pub fn part2(input: &str) -> usize {
    parse(input)
        .iter()
        .filter_map(|bank| joltage(bank, 12))
        .sum()
}

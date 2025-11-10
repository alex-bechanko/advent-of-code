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

fn parse(input: &str) -> (Vec<Vec<bool>>, Vec<Vec<usize>>) {
    let mut input = input.split("\n\n");
    let rules_input = input.next().unwrap();
    let pages = input.next().unwrap();

    let mut rules: Vec<Vec<bool>> = vec![vec![false; 100]; 100];

    for line in rules_input.lines() {
        let mut nums = line.split("|").map(|x| x.parse().unwrap());
        let n1: usize = nums.next().unwrap();
        let n2: usize = nums.next().unwrap();

        rules[n1][n2] = true;
    }

    let pages: Vec<Vec<usize>> = pages
        .lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, pages)
}

pub fn part1(input: &str) -> usize {
    let (rules, pages) = parse(input);

    pages
        .iter()
        .filter(|nums| nums.is_sorted_by(|&a, &b| !rules[b][a]))
        .map(|nums| nums[nums.len() / 2])
        .sum()
}

pub fn part2(input: &str) -> usize {
    let (rules, mut pages) = parse(input);

    pages
        .iter_mut()
        .filter(|nums| !nums.is_sorted_by(|&a, &b| !rules[b][a]))
        .map(|nums| {
            nums.sort_unstable_by(|&a, &b| {
                if rules[b][a] {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            });

            nums[nums.len() / 2]
        })
        .sum()
}

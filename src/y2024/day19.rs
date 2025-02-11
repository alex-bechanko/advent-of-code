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
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

struct SolutionError;

impl TryFrom<char> for Color {
    type Error = SolutionError;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'w' => Ok(Self::White),
            'u' => Ok(Self::Blue),
            'b' => Ok(Self::Black),
            'r' => Ok(Self::Red),
            'g' => Ok(Self::Green),
            _ => Err(SolutionError),
        }
    }
}

type Towel = Vec<Color>;
type Pattern = Vec<Color>;
fn parse(input: &str) -> Option<(Vec<Towel>, Vec<Pattern>)> {
    let (towels_input, designs_input) = input.split_once("\n\n")?;
    let mut towels = vec![];
    for towel_input in towels_input.split(", ") {
        let mut towel = vec![];
        for color_input in towel_input.chars() {
            let color: Color = color_input.try_into().ok()?;
            towel.push(color);
        }
        towels.push(towel);
    }

    let mut designs = vec![];
    for design_input in designs_input.lines() {
        let mut design = vec![];
        for color_input in design_input.chars() {
            let color: Color = color_input.try_into().ok()?;
            design.push(color);
        }
        designs.push(design);
    }

    Some((towels, designs))
}

fn towel_pattern_exists(
    towels: &Vec<Vec<Color>>,
    design: &Vec<Color>,
    design_index: usize,
) -> bool {
    if design_index >= design.len() {
        // solution found, build pattern
        return true;
    }

    for i in 0..towels.len() {
        let towel = &towels[i];

        let n = towel
            .iter()
            .zip(&design[design_index..])
            .filter(|(tc, dc)| tc == dc)
            .count();
        if n == towel.len() {
            // towel match, recurse
            let design_index = design_index + towel.len();

            if towel_pattern_exists(towels, design, design_index) {
                return true;
            }
        }
    }

    false
}

fn count_towel_pattern(
    towels: &Vec<Vec<Color>>,
    design: &Vec<Color>,
    design_index: usize,
    memo: &mut HashMap<usize, usize>,
) -> usize {
    if design_index >= design.len() {
        return 1;
    }

    if let Some(&n) = memo.get(&design_index) {
        return n;
    }

    let mut count = 0;
    for i in 0..towels.len() {
        let towel = &towels[i];
        let n = towel
            .iter()
            .zip(&design[design_index..])
            .filter(|(tc, dc)| tc == dc)
            .count();
        if n == towel.len() {
            // towel match, recurse
            let design_index = design_index + towel.len();

            count += count_towel_pattern(towels, design, design_index, memo);
        }
    }

    memo.insert(design_index, count);
    count
}

/// # Panics
pub fn part1(input: &str) -> usize {
    let (towels, designs) = parse(input).expect("Failed to parse input");

    designs
        .iter()
        .filter(|d| towel_pattern_exists(&towels, d, 0))
        .count()
}

/// # Panics
pub fn part2(input: &str) -> usize {
    let (towels, designs) = parse(input).expect("Failed to parse input");
    designs
        .iter()
        .map(|d| count_towel_pattern(&towels, d, 0, &mut HashMap::new()))
        .sum()
}

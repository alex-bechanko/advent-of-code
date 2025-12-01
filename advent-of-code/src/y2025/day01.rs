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

const DIAL_SIZE: isize = 100;
const START_DIAL_POSITION: isize = 50;

fn parse(input: &str) -> impl Iterator<Item = isize> {
    input.lines().filter_map(|line| {
        if let Some(num) = line.strip_prefix("R") {
            num.parse().ok()
        } else if let Some(num) = line.strip_prefix("L") {
            num.parse::<isize>().map(|n| -n).ok()
        } else {
            None
        }
    })
}

pub fn part1(input: &str) -> usize {
    parse(input)
        .scan(START_DIAL_POSITION, |dial, rotation| {
            *dial = (*dial + rotation) % DIAL_SIZE;
            Some(*dial)
        })
        .filter(|&n| n == 0)
        .count()
}

pub fn part2(input: &str) -> isize {
    parse(input)
        .scan(START_DIAL_POSITION, |dial, rotation| {
            let next = *dial + (rotation % DIAL_SIZE);
            let crossed = next >= DIAL_SIZE || (next <= 0 && *dial > 0);
            let num_zeros = (rotation / DIAL_SIZE).abs() + crossed as isize;

            *dial = next.rem_euclid(DIAL_SIZE);
            Some(num_zeros)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let tests = vec![
            ("L50", 1),
            ("R50", 1),
            ("L51", 1),
            ("R51", 1),
            ("R150", 2),
            ("R151", 2),
            ("L150", 2),
            ("L151", 2),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, part2(input));
        }
    }
}

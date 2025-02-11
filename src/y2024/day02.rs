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

fn parse_intlist(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn safe_sequence_dampen(seq: &[i32], idx: Option<usize>) -> bool {
    if seq.len() < 2 {
        return true;
    }

    let ordering = match idx {
        Some(0) => seq[1].cmp(&seq[2]),
        Some(1) => seq[0].cmp(&seq[2]),
        _ => seq[0].cmp(&seq[1]),
    };

    let max_threshold = 3;
    let min_threshold = 1;

    let dampened_idxs: Vec<_> = (0..seq.len()).filter(|&x| Some(x) != idx).collect();

    for i in 0..dampened_idxs.len() - 1 {
        let (l, r) = (seq[dampened_idxs[i]], seq[dampened_idxs[i + 1]]);

        if ordering != l.cmp(&r) || l.abs_diff(r) > max_threshold || l.abs_diff(r) < min_threshold {
            return false;
        }
    }

    true
}

fn safe_sequence_part2(seq: &[i32]) -> bool {
    std::iter::once(None)
        .chain((0..seq.len()).map(Some))
        .any(|idx| safe_sequence_dampen(seq, idx))
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .map(parse_intlist)
        .filter(|s| safe_sequence_dampen(s, None))
        .count()
}

pub fn part2(input: &str) -> usize {
    input
        .lines()
        .map(parse_intlist)
        .filter(|v| safe_sequence_part2(v))
        .count()
}

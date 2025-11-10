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

type Point = (usize, usize);
const NUMERIC_KEYPAD: [(char, Point); 11] = [
    ('7', (0, 0)),
    ('8', (1, 0)),
    ('9', (2, 0)),
    ('4', (0, 1)),
    ('5', (1, 1)),
    ('6', (2, 1)),
    ('1', (0, 2)),
    ('2', (1, 2)),
    ('3', (2, 2)),
    ('0', (1, 3)),
    ('A', (2, 3)),
];

const NUMERIC_KEYPAD_GAP: Point = (0, 3);

const DIRECTIONAL_KEYPAD: [(char, Point); 5] = [
    ('^', (1, 0)),
    ('A', (2, 0)),
    ('<', (0, 1)),
    ('v', (1, 1)),
    ('>', (2, 1)),
];

const DIRECTIONAL_KEYPAD_GAP: Point = (0, 0);

fn directions(&gap: &Point, &(start_x, start_y): &Point, &(end_x, end_y): &Point) -> Vec<String> {
    let dx = end_x.abs_diff(start_x);
    let dy = end_y.abs_diff(start_y);

    let left = if end_x < start_x {
        "<".repeat(dx)
    } else {
        String::new()
    };
    let right = if start_x < end_x {
        ">".repeat(dx)
    } else {
        String::new()
    };
    let down = if start_y < end_y {
        "v".repeat(dy)
    } else {
        String::new()
    };
    let up = if end_y < start_y {
        "^".repeat(dy)
    } else {
        String::new()
    };

    let mut dirs = vec![];
    if (start_x, end_y) != gap {
        dirs.push(String::from(&down) + &up + &left + &right + "A");
    }

    if (end_x, start_y) != gap {
        dirs.push(left + &right + &down + &up + "A");
    }

    dirs
}

fn keypad_instructions(pad: &[(char, Point)], gap: &Point) -> HashMap<(char, char), Vec<String>> {
    let mut pad_intructions = HashMap::new();
    for &(k1, v1) in pad {
        for &(k2, v2) in pad {
            let d = directions(gap, &v1, &v2);
            pad_intructions.insert((k1, k2), d);
        }
    }

    pad_intructions
}

fn instructions_length(
    cache: &mut HashMap<(String, usize), usize>,
    number_keypad: &HashMap<(char, char), Vec<String>>,
    directional_keypad: &HashMap<(char, char), Vec<String>>,
    sequence: &str,
    num_robots: usize,
    start: bool,
) -> usize {
    if num_robots == 0 {
        return sequence.chars().count();
    }

    if let Some(len) = cache.get(&(sequence.to_string(), num_robots)) {
        return *len;
    }

    let keypad = if start {
        number_keypad
    } else {
        directional_keypad
    };

    let mut prev = 'A';
    let mut length = 0;
    for c in sequence.chars() {
        let e = (prev, c);
        let seqs = &keypad[&e];

        let sublen = seqs
            .iter()
            .map(|seq| {
                instructions_length(
                    cache,
                    number_keypad,
                    directional_keypad,
                    seq,
                    num_robots - 1,
                    false,
                )
            })
            .min()
            .expect("directions array was empty");
        length += sublen;
        prev = c;
    }

    cache.insert((sequence.to_string(), num_robots), length);
    length
}

fn number(code: &str) -> Option<usize> {
    code.strip_suffix("A")?.parse().ok()
}

fn solution(input: &str, num_robots: usize) -> usize {
    let numeric_keypad = keypad_instructions(&NUMERIC_KEYPAD, &NUMERIC_KEYPAD_GAP);
    let directional_keypad = keypad_instructions(&DIRECTIONAL_KEYPAD, &DIRECTIONAL_KEYPAD_GAP);

    let mut cache = HashMap::new();

    input
        .lines()
        .map(|code| {
            let num = number(code).expect("code parse failure");
            let length = instructions_length(
                &mut cache,
                &numeric_keypad,
                &directional_keypad,
                code,
                num_robots,
                true,
            );
            num * length
        })
        .sum()
}

/// # Panics
pub fn part1(input: &str) -> usize {
    // 1 robot on numpad, 2 on keypad
    let num_robots = 3;
    solution(input, num_robots)
}

/// # Panics
pub fn part2(input: &str) -> usize {
    // 1 robot on numpad, 25 on keypads
    let num_robots = 26;
    solution(input, num_robots)
}

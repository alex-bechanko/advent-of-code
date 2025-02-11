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

fn digit(input: &[u8], cursor: usize) -> Option<(usize, u32)> {
    let d = input.get(cursor)?;
    let d = match d {
        b'0' => 0,
        b'1' => 1,
        b'2' => 2,
        b'3' => 3,
        b'4' => 4,
        b'5' => 5,
        b'6' => 6,
        b'7' => 7,
        b'8' => 8,
        b'9' => 9,
        _ => {
            return None;
        }
    };

    Some((cursor + 1, d))
}

fn num(input: &[u8], cursor: usize) -> Option<(usize, u32)> {
    // first digit is required
    let (cursor, num) = digit(input, cursor)?;

    let (cursor, num) = match digit(input, cursor) {
        Some((c, n)) => (c, num * 10 + n),
        None => {
            return Some((cursor, num));
        }
    };

    let (cursor, num) = match digit(input, cursor) {
        Some((c, n)) => (c, num * 10 + n),
        None => {
            return Some((cursor, num));
        }
    };

    Some((cursor, num))
}

fn word(input: &[u8], cursor: usize, word: &[u8]) -> Option<usize> {
    if input.len() - cursor < word.len() {
        None
    } else if &input[cursor..cursor + word.len()] == word {
        Some(cursor + word.len())
    } else {
        None
    }
}

fn instruction_mul(input: &[u8], cursor: usize) -> Option<(usize, u32, u32)> {
    let cursor = word(input, cursor, b"mul(")?;

    let (cursor, num1) = num(input, cursor)?;

    let cursor = word(input, cursor, b",")?;

    let (cursor, num2) = num(input, cursor)?;

    let cursor = word(input, cursor, b")")?;

    Some((cursor, num1, num2))
}

pub fn part1(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut cursor: usize = 0;
    let mut total = 0;

    while cursor < input.len() {
        if let Some((c, num1, num2)) = instruction_mul(input, cursor) {
            total += num1 * num2;
            cursor = c;
        } else {
            cursor += 1;
        }
    }

    total
}

pub fn part2(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut cursor: usize = 0;
    let mut total = 0;
    let mut enabled = true;

    while cursor < input.len() {
        if let (true, Some((c, num1, num2))) = (enabled, instruction_mul(input, cursor)) {
            total += num1 * num2;
            cursor = c;
        } else if let Some(c) = word(input, cursor, b"do()") {
            enabled = true;
            cursor = c;
        } else if let Some(c) = word(input, cursor, b"don't()") {
            enabled = false;
            cursor = c;
        } else {
            cursor += 1;
        }
    }

    total
}

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

pub fn part1(input: &str) -> usize {
    let input = input.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    let grid_width = input[0].len();
    let grid_height = input.len();

    let mut count = 0;
    for row in 0..grid_height {
        for col in 0..grid_width {
            if input[row][col] != b'X' && input[row][col] != b'S' {
                continue;
            }

            count += if col > 2 {
                match (
                    input[row][col - 3],
                    input[row][col - 2],
                    input[row][col - 1],
                    input[row][col],
                ) {
                    (b'X', b'M', b'A', b'S') => 1,
                    (b'S', b'A', b'M', b'X') => 1,
                    _ => 0,
                }
            } else {
                0
            };

            count += if row > 2 {
                match (
                    input[row - 3][col],
                    input[row - 2][col],
                    input[row - 1][col],
                    input[row][col],
                ) {
                    (b'X', b'M', b'A', b'S') => 1,
                    (b'S', b'A', b'M', b'X') => 1,
                    _ => 0,
                }
            } else {
                0
            };

            count += if col > 2 && row > 2 {
                match (
                    input[row - 3][col - 3],
                    input[row - 2][col - 2],
                    input[row - 1][col - 1],
                    input[row][col],
                ) {
                    (b'X', b'M', b'A', b'S') => 1,
                    (b'S', b'A', b'M', b'X') => 1,
                    _ => 0,
                }
            } else {
                0
            };

            count += if col < grid_width - 3 && row > 2 {
                match (
                    input[row - 3][col + 3],
                    input[row - 2][col + 2],
                    input[row - 1][col + 1],
                    input[row][col],
                ) {
                    (b'X', b'M', b'A', b'S') => 1,
                    (b'S', b'A', b'M', b'X') => 1,
                    _ => 0,
                }
            } else {
                0
            };
        }
    }

    count
}

pub fn part2(input: &str) -> usize {
    let input = input.lines().map(|l| l.as_bytes()).collect::<Vec<&[u8]>>();
    let grid_width = input[0].len();
    let grid_height = input.len();

    let mut count = 0;
    for row in 2..grid_height {
        for col in 2..grid_width {
            if input[row][col] != b'M' && input[row][col] != b'S' {
                continue;
            }

            if !matches!(
                (
                    input[row - 2][col - 2],
                    input[row - 1][col - 1],
                    input[row][col],
                ),
                (b'M', b'A', b'S') | (b'S', b'A', b'M')
            ) {
                continue;
            }

            if !matches!(
                (
                    input[row - 2][col],
                    input[row - 1][col - 1],
                    input[row][col - 2],
                ),
                (b'M', b'A', b'S') | (b'S', b'A', b'M')
            ) {
                continue;
            }

            count += 1;
        }
    }

    count
}

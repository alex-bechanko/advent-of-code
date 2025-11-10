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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Space,
    Wall,
}

type Position<T> = (T, T);
fn parse(input: &str) -> (Position<usize>, Position<usize>, Vec<Vec<Cell>>) {
    let mut start = None;
    let mut end = None;

    let mut maze = vec![];
    for (row, line) in input.lines().enumerate() {
        let mut r = vec![];
        for (col, c) in line.chars().enumerate() {
            let cell = match c {
                '#' => Cell::Wall,
                '.' => Cell::Space,
                'S' => {
                    start = Some((col, row));
                    Cell::Space
                }
                'E' => {
                    end = Some((col, row));
                    Cell::Space
                }
                _ => panic!("Unexpected character in input {c:?}"),
            };

            r.push(cell);
        }
        maze.push(r);
    }

    let start = start.expect("No starting point");
    let end = end.expect("No end point");

    (start, end, maze)
}

fn position(position: (usize, usize), offset: (isize, isize)) -> Option<(usize, usize)> {
    let (x, y) = position;
    let (dx, dy) = offset;

    let x = x.checked_add_signed(dx)?;
    let y = y.checked_add_signed(dy)?;

    Some((x, y))
}

fn cell(maze: &[Vec<Cell>], position: (usize, usize)) -> Option<Cell> {
    let (x, y) = position;
    if x >= maze[0].len() || y >= maze.len() {
        return None;
    }

    Some(maze[y][x])
}

fn path(
    maze: &[Vec<Cell>],
    start: (usize, usize),
    end: (usize, usize),
) -> Option<Vec<(usize, usize)>> {
    // find the path, but dont cheat
    // since there is a single path, we just need to find the connection
    let mut path = vec![start];
    while path.last() != Some(&end) && !path.is_empty() {
        let &current = path.last().unwrap();
        let offsets: Vec<(isize, isize)> = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        let mut found = false;
        for offset in offsets {
            if let Some(possible) = position(current, offset) {
                if path.contains(&possible) {
                    continue;
                }

                if let Some(Cell::Space) = cell(maze, possible) {
                    path.push(possible);
                    found = true;
                }
            }
        }

        if !found {
            return None;
        }
    }

    Some(path)
}

fn count_cheats(path: &[(usize, usize)], max_offset: usize, min_savings: usize) -> usize {
    if max_offset < 2 {
        return 0;
    }

    let mut count = 0;
    for i in 0..path.len() {
        for j in i + 1..path.len() {
            let start = path[i];
            let end = path[j];

            let offset = start.0.abs_diff(end.0) + start.1.abs_diff(end.1);
            if offset > max_offset || j - i <= offset || j - i - offset < min_savings {
                continue;
            }

            count += 1;
        }
    }

    count
}

/// # Panics
pub fn part1(input: &str) -> usize {
    let (start, end, maze) = parse(input);
    let positions = path(&maze, start, end).expect("No path from start to finish");
    let max_offset = 2;
    let min_speed_savings = 100;

    count_cheats(&positions, max_offset, min_speed_savings)
}

/// # Panics
pub fn part2(input: &str) -> usize {
    let (start, end, maze) = parse(input);
    let positions = path(&maze, start, end).expect("No path from start to finish");
    let max_offset = 20;
    let min_speed_savings = 100;

    count_cheats(&positions, max_offset, min_speed_savings)
}

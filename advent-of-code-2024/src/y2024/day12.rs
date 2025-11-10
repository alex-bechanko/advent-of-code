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

use std::collections::HashSet;
fn region(garden: &[Vec<char>], row: usize, col: usize) -> HashSet<(usize, usize)> {
    let plant = garden[row][col];
    let mut visited = HashSet::new();
    let mut stack = vec![(row, col)];

    let sides: Vec<(isize, isize)> = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

    while let Some((row, col)) = stack.pop() {
        if !visited.insert((row, col)) {
            continue;
        }

        sides
            .iter()
            .filter_map(|&(dr, dc)| {
                let row = row.checked_add_signed(dr)?;
                let col = col.checked_add_signed(dc)?;
                if row >= garden.len() || col >= garden[0].len() || garden[row][col] != plant {
                    return None;
                }

                Some((row, col))
            })
            .for_each(|cell| stack.push(cell));
    }

    visited
}

fn perimeter(cells: &HashSet<(usize, usize)>) -> u32 {
    let mut perimeter: u32 = 0;

    for &(r, c) in cells {
        let sides = [
            r.checked_sub(1).map(|r| (r, c)),
            c.checked_sub(1).map(|c| (r, c)),
            Some((r + 1, c)),
            Some((r, c + 1)),
        ];

        let sides = sides
            .iter()
            .filter_map(|cell| cell.and_then(|cell| cells.get(&cell)))
            .count();
        let sides: u32 = sides.try_into().unwrap();

        perimeter += 4 - sides;
    }

    perimeter
}

fn area(cells: &HashSet<(usize, usize)>) -> u32 {
    cells.len().try_into().unwrap()
}

fn neighbor(
    cells: &HashSet<(usize, usize)>,
    (row, col): (usize, usize),
    side: Side,
) -> Option<(usize, usize)> {
    let (r, c): (isize, isize) = match side {
        Side::Up => (1, 0),
        Side::Down => (-1, 0),
        Side::Left => (0, -1),
        Side::Right => (0, 1),
    };

    let row = row.checked_add_signed(r)?;
    let col = col.checked_add_signed(c)?;

    cells.get(&(row, col)).copied()
}

fn sides(cells: &HashSet<(usize, usize)>) -> u32 {
    // find a cell that has a top-edge to start
    let mut visited: HashSet<((usize, usize), Side)> = HashSet::new();

    let mut num_sides = 0;

    loop {
        let mut stack: Vec<((usize, usize), Side)> = vec![];

        let start_cell = cells
            .iter()
            .find(|&&cell| {
                neighbor(cells, cell, Side::Up).is_none() && !visited.contains(&(cell, Side::Up))
            })
            .copied();
        if let Some(cell) = start_cell {
            stack.push((cell, Side::Up));
        } else {
            break;
        }

        while let Some((cell, side)) = stack.pop() {
            if !visited.insert((cell, side)) {
                continue;
            }

            if let Some(next) = neighbor(cells, cell, side) {
                // the SIDE edge ended and we have an interior side
                num_sides += 1;
                stack.push((next, rotate(side, 3)));
            } else if let Some(next) = neighbor(cells, cell, rotate(side, 1)) {
                // check if there is a cell that continues the edge
                // if there is, add that cell to the stack and continue
                stack.push((next, side));
            } else {
                // a neighbor doesn't exist, so a side has met its end
                stack.push((cell, rotate(side, 1)));
                num_sides += 1;
            }
        }
    }

    num_sides
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Side {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

fn rotate(side: Side, turns: usize) -> Side {
    let clockwise = [Side::Up, Side::Right, Side::Down, Side::Left];

    clockwise[((side as usize) + turns) % 4]
}

fn solution<F>(input: &str, cost_func: F) -> u32
where
    F: Fn(&HashSet<(usize, usize)>) -> u32,
{
    let input: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let num_rows = input.len();
    let num_cols = input[0].len();
    let mut visited = vec![vec![false; num_cols]; num_rows];

    let mut cost = 0;
    for row in 0..num_rows {
        for col in 0..num_cols {
            if visited[row][col] {
                continue;
            }

            let cells = region(&input, row, col);
            for &(cell_row, cell_col) in &cells {
                visited[cell_row][cell_col] = true;
            }

            cost += cost_func(&cells);
        }
    }

    cost
}

pub fn part1(input: &str) -> u32 {
    solution(input, |cells| {
        let perimeter: u32 = perimeter(cells);
        let area: u32 = area(cells);

        area * perimeter
    })
}

pub fn part2(input: &str) -> u32 {
    solution(input, |cells| {
        let sides: u32 = sides(cells);
        let area: u32 = area(cells);

        area * sides
    })
}

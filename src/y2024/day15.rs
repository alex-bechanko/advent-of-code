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

use std::fmt::Display;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Robot,
    Box(bool),
    Wall,
    Space,
}

#[derive(Clone)]
struct Warehouse {
    expanded: bool,
    robot_row: usize,
    robot_col: usize,
    contents: Vec<Vec<Cell>>,
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.contents {
            for col in row {
                match col {
                    Cell::Box(false) => write!(f, "]")?,
                    Cell::Box(true) => {
                        if self.expanded {
                            write!(f, "[")?;
                        } else {
                            write!(f, "0")?;
                        }
                    }
                    Cell::Robot => write!(f, "\x1b[31m@\x1b[0m")?,
                    Cell::Space => write!(f, ".")?,
                    Cell::Wall => write!(f, "#")?,
                };
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Warehouse {
    fn with_expansion(input: &str, expanded: bool) -> Option<Warehouse> {
        let mut robot = None;
        let mut contents = vec![];
        for line in input.lines() {
            let mut row = vec![];
            for c in line.chars() {
                let cell = match c {
                    '#' => Some(Cell::Wall),
                    'O' => Some(Cell::Box(true)),
                    '@' => Some(Cell::Robot),
                    '.' => Some(Cell::Space),
                    _ => None,
                }?;

                if cell == Cell::Robot {
                    robot = Some((contents.len(), row.len()));
                }

                row.push(cell);
                if expanded {
                    let c = match cell {
                        Cell::Robot => Cell::Space,
                        Cell::Box(true) => Cell::Box(false),
                        c => c,
                    };

                    row.push(c);
                }
            }
            contents.push(row);
        }

        let (robot_row, robot_col) = robot?;
        let warehouse = Warehouse {
            expanded,
            robot_row,
            robot_col,
            contents,
        };

        Some(warehouse)
    }

    fn gps_sum(&self) -> usize {
        let mut total = 0;
        for row in 0..self.contents.len() {
            for col in 0..self.contents[0].len() {
                if self.contents[row][col] == Cell::Box(true) {
                    total += 100 * row + col;
                }
            }
        }

        total
    }

    fn walk(
        &mut self,
        moves: &mut Vec<(usize, usize, Cell)>,
        row: usize,
        col: usize,
        direction: Direction,
    ) -> Option<()> {
        let cell = self.contents[row][col];
        match cell {
            Cell::Space => Some(()),
            Cell::Wall => {
                moves.clear();
                None
            }
            Cell::Robot => {
                let (row2, col2) = next_pair(row, col, direction).unwrap();
                moves.push((row2, col2, Cell::Robot));
                self.walk(moves, row2, col2, direction)
            }
            Cell::Box(false) => self.walk(moves, row, col - 1, direction),
            Cell::Box(true) if self.expanded => match direction {
                Direction::Up | Direction::Down => {
                    let (row2, col2) = next_pair(row, col, direction).unwrap();
                    moves.push((row2, col2, Cell::Box(true)));
                    moves.push((row2, col2 + 1, Cell::Box(false)));
                    self.walk(moves, row2, col2, direction)?;
                    self.walk(moves, row2, col2 + 1, direction)
                }
                Direction::Right => {
                    let (row2, col2) = next_pair(row, col, direction).unwrap();
                    moves.push((row2, col2, Cell::Box(true)));
                    moves.push((row2, col2 + 1, Cell::Box(false)));
                    self.walk(moves, row2, col2 + 1, direction)
                }
                Direction::Left => {
                    let (row2, col2) = next_pair(row, col, direction).unwrap();
                    moves.push((row2, col2, Cell::Box(true)));
                    moves.push((row2, col2 + 1, Cell::Box(false)));
                    self.walk(moves, row2, col2, direction)
                }
            },
            Cell::Box(true) if !self.expanded => {
                let (row2, col2) = next_pair(row, col, direction).unwrap();
                moves.push((row2, col2, Cell::Box(true)));
                self.walk(moves, row2, col2, direction)
            }
            Cell::Box(_) => unreachable!(),
        }
    }

    fn move_robot(&mut self, direction: Direction) -> bool {
        let mut moves = vec![];
        let Some(()) = self.walk(&mut moves, self.robot_row, self.robot_col, direction) else {
            return false;
        };

        for &(row, col, cell) in &moves {
            // for each move, we need to check if the cell needs to be backfilled with a space
            let (prev_row, prev_col) = next_pair(row, col, direction.opposite()).unwrap();
            if moves
                .iter()
                .any(|(r, c, _)| *r == prev_row && *c == prev_col)
            {
                self.contents[prev_row][prev_col] = Cell::Space;
            }

            self.contents[row][col] = cell;
            if cell == Cell::Robot {
                self.robot_row = row;
                self.robot_col = col;
            }
        }

        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(value: char) -> Option<Direction> {
        match value {
            '<' => Some(Direction::Left),
            '>' => Some(Direction::Right),
            'v' => Some(Direction::Down),
            '^' => Some(Direction::Up),
            _ => None,
        }
    }

    fn opposite(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

fn next_pair(row: usize, col: usize, d: Direction) -> Option<(usize, usize)> {
    let (dr, dc) = match d {
        Direction::Up => (-1, 0),
        Direction::Down => (1, 0),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
    };

    // failure to add means that row or col was 0
    // in otherwords, an invalid move and so the warehouse
    // doesnt change
    let row = row.checked_add_signed(dr)?;
    let col = col.checked_add_signed(dc)?;

    Some((row, col))
}

fn parse(input: &str, part_two: bool) -> Option<(Warehouse, Vec<Direction>)> {
    let (warehouse, directions) = input.split_once("\n\n")?;
    let warehouse = Warehouse::with_expansion(warehouse, part_two)?;

    let robot_moves = directions
        .lines()
        .flat_map(|l| l.chars().map(|d| Direction::from_char(d).unwrap()))
        .collect();

    Some((warehouse, robot_moves))
}

fn solve(input: &str, part_two: bool) -> usize {
    let (mut warehouse, directions) = parse(input, part_two).expect("Failed to parse input");
    for &d in &directions {
        warehouse.move_robot(d);
    }
    warehouse.gps_sum()
}

pub fn part1(input: &str) -> usize {
    solve(input, false)
}

pub fn part2(input: &str) -> usize {
    solve(input, true)
}

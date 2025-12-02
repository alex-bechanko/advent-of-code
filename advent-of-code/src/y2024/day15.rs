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

use crate::y2024::direction::CardinalDirection;
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Wall,
    Box,
    BigBoxStart,
    BigBoxEnd,
    Robot,
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Box => 'O',
            Self::Wall => '#',
            Self::Robot => '@',
            Self::BigBoxStart => '[',
            Self::BigBoxEnd => ']',
        };

        write!(f, "{c}")
    }
}

#[derive(Debug)]
struct Warehouse {
    spaces: Vec<Space>,
    robot: usize,
    width: usize,
}

impl Warehouse {
    fn goal(&self, index: usize, direction: CardinalDirection) -> usize {
        use CardinalDirection::{Down, Left, Right, Up};
        match direction {
            Up => index - self.width,
            Down => index + self.width,
            Left => index - 1,
            Right => index + 1,
        }
    }

    fn dependencies(&self, index: usize, direction: CardinalDirection) -> Vec<usize> {
        let goal = self.goal(index, direction);
        let mut frontier = VecDeque::new();
        frontier.push_back(goal);

        let mut dependencies = vec![index];
        while let Some(s) = frontier.pop_front() {
            if dependencies.contains(&s) {
                continue;
            }

            let space = self.spaces[s];
            match space {
                Space::Empty => {}
                Space::Wall | Space::Robot => dependencies.push(s),
                Space::Box => {
                    dependencies.push(s);
                    frontier.push_back(self.goal(s, direction));
                }
                Space::BigBoxStart => {
                    dependencies.push(s);
                    frontier.push_back(s + 1);
                    frontier.push_back(self.goal(s, direction));
                }
                Space::BigBoxEnd => {
                    dependencies.push(s);
                    frontier.push_back(s - 1);
                    frontier.push_back(self.goal(s, direction));
                }
            }
        }

        dependencies
    }

    fn move_robot(&mut self, direction: CardinalDirection) {
        let ds = self.dependencies(self.robot, direction);
        let wall_in_way = ds.iter().map(|&i| self.spaces[i]).any(|s| s == Space::Wall);

        if wall_in_way {
            return;
        }

        let goal = self.goal(self.robot, direction);

        let mut ds = ds;
        while let Some(d) = ds.pop() {
            let goal = self.goal(d, direction);
            self.spaces[goal] = self.spaces[d];
            self.spaces[d] = Space::Empty;
        }

        self.robot = goal;
    }
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.spaces.iter().enumerate() {
            if i != 0 && i % self.width == 0 {
                writeln!(f)?;
            }
            write!(f, "{v}")?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> Option<(Warehouse, Vec<CardinalDirection>)> {
    let (spaces, directions) = input.split_once("\n\n")?;

    let width = spaces.lines().next()?;
    let width = width.chars().count();

    let spaces: Vec<Space> = spaces
        .chars()
        .filter_map(|c| match c {
            '.' => Some(Space::Empty),
            'O' => Some(Space::Box),
            '@' => Some(Space::Robot),
            '#' => Some(Space::Wall),
            _ => None,
        })
        .collect();

    let robot = spaces.iter().position(|&r| r == Space::Robot)?;

    let warehouse = Warehouse {
        width,
        robot,
        spaces,
    };

    let directions: Vec<CardinalDirection> = directions
        .chars()
        .filter_map(|c| match c {
            '<' => Some(CardinalDirection::Left),
            '>' => Some(CardinalDirection::Right),
            'v' => Some(CardinalDirection::Down),
            '^' => Some(CardinalDirection::Up),
            _ => None,
        })
        .collect();

    Some((warehouse, directions))
}

pub fn part1(input: &str) -> usize {
    let (warehouse, directions) = parse(input).expect("input parse failure");

    let warehouse = directions.iter().fold(warehouse, |mut w, &d| {
        w.move_robot(d);
        w
    });

    warehouse
        .spaces
        .iter()
        .enumerate()
        .filter_map(|(i, &s)| match s {
            Space::Box => {
                let row = i / warehouse.width;
                let col = i % warehouse.width;
                Some(row * 100 + col)
            }
            _ => None,
        })
        .sum()
}

fn expand(warehouse: Warehouse) -> Warehouse {
    let spaces = warehouse
        .spaces
        .iter()
        .flat_map(|space| match space {
            Space::Empty => [Space::Empty, Space::Empty],
            Space::Wall => [Space::Wall, Space::Wall],
            Space::Robot => [Space::Robot, Space::Empty],
            Space::Box => [Space::BigBoxStart, Space::BigBoxEnd],
            _ => panic!("Can not expand even further"),
        })
        .collect();

    Warehouse {
        spaces,
        robot: warehouse.robot * 2,
        width: warehouse.width * 2,
    }
}

pub fn part2(input: &str) -> usize {
    let (warehouse, directions) = parse(input).expect("input parse failure");
    let warehouse = expand(warehouse);

    let warehouse = directions.iter().fold(warehouse, |mut w, &d| {
        w.move_robot(d);
        w
    });

    warehouse
        .spaces
        .iter()
        .enumerate()
        .filter_map(|(i, &s)| match s {
            Space::BigBoxStart => {
                let row = i / warehouse.width;
                let col = i % warehouse.width;
                Some(row * 100 + col)
            }
            _ => None,
        })
        .sum()
}

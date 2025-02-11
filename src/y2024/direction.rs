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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CardinalDirection {
    Up,
    Down,
    Left,
    Right,
}

impl CardinalDirection {
    #[must_use]
    pub fn rotate_clockwise(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    #[must_use]
    pub fn rotate_counterclockwise(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    #[must_use]
    pub fn all() -> Vec<Self> {
        vec![Self::Up, Self::Right, Self::Down, Self::Left]
    }
}

impl From<CardinalDirection> for (isize, isize) {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::Up => (0, -1),
            CardinalDirection::Down => (0, 1),
            CardinalDirection::Left => (-1, 0),
            CardinalDirection::Right => (1, 0),
        }
    }
}

impl From<CardinalDirection> for (i32, i32) {
    fn from(value: CardinalDirection) -> Self {
        match value {
            CardinalDirection::Up => (0, -1),
            CardinalDirection::Down => (0, 1),
            CardinalDirection::Left => (-1, 0),
            CardinalDirection::Right => (1, 0),
        }
    }
}

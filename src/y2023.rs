/*
* Advent of Code Solutions Copyright (C) 2023 Alex Bechanko
* <alexbechanko@gmail.com>
*
* This program is free software: you can redistribute it and/or modify it under
* the terms of the GNU General Public License as published by the Free Software
* Foundation, either version 3 of the License, or (at your option) any later
* version.
*
* This program is distributed in the hope that it will be useful, but WITHOUT
* ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS
* FOR A PARTICULAR PURPOSE.  See the GNU General Public License for more
* details.
*
* You should have received a copy of the GNU General Public License along with
* this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod day01;
mod day02;
use crate::Solution;

pub const DAYS: [(Solution, Solution); 2] = [
    (&day01::part_a, &day01::part_b),
    (&day02::part_a, &day02::part_b),
];

pub const DAY_INPUTS: [&str; 2] = [
    include_str!("../inputs/2023-12-01.txt"),
    include_str!("../inputs/2023-12-02.txt"),
];

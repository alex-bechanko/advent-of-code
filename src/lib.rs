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

mod y2022 {
    pub mod day01;
    use crate::Solution;

    pub const DAYS: [(Solution, Solution); 1] = [(&day01::part_a, &day01::part_b)];

    pub const DAY_INPUTS: [&str; 1] = [include_str!("../inputs/2022-12-01.txt")];
}

mod y2023 {
    mod day01;
    mod day02;
    mod day03;
    mod day04;
    use crate::Solution;

    pub const DAYS: [(Solution, Solution); 4] = [
        (&day01::part_a, &day01::part_b),
        (&day02::part_a, &day02::part_b),
        (&day03::part_a, &day03::part_b),
        (&day04::part_a, &day04::part_b),
    ];

    pub const DAY_INPUTS: [&str; 4] = [
        include_str!("../inputs/2023-12-01.txt"),
        include_str!("../inputs/2023-12-02.txt"),
        include_str!("../inputs/2023-12-03.txt"),
        include_str!("../inputs/2023-12-04.txt"),
    ];
}

pub type Solution<'a> = &'a dyn Fn(&str) -> Result<String, String>;
pub const YEARS: [&[(Solution, Solution)]; 2] = [&y2022::DAYS, &y2023::DAYS];
pub const YEAR_INPUTS: [&[&str]; 2] = [&y2022::DAY_INPUTS, &y2023::DAY_INPUTS];

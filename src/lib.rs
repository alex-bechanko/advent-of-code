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

pub mod y2024 {
    pub mod day01;
    pub mod day02;
    pub mod day03;
    pub mod day04;
    pub mod day05;
    pub mod day06;
    pub mod day07;
    pub mod day08;
    pub mod day09;
    pub mod day10;
    pub mod day11;
    pub mod day12;
    pub mod day13;
    pub mod day14;
    pub mod day15;
}

use std::env;

const USAGE: &str = "\
advent-of-code
Compute solutions to Advent of Code problems

Usage: advent-of-code [OPTION] PUZZLE [INPUT]

Arguments:
    PUZZLE  The puzzle to run. Valid values are dates in the format YYYY-12-DD.
            The year YYYY is in 2015-2024.
            The day DD is 01-25. The zero prefix is required.
    INPUT   Path to the puzzle input.
            If stdin is preferred, then -- can be passed to indicate that.
            If no input is provided, then the path ./inputs/PUZZLE.txt where PUZZLE is the
            provided puzzle argument.

Options:
    -h, --help      Print this usage message and exit
    -v, --version   Print version information and exit
";

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn main() {
    if env::args().any(|x| x == "-v" || x == "--version") {
        println!("Version: {}", VERSION);
        std::process::exit(0);
    }

    if env::args().any(|x| x == "-h" || x == "--help") {
        println!("{}", USAGE);
        std::process::exit(0);
    }

    let mut args = env::args().skip(1);

    let puzzle = match args.next() {
        Some(p) => p,
        None => {
            println!("{USAGE}");
            println!("Missing 'PUZZLE' argument");
            std::process::exit(1);
        }
    };

    let file = match args.next() {
        Some(f) => f,
        None => {
            let f = format!("inputs/{puzzle}.txt");
            println!("No input provided, using default {f}.");
            f
        }
    };

    let contents = match file.as_str() {
        "--" => match std::io::read_to_string(std::io::stdin()) {
            Ok(s) => s,
            Err(_) => {
                println!("Error occurred reading from stdin");
                std::process::exit(1);
            }
        },
        path => match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(_) => {
                println!("Error occurred reading from file: {}", path);
                std::process::exit(1);
            }
        },
    };

    run(&puzzle, &contents);
}

fn run_puzzle<F, G, A, B>(input: &str, part1: F, part2: G)
where
    F: Fn(&str) -> A,
    G: Fn(&str) -> B,
    A: ToString,
    B: ToString,
{
    let soln1 = part1(input).to_string();
    println!("Part 1 Solution: {soln1}");

    let soln2 = part2(input).to_string();
    println!("Part 2 Solution: {soln2}");
}

fn run(puzzle: &str, input: &str) {
    use crate::*;
    match puzzle {
        "2024-12-01" => run_puzzle(input, y2024::day01::part1, y2024::day01::part2),
        "2024-12-02" => run_puzzle(input, y2024::day02::part1, y2024::day02::part2),
        "2024-12-03" => run_puzzle(input, y2024::day03::part1, y2024::day03::part2),
        "2024-12-04" => run_puzzle(input, y2024::day04::part1, y2024::day04::part2),
        "2024-12-05" => run_puzzle(input, y2024::day05::part1, y2024::day05::part2),
        "2024-12-06" => run_puzzle(input, y2024::day06::part1, y2024::day06::part2),
        "2024-12-07" => run_puzzle(input, y2024::day07::part1, y2024::day07::part2),
        "2024-12-08" => run_puzzle(input, y2024::day08::part1, y2024::day08::part2),
        "2024-12-09" => run_puzzle(input, y2024::day09::part1, y2024::day09::part2),
        "2024-12-10" => run_puzzle(input, y2024::day10::part1, y2024::day10::part2),
        "2024-12-11" => run_puzzle(input, y2024::day11::part1, y2024::day11::part2),
        "2024-12-12" => run_puzzle(input, y2024::day12::part1, y2024::day12::part2),
        "2024-12-13" => run_puzzle(input, y2024::day13::part1, y2024::day13::part2),
        "2024-12-14" => run_puzzle(input, y2024::day14::part1, y2024::day14::part2),
        "2024-12-15" => run_puzzle(input, y2024::day15::part1, y2024::day15::part2),
        _ => {
            println!("Unrecognized PUZZLE '{puzzle}'");
            std::process::exit(1);
        }
    }
}

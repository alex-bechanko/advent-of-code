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

fn run(puzzle: &str, _input: &str) {
    println!("Unrecognized PUZZLE '{puzzle}'");
    std::process::exit(1);
}

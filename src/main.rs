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


use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "advent-of-code", author = "Alex Bechanko")]
#[command(about = "Compute Advent of Code puzzle solutions")]
pub struct Args {
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u32).range(2022..2024))]
    year: u32,

    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u32).range(1..26))]
    puzzle: u32,

    #[arg(short, long)]
    file: PathBuf,
}

fn main() {
    let args = Args::parse();
    let year_index = match usize::try_from(args.year).map(|y| y - 2022) {
        Ok(y) => y,
        Err(_) => {
            println!("Failed to parse year: {}", args.year);
            std::process::exit(1);
        }
    };

    let &year = match aoc::YEARS.get(year_index) {
        Some(y) => y,
        None => {
            println!("Year {} is not implemented yet", args.year);
            std::process::exit(1);
        }
    };

    let puzzle_index = match usize::try_from(args.puzzle).map(|d| d - 1) {
        Ok(p) => p,
        Err(_) => {
            println!("Failed to parse puzzle day: {}", args.puzzle);
            std::process::exit(1);
        }
    };

    let &puzzle = match year.get(puzzle_index) {
        Some(p) => p,
        None => {
            println!("Puzzle on day {} is not implemented yet", args.puzzle);
            std::process::exit(1);
        }
    };

    let input = match std::fs::read_to_string(args.file.as_path()) {
        Ok(inp) => inp,
        Err(why) => {
            println!(
                "Failed to read file {}: {}",
                args.file.to_string_lossy(),
                why
            );
            std::process::exit(1);
        }
    };

    let total_timer = took::Timer::new();

    let part_a_timer = took::Timer::new();
    let part_a_answer = puzzle.0(input.as_str());
    let part_a_time = part_a_timer.took();

    let part_b_timer = took::Timer::new();
    let part_b_answer = puzzle.1(input.as_str());
    let part_b_time = part_b_timer.took();

    let total_time = total_timer.took();

    println!("Puzzle {} for Advent of Code {} ({})", args.puzzle, args.year, total_time);
    match part_a_answer {
        Ok(ans) => println!("\tPart A Solution ({}): {}", part_a_time, ans),
        Err(why) => println!("\tPart A Solution Error: {}", why),
    }

    match part_b_answer {
        Ok(ans) => println!("\tPart B Solution ({}): {}", part_b_time, ans),
        Err(why) => println!("\tPart B Solution Error: {}", why),
    }
}

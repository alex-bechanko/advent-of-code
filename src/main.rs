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

#[derive(Debug, Parser)]
#[command(name = "advent-of-code", author = "Alex Bechanko")]
#[command(about = "Compute Advent of Code puzzle solutions")]
struct Args {
    #[arg(short, long)]
    #[arg(value_parser = clap::value_parser!(u32).range(2022..2024))]
    year: u32,

    #[arg(short = 'p', long = "puzzle")]
    #[arg(value_parser = clap::value_parser!(u32).range(1..26))]
    puzzle: Option<u32>,

    #[arg(short, long, action=clap::ArgAction::SetTrue, conflicts_with = "puzzle")]
    all: bool,
}

struct PrettyDuration {
    elapsed: std::time::Duration,
}

impl std::fmt::Display for PrettyDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = self.elapsed;
        let secs = u128::from(e.as_secs());
        let millis = e.as_millis();
        let micros = e.as_micros();

        let (major, minor, unit) = if secs > 0 {
            (secs, millis, "s")
        } else {
            (millis, micros, "ms")
        };

        let time = major as f64 + (minor - major * 1000) as f64 / 1000.0;
        f.pad(&format!("{:2.3} {}", time, unit))
    }
}

fn pretty_duration(duration: std::time::Duration) -> PrettyDuration {
    PrettyDuration { elapsed: duration }
}


fn run_all_puzzles(year: usize) -> Result<(), Box<dyn std::error::Error>> {

    let &year_puzzles = aoc::YEARS.get(year)
        .ok_or(format!("Year {} is not implemented", 2022 + year))?;

    let mut answers: Vec<(std::time::Duration, std::time::Duration)> = vec![];
    answers.reserve(year_puzzles.len());

    let mut total_duration = std::time::Duration::new(0,0);

    for i in 0..year_puzzles.len() {
        let (part_a, part_b, input) = year_puzzles[i];

        let start = std::time::Instant::now();
        let a = part_a(input);
        let duration_a = start.elapsed();
        total_duration += duration_a;


        let start = std::time::Instant::now();
        let b = part_b(input);
        let duration_b = start.elapsed();
        total_duration += duration_b;

        answers.push((duration_a, duration_b));


        let a = a.unwrap_or("ERROR".to_string());
        println!("{}.a\t{:>10}\t{}", i + 1, pretty_duration(duration_a), a);

        let b = b.unwrap_or("ERROR".to_string());
        println!("{}.b\t{:>10}\t{}", i + 1, pretty_duration(duration_b), b);
    }

    println!("Total\t{}", pretty_duration(total_duration));

    Ok(())
}

fn run_puzzle(year: usize, puzzle: usize) -> Result<(), Box<dyn std::error::Error>> {
    let year_puzzles = aoc::YEARS
        .get(year)
        .ok_or(format!("Year {} is not implemented", 2022 + year))?;
    let (part_a, part_b, input) = year_puzzles
        .get(puzzle)
        .ok_or(format!("Puzzle Day {} is not implemented", puzzle + 1))?;

    let start = std::time::Instant::now();
    let a = part_a(input);
    let duration_a = start.elapsed();

    let start = std::time::Instant::now();
    let b = part_b(input);
    let duration_b = start.elapsed();

    let total_duration = duration_a + duration_b;

    println!(
        "Puzzle {} for Advent of Code {} ({})",
        puzzle + 1,
        year + 2022,
        pretty_duration(total_duration)
    );
    match a {
        Ok(ans) => println!("\tPart A Solution ({}): {}", pretty_duration(duration_a), ans),
        Err(why) => println!("\tPart A Solution Error: {}", why),
    }

    match b {
        Ok(ans) => println!("\tPart B Solution ({}): {}", pretty_duration(duration_b), ans),
        Err(why) => println!("\tPart B Solution Error: {}", why),
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let year = usize::try_from(args.year)
        .map(|y| y - 2022)
        .map_err(|_| format!("Invalid year parameter: {}", args.year))?;

    if args.all {
        run_all_puzzles(year)
    } else {
        let puzzle: usize = usize::try_from(args.puzzle.unwrap() - 1)
            .map_err(|_| format!("Invalid day parameter: {}", args.puzzle.unwrap()))?;

        run_puzzle(year, puzzle)
    }
}

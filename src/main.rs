use std::path::PathBuf;
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name="advent-of-code", author="Alex Bechanko")]
#[command(about="Compute Advent of Code puzzle solutions")]
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
            println!("Failed to read file {}: {}", args.file.to_string_lossy(), why);
            std::process::exit(1);
        }
    };

    println!("Puzzle {} for Advent of Code {}", args.puzzle, args.year);
    match puzzle.0(input.as_str()) {
        Ok(ans) => println!("Part A Solution: {}", ans),
        Err(why) => println!("Part A Solution Error: {}", why),
    }

    match puzzle.1(input.as_str()) {
        Ok(ans) => println!("Part B Solution: {}", ans),
        Err(why) => println!("Part B Solution Error: {}", why),
    }


}

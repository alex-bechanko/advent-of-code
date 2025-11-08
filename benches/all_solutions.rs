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

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn read_input_file(file: &str) -> String {
    let err = format!("Expected input file {file}");
    std::fs::read_to_string(file).expect(&err)
}

#[allow(clippy::too_many_lines)]
pub fn aoc2024(c: &mut Criterion) {
    c.bench_function("all_solutions", |b| {
        b.iter(|| {
            aoc::y2024::day01::part1(black_box(&read_input_file("./inputs/2024-12-01.txt")));
            aoc::y2024::day01::part2(black_box(&read_input_file("./inputs/2024-12-01.txt")));
            aoc::y2024::day02::part1(black_box(&read_input_file("./inputs/2024-12-02.txt")));
            aoc::y2024::day02::part2(black_box(&read_input_file("./inputs/2024-12-02.txt")));
            aoc::y2024::day03::part1(black_box(&read_input_file("./inputs/2024-12-03.txt")));
            aoc::y2024::day03::part2(black_box(&read_input_file("./inputs/2024-12-03.txt")));
            aoc::y2024::day04::part1(black_box(&read_input_file("./inputs/2024-12-04.txt")));
            aoc::y2024::day04::part2(black_box(&read_input_file("./inputs/2024-12-04.txt")));
            aoc::y2024::day05::part1(black_box(&read_input_file("./inputs/2024-12-05.txt")));
            aoc::y2024::day05::part2(black_box(&read_input_file("./inputs/2024-12-05.txt")));
            aoc::y2024::day06::part1(black_box(&read_input_file("./inputs/2024-12-06.txt")));
            aoc::y2024::day06::part2(black_box(&read_input_file("./inputs/2024-12-06.txt")));
            aoc::y2024::day07::part1(black_box(&read_input_file("./inputs/2024-12-07.txt")));
            aoc::y2024::day07::part2(black_box(&read_input_file("./inputs/2024-12-07.txt")));
            aoc::y2024::day08::part1(black_box(&read_input_file("./inputs/2024-12-08.txt")));
            aoc::y2024::day08::part2(black_box(&read_input_file("./inputs/2024-12-08.txt")));
            aoc::y2024::day09::part1(black_box(&read_input_file("./inputs/2024-12-09.txt")));
            aoc::y2024::day09::part2(black_box(&read_input_file("./inputs/2024-12-09.txt")));
            aoc::y2024::day10::part1(black_box(&read_input_file("./inputs/2024-12-10.txt")));
            aoc::y2024::day10::part2(black_box(&read_input_file("./inputs/2024-12-10.txt")));
            aoc::y2024::day11::part1(black_box(&read_input_file("./inputs/2024-12-11.txt")));
            aoc::y2024::day11::part2(black_box(&read_input_file("./inputs/2024-12-11.txt")));
            aoc::y2024::day12::part1(black_box(&read_input_file("./inputs/2024-12-12.txt")));
            aoc::y2024::day12::part2(black_box(&read_input_file("./inputs/2024-12-12.txt")));
            aoc::y2024::day13::part1(black_box(&read_input_file("./inputs/2024-12-13.txt")));
            aoc::y2024::day13::part2(black_box(&read_input_file("./inputs/2024-12-13.txt")));
            aoc::y2024::day14::part1(black_box(&read_input_file("./inputs/2024-12-14.txt")));
            aoc::y2024::day14::part2(black_box(&read_input_file("./inputs/2024-12-14.txt")));
            aoc::y2024::day15::part1(black_box(&read_input_file("./inputs/2024-12-15.txt")));
            aoc::y2024::day15::part2(black_box(&read_input_file("./inputs/2024-12-15.txt")));
            aoc::y2024::day16::part1(black_box(&read_input_file("./inputs/2024-12-16.txt")));
            aoc::y2024::day16::part2(black_box(&read_input_file("./inputs/2024-12-16.txt")));
            aoc::y2024::day17::part1(black_box(&read_input_file("./inputs/2024-12-17.txt")));
            aoc::y2024::day17::part2(black_box(&read_input_file("./inputs/2024-12-17.txt")));
            aoc::y2024::day18::part1(black_box(&read_input_file("./inputs/2024-12-18.txt")));
            aoc::y2024::day18::part2(black_box(&read_input_file("./inputs/2024-12-18.txt")));
            aoc::y2024::day19::part1(black_box(&read_input_file("./inputs/2024-12-19.txt")));
            aoc::y2024::day19::part2(black_box(&read_input_file("./inputs/2024-12-19.txt")));
            aoc::y2024::day20::part1(black_box(&read_input_file("./inputs/2024-12-20.txt")));
            aoc::y2024::day20::part2(black_box(&read_input_file("./inputs/2024-12-20.txt")));
            aoc::y2024::day21::part1(black_box(&read_input_file("./inputs/2024-12-21.txt")));
            aoc::y2024::day21::part2(black_box(&read_input_file("./inputs/2024-12-21.txt")));
            aoc::y2024::day22::part1(black_box(&read_input_file("./inputs/2024-12-22.txt")));
            aoc::y2024::day22::part2(black_box(&read_input_file("./inputs/2024-12-22.txt")));
            aoc::y2024::day23::part1(black_box(&read_input_file("./inputs/2024-12-23.txt")));
            aoc::y2024::day23::part2(black_box(&read_input_file("./inputs/2024-12-23.txt")));
            aoc::y2024::day24::part1(black_box(&read_input_file("./inputs/2024-12-24.txt")));
            aoc::y2024::day24::part2(black_box(&read_input_file("./inputs/2024-12-24.txt")));
            aoc::y2024::day25::part1(black_box(&read_input_file("./inputs/2024-12-25.txt")));
            // no part 2024-12-25 part 2, so don't bench the "No Solution"
        });
    });
}

criterion_group!(solutions, aoc2024);

criterion_main!(solutions);

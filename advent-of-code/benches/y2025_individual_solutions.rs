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

use criterion::{Criterion, black_box, criterion_group, criterion_main};

fn read_input_file(file: &str) -> String {
    let err = format!("Expected input file {file}");
    std::fs::read_to_string(file).expect(&err)
}

pub fn aoc2025(c: &mut Criterion) {
    c.bench_function("2025-12-01 Part 1", |b| {
        b.iter(|| aoc::y2025::day01::part1(black_box(&read_input_file("./inputs/2025-12-01.txt"))))
    });
    c.bench_function("2025-12-01 Part 2", |b| {
        b.iter(|| aoc::y2025::day01::part2(black_box(&read_input_file("./inputs/2025-12-01.txt"))))
    });
    c.bench_function("2025-12-02 Part 1", |b| {
        b.iter(|| aoc::y2025::day02::part1(black_box(&read_input_file("./inputs/2025-12-02.txt"))))
    });
    c.bench_function("2025-12-02 Part 2", |b| {
        b.iter(|| aoc::y2025::day02::part2(black_box(&read_input_file("./inputs/2025-12-02.txt"))))
    });
}

criterion_group!(solutions, aoc2025);

criterion_main!(solutions);

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

pub fn aoc2024(c: &mut Criterion) {
    use aoc::y2024::*;
    c.bench_function("2024-12-01 Part 1", |b| {
        b.iter(|| day01::part1(black_box(&read_input_file("./inputs/2024-12-01.txt"))));
    });
    c.bench_function("2024-12-01 Part 2", |b| {
        b.iter(|| day01::part2(black_box(&read_input_file("./inputs/2024-12-01.txt"))));
    });

    c.bench_function("2024-12-02 Part 1", |b| {
        b.iter(|| day02::part1(black_box(&read_input_file("./inputs/2024-12-02.txt"))));
    });
    c.bench_function("2024-12-02 Part 2", |b| {
        b.iter(|| day02::part2(black_box(&read_input_file("./inputs/2024-12-02.txt"))));
    });

    c.bench_function("2024-12-03 Part 1", |b| {
        b.iter(|| day03::part1(black_box(&read_input_file("./inputs/2024-12-03.txt"))));
    });
    c.bench_function("2024-12-03 Part 2", |b| {
        b.iter(|| day03::part2(black_box(&read_input_file("./inputs/2024-12-03.txt"))));
    });

    c.bench_function("2024-12-04 Part 1", |b| {
        b.iter(|| day04::part1(black_box(&read_input_file("./inputs/2024-12-04.txt"))));
    });
    c.bench_function("2024-12-04 Part 2", |b| {
        b.iter(|| day04::part2(black_box(&read_input_file("./inputs/2024-12-04.txt"))));
    });

    c.bench_function("2024-12-05 Part 1", |b| {
        b.iter(|| day05::part1(black_box(&read_input_file("./inputs/2024-12-05.txt"))));
    });
    c.bench_function("2024-12-05 Part 2", |b| {
        b.iter(|| day05::part2(black_box(&read_input_file("./inputs/2024-12-05.txt"))));
    });

    c.bench_function("2024-12-06 Part 1", |b| {
        b.iter(|| day06::part1(black_box(&read_input_file("./inputs/2024-12-06.txt"))));
    });
    c.bench_function("2024-12-06 Part 2", |b| {
        b.iter(|| day06::part2(black_box(&read_input_file("./inputs/2024-12-06.txt"))));
    });

    c.bench_function("2024-12-07 Part 1", |b| {
        b.iter(|| day07::part1(black_box(&read_input_file("./inputs/2024-12-07.txt"))));
    });
    c.bench_function("2024-12-07 Part 2", |b| {
        b.iter(|| day07::part2(black_box(&read_input_file("./inputs/2024-12-07.txt"))));
    });

    c.bench_function("2024-12-08 Part 1", |b| {
        b.iter(|| day08::part1(black_box(&read_input_file("./inputs/2024-12-08.txt"))));
    });
    c.bench_function("2024-12-08 Part 2", |b| {
        b.iter(|| day08::part2(black_box(&read_input_file("./inputs/2024-12-08.txt"))));
    });

    c.bench_function("2024-12-09 Part 1", |b| {
        b.iter(|| day09::part1(black_box(&read_input_file("./inputs/2024-12-09.txt"))));
    });
    c.bench_function("2024-12-09 Part 2", |b| {
        b.iter(|| day09::part2(black_box(&read_input_file("./inputs/2024-12-09.txt"))));
    });

    c.bench_function("2024-12-10 Part 1", |b| {
        b.iter(|| day10::part1(black_box(&read_input_file("./inputs/2024-12-10.txt"))));
    });
    c.bench_function("2024-12-10 Part 2", |b| {
        b.iter(|| day10::part2(black_box(&read_input_file("./inputs/2024-12-10.txt"))));
    });

    c.bench_function("2024-12-11 Part 1", |b| {
        b.iter(|| day11::part1(black_box(&read_input_file("./inputs/2024-12-11.txt"))));
    });
    c.bench_function("2024-12-11 Part 2", |b| {
        b.iter(|| day11::part2(black_box(&read_input_file("./inputs/2024-12-11.txt"))));
    });

    c.bench_function("2024-12-12 Part 1", |b| {
        b.iter(|| day12::part1(black_box(&read_input_file("./inputs/2024-12-12.txt"))));
    });
    c.bench_function("2024-12-12 Part 2", |b| {
        b.iter(|| day12::part2(black_box(&read_input_file("./inputs/2024-12-12.txt"))));
    });

    c.bench_function("2024-12-13 Part 1", |b| {
        b.iter(|| day13::part1(black_box(&read_input_file("./inputs/2024-12-13.txt"))));
    });
    c.bench_function("2024-12-13 Part 2", |b| {
        b.iter(|| day13::part2(black_box(&read_input_file("./inputs/2024-12-13.txt"))));
    });

    c.bench_function("2024-12-14 Part 1", |b| {
        b.iter(|| day14::part1(black_box(&read_input_file("./inputs/2024-12-14.txt"))));
    });
    c.bench_function("2024-12-14 Part 2", |b| {
        b.iter(|| day14::part2(black_box(&read_input_file("./inputs/2024-12-14.txt"))));
    });
}

criterion_group!(solutions, aoc2024);

criterion_main!(solutions);

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

fn parse(input: &str) -> Vec<(i64, Vec<i64>)> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let (ans, numbers) = l
                .split_once(":")
                .unwrap_or_else(|| panic!("Split on ':' failure on line {i}"));
            let ans = ans
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("Parse i64 failure on line {i}"));

            let numbers: Vec<i64> = numbers
                .split_whitespace()
                .map(|x| {
                    x.parse::<i64>()
                        .unwrap_or_else(|_| panic!("Parse i64 failure on line {i}"))
                })
                .collect();

            (ans, numbers)
        })
        .collect()
}

fn is_part1_calibration(numbers: &[i64], answer: i64, n: usize) -> bool {
    if n == 0 {
        return answer == numbers[0];
    }

    if answer % numbers[n] == 0 && is_part1_calibration(numbers, answer / numbers[n], n - 1) {
        return true;
    }

    (answer - numbers[n] > 0) && is_part1_calibration(numbers, answer - numbers[n], n - 1)
}

pub fn part1(input: &str) -> i64 {
    let equations = parse(input);

    equations
        .iter()
        .filter(|(ans, numbers)| is_part1_calibration(numbers, *ans, numbers.len() - 1))
        .map(|(ans, _)| ans)
        .sum()
}

fn is_part2_calibration(numbers: &[i64], answer: i64, n: usize) -> bool {
    if n == 0 {
        return answer == numbers[n];
    }

    let num = numbers[n];

    if answer % num == 0 && is_part2_calibration(numbers, answer / num, n - 1) {
        return true;
    }

    if (answer - num > 0) && is_part2_calibration(numbers, answer - num, n - 1) {
        return true;
    }

    let digits = num.ilog10() + 1;
    let ans_digits = answer.ilog10() + 1;

    ans_digits > digits
        && answer % 10_i64.pow(digits) == num
        && is_part2_calibration(numbers, answer / 10_i64.pow(digits), n - 1)
}

pub fn part2(input: &str) -> i64 {
    let equations = parse(input);

    equations
        .iter()
        .filter(|(ans, numbers)| is_part2_calibration(numbers, *ans, numbers.len() - 1))
        .map(|(ans, _)| ans)
        .sum()
}

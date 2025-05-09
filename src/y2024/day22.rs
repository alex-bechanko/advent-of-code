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

#[derive(Debug)]
struct SecretIterator(isize);

impl Iterator for SecretIterator {
    type Item = isize;
    fn next(&mut self) -> Option<Self::Item> {
        let secret = self.0;
        let prune_number = 16_777_216;
        let step1 = ((secret * 64) ^ secret) % prune_number;
        let step2 = ((step1 / 32) ^ step1) % prune_number;
        let step3 = ((step2 * 2048) ^ step2) % prune_number;
        self.0 = step3;

        Some(secret)
    }
}

fn bananas(secret: isize) -> isize {
    secret % 10
}

fn differences(arr: &[isize]) -> Vec<isize> {
    arr.windows(2).map(|win| win[1] - win[0]).collect()
}

fn sequence_matrix(monkeys: &[Vec<isize>]) -> Vec<isize> {
    let monkey_diffs: Vec<Vec<isize>> = monkeys.iter().map(|monkey| differences(monkey)).collect();
    let mut monkey_seqs: Vec<isize> = vec![0; 19usize.pow(4)];
    let mut monkey_seq_exists: Vec<Vec<bool>> = vec![vec![false; 19usize.pow(4)]; monkeys.len()];

    for (monkey, diffs) in monkey_diffs.iter().enumerate() {
        for (seq_start_index, seq) in diffs.windows(4).enumerate() {
            let encoded_sequence: isize = (seq[0] + 9)
                + (seq[1] + 9) * 19isize
                + (seq[2] + 9) * 19isize.pow(2)
                + (seq[3] + 9) * 19isize.pow(3);
            let encoded_sequence: usize = encoded_sequence.try_into().unwrap();
            let bananas = monkeys[monkey][seq_start_index + 4];

            if !monkey_seq_exists[monkey][encoded_sequence] {
                monkey_seqs[encoded_sequence] += bananas;
                monkey_seq_exists[monkey][encoded_sequence] = true;
            }
        }
    }

    monkey_seqs
}

fn max_sequence(secrets: &[isize], lookahead: usize) -> Option<isize> {
    let monkeys: Vec<Vec<isize>> = secrets
        .iter()
        .map(|&secret| {
            SecretIterator(secret)
                .take(lookahead)
                .map(bananas)
                .collect()
        })
        .collect();

    let sequences = sequence_matrix(&monkeys);

    sequences.into_iter().max()
}

pub fn part1(input: &str) -> isize {
    input
        .lines()
        .filter_map(|line| {
            line.parse::<isize>()
                .ok()
                .and_then(|secret| SecretIterator(secret).nth(2000))
        })
        .sum()
}

/// # Panics
pub fn part2(input: &str) -> isize {
    let num_prices = 2000;
    let secrets: Vec<isize> = input
        .lines()
        .filter_map(|line| line.parse::<isize>().ok())
        .collect();

    let ans = max_sequence(&secrets, num_prices);

    ans.unwrap()
}

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
struct SecretIterator(usize);

impl Iterator for SecretIterator {
    type Item = usize;
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

fn bananas(secret: usize) -> usize {
    secret % 10
}

fn sequence_totals(monkeys: &[usize], lookahead: usize) -> Vec<usize> {
    let mut sequence_bananas = vec![0usize; 19usize.pow(4)];
    let mut last_monkey = vec![0usize; 19usize.pow(4)];

    for (i, &monkey) in monkeys.iter().enumerate() {
        let mut monkey = SecretIterator(monkey).map(bananas);

        // calculate the first hash
        let (mut hash, mut prev) = {
            let c1 = monkey.next().unwrap();
            let c2 = monkey.next().unwrap();
            let c3 = monkey.next().unwrap();
            let c4 = monkey.next().unwrap();
            let c5 = monkey.next().unwrap();

            // adding 9 first removes need to case to isize first
            let d1 = 9 + c2 - c1;
            let d2 = 9 + c3 - c2;
            let d3 = 9 + c4 - c3;
            let d4 = 9 + c5 - c4;

            let hash = d1 + (d2 * 19usize) + (d3 * 19usize.pow(2)) + (d4 * 19usize.pow(3));

            (hash, c5)
        };

        sequence_bananas[hash] += prev;
        last_monkey[hash] = i;

        for curr in monkey.take(lookahead - 5) {
            let diff = 9 + curr - prev;
            hash -= hash % 19;
            hash /= 19;
            hash += diff * 19usize.pow(3);

            if last_monkey[hash] != i {
                sequence_bananas[hash] += curr;
                last_monkey[hash] = i;
            }

            prev = curr;
        }
    }

    sequence_bananas
}

pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|line| {
            line.parse::<usize>()
                .ok()
                .and_then(|secret| SecretIterator(secret).nth(2000))
        })
        .sum()
}

/// # Panics
pub fn part2(input: &str) -> usize {
    let num_prices = 2000;
    let secrets: Vec<usize> = input.lines().filter_map(|line| line.parse().ok()).collect();

    sequence_totals(&secrets, num_prices)
        .iter()
        .max()
        .copied()
        .unwrap()
}

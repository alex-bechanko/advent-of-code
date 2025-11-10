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

fn parse_line<T>(input: &str, prefix: &str, operator: &str) -> Option<(T, T)>
where
    T: std::str::FromStr,
{
    let input = input.strip_prefix(prefix)?;
    let (x, y) = input.split_once(", ")?;

    let x = x.strip_prefix("X")?;
    let x = x.strip_prefix(operator)?;
    let x = x.parse().ok()?;

    let y = y.strip_prefix("Y")?;
    let y = y.strip_prefix(operator)?;
    let y = y.parse().ok()?;

    Some((x, y))
}

type Pair<T> = (T, T);
type Puzzle<T> = (Pair<T>, Pair<T>, Pair<T>);
fn parse<T>(input: &str) -> Vec<Puzzle<T>>
where
    T: std::str::FromStr,
{
    input
        .split("\n\n")
        .map(|chunk| {
            let mut chunk = chunk.lines();
            let button_a = chunk
                .next()
                .and_then(|l| parse_line(l, "Button A: ", "+"))
                .expect("Missing button A");

            let button_b = chunk
                .next()
                .and_then(|l| parse_line(l, "Button B: ", "+"))
                .expect("Missing button B");

            let prize = chunk
                .next()
                .and_then(|l| parse_line(l, "Prize: ", "="))
                .expect("Missing prize");

            (button_a, button_b, prize)
        })
        .collect()
}

fn machine_tokens((button_a, button_b, prize): Puzzle<i128>) -> Option<i128> {
    let d = button_a.0 * button_b.1 - button_b.0 * button_a.1;
    if d == 0 {
        return None;
    }

    let x = prize.0 * button_b.1 - button_b.0 * prize.1;
    let y = button_a.0 * prize.1 - prize.0 * button_a.1;

    if x % d != 0 || y % d != 0 {
        return None;
    }

    let ans = (3 * x + y) / d;

    Some(ans)
}

pub fn part1(input: &str) -> i128 {
    parse(input).into_iter().filter_map(machine_tokens).sum()
}

pub fn part2(input: &str) -> i128 {
    parse(input)
        .into_iter()
        .map(|(a, b, p)| {
            let p0 = p.0 + 10_000_000_000_000_i128;
            let p1 = p.1 + 10_000_000_000_000_i128;
            (a, b, (p0, p1))
        })
        .filter_map(machine_tokens)
        .sum()
}

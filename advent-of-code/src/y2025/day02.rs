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

fn is_repetition(id: usize, size: u32) -> bool {
    let chunk_size = 10usize.pow(size);
    let chunk1 = id % chunk_size;
    let chunk2 = (id / chunk_size) % chunk_size;

    if id == 0 {
        true
    } else if chunk1 != chunk2 {
        false
    } else {
        id.ilog10() + 1 == 2 * size || is_repetition(id / chunk_size, size)
    }
}

fn is_part1_valid(id: usize) -> bool {
    let digits = id.ilog10() + 1;

    if digits % 2 == 1 {
        return true;
    }

    !is_repetition(id, digits / 2)
}

fn is_part2_valid(id: usize) -> bool {
    let min = 1;
    let max = id.ilog10().div_ceil(2);

    (min..=max).all(|size| !is_repetition(id, size))
}

fn parse(input: &str) -> impl Iterator<Item = usize> {
    input
        .trim()
        .split(",")
        .map(|rng| {
            let (start, stop) = rng.split_once("-").unwrap();
            let start: usize = start.parse().ok().unwrap();
            let stop: usize = stop.parse().ok().unwrap();
            (start, stop)
        })
        .flat_map(|(start, stop)| start..=stop)
}

pub fn part1(input: &str) -> usize {
    parse(input).filter(|&id| !is_part1_valid(id)).sum()
}

pub fn part2(input: &str) -> usize {
    parse(input).filter(|&id| !is_part2_valid(id)).sum()
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_is_repetition() {
        let tests = vec![
            (456456, 3, true),
            (44, 1, true),
            (555, 1, true),
            (555555, 1, true),
            (555555, 2, true),
            (555555, 3, true),
            (555555, 4, false),
            (555555, 5, false),
        ];

        for (id, size, expected) in tests {
            //let id = id.as_bytes();
            assert_eq!(expected, is_repetition(id, size), "{id} -- {size}");
        }
    }
}

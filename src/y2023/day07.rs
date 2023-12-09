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
use thiserror::Error;

use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandKind {
    High,
    Pair,
    Dubs,
    Trip,
    Full,
    Four,
    Five,
}

fn hand_kind(cards: &str) -> Result<HandKind, Error> {
    if cards.len() != 5 {
        return Err(Error::HandParseError(cards.to_string()));
    }

    let fs = freqs(cards.chars());

    let kinds = fs
        .iter()
        .map(|(_, cnt)| match cnt {
            5 => Ok(HandKind::Five),
            4 => Ok(HandKind::Four),
            3 => Ok(HandKind::Trip),
            2 => Ok(HandKind::Pair),
            1 => Ok(HandKind::High),
            _ => Err(Error::HandParseError(cards.to_string())),
        })
        .collect::<Result<Vec<HandKind>, _>>()?;

    let num_pairs = kinds.iter().filter(|&&x| x == HandKind::Pair).count();
    let &kind = kinds.iter().max().unwrap();
    let &num_wild = fs.get(&'Z').unwrap_or(&0);

    let special_map = vec![
        (HandKind::Five, kind == HandKind::Four && num_wild == 1),
        (HandKind::Five, kind == HandKind::Trip && num_wild == 2),
        (
            HandKind::Five,
            kind == HandKind::Trip && num_wild == 3 && num_pairs == 1,
        ),
        (HandKind::Five, kind == HandKind::Four && num_wild == 4),
        
        (HandKind::Four, kind == HandKind::Trip && num_wild == 1),
        (
            HandKind::Four,
            kind == HandKind::Pair && num_wild == 2 && num_pairs == 2,
        ),
        (HandKind::Four, kind == HandKind::Trip && num_wild == 3 ),
        (HandKind::Full, kind == HandKind::Trip && num_pairs == 1),
        (HandKind::Full, num_pairs == 2 && num_wild == 1),
        (HandKind::Trip, kind == HandKind::Pair && num_wild == 1 && num_pairs == 1),
        (HandKind::Trip, kind == HandKind::Pair && num_wild == 2 && num_pairs == 1),
        (HandKind::Dubs, num_pairs == 2),
        (HandKind::Pair, kind == HandKind::High && num_wild == 1),
    ];

    let kind = special_map
        .into_iter()
        .find(|&(_, b)| b == true)
        .map(|(x, _)| x)
        .unwrap_or(kind);

    Ok(kind)
}

fn hand_cmp(hk1: &HandKind, cs1: &str, hk2: &HandKind, cs2: &str) -> std::cmp::Ordering {
    if hk1 != hk2 {
        return hk1.cmp(&hk2);
    }

    for (c1, c2) in cs1.chars().zip(cs2.chars()) {
        if c1 == c2 {
            continue;
        }

        let r1 = rank(c1).unwrap();
        let r2 = rank(c2).unwrap();

        return r1.cmp(&r2);
    }

    std::cmp::Ordering::Equal
}

fn rank(c: char) -> Result<u32, Error> {
    let v = match c {
        'Z' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => return Err(Error::RankParseError(c)),
    };

    Ok(v)
}

fn freqs<T: Eq + Hash>(elems: impl Iterator<Item = T>) -> HashMap<T, u32> {
    elems.into_iter().fold(HashMap::new(), |mut m, elem| {
        *m.entry(elem).or_default() += 1;
        m
    })
}

#[derive(Debug, PartialEq, Eq, Error)]
enum Error {
    #[error("Failed to parse {0} into Hand")]
    HandParseError(String),

    #[error("Failed to parse rank from '{0}'")]
    RankParseError(char),
}

fn hand_sum(input: &str) -> Result<String, String> {
    let hands = match parse::parse(input) {
        Ok((_, h)) => h,
        Err(why) => return Err(format!("Failed to parse input: {}", why)),
    };
    let mut hands = hands
        .into_iter()
        .map(|(cs, bid)| hand_kind(cs).map(|k| (k, cs, bid)))
        .collect::<Result<Vec<(HandKind, &str, u32)>, Error>>()
        .map_err(|e| e.to_string())?;

    hands.sort_by(|(hk1, cs1, _), (hk2, cs2, _)| hand_cmp(hk1, cs1, hk2, cs2));

    let ans: u32 = hands
        .into_iter()
        .enumerate()
        .map(|(i, (_, _, bid))| bid * (i as u32 + 1))
        .sum();

    Ok(ans.to_string())
}

pub fn part_a(input: &str) -> Result<String, String> {
    hand_sum(input)
}

pub fn part_b(input: &str) -> Result<String, String> {
    let input = input.replace("J", "Z");
    hand_sum(input.as_str())
}

mod parse {
    use nom::{
        character::complete::{alphanumeric1, newline, space1, u32},
        combinator::{all_consuming, map_res, opt},
        multi::separated_list1,
        sequence::{separated_pair, terminated},
        IResult,
    };

    use super::Error;

    pub fn parse(input: &str) -> IResult<&str, Vec<(&str, u32)>> {
        all_consuming(terminated(
            separated_list1(
                newline,
                separated_pair(map_res(alphanumeric1, hand), space1, u32),
            ),
            opt(newline),
        ))(input)
    }

    fn hand(cards: &str) -> Result<&str, Error> {
        if !cards.chars().any(|c| {
            c.is_digit(10) || c == 'A' || c == 'K' || c == 'Q' || c == 'J' || c == 'T' || c == 'Z'
        }) {
            Err(Error::HandParseError(cards.to_string()))
        } else {
            Ok(cards)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/2023-12-07.1.txt");

    #[test]
    fn test_part_a() {
        let input = EXAMPLE;
        let expected = Ok("6440".to_string());
        let actual = part_a(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = include_str!("../../examples/2023-12-07.five.txt");
        let (_, hands) = parse::parse(input).unwrap();
        for (cs, _) in hands {
            assert_eq!(Ok(HandKind::Five), hand_kind(cs), "Cards: {}", cs)
        }

        let input = include_str!("../../examples/2023-12-07.four.txt");
        let (_, hands) = parse::parse(input).unwrap();
        for (cs, _) in hands {
            assert_eq!(Ok(HandKind::Four), hand_kind(cs), "Cards: {}", cs)
        }

        let input = include_str!("../../examples/2023-12-07.trip.txt");
        let (_, hands) = parse::parse(input).unwrap();
        for (cs, _) in hands {
            assert_eq!(Ok(HandKind::Trip), hand_kind(cs), "Cards: {}", cs)
        }

        let input = include_str!("../../examples/2023-12-07.dubs.txt");
        let (_, hands) = parse::parse(input).unwrap();
        for (cs, _) in hands {
            assert_eq!(Ok(HandKind::Dubs), hand_kind(cs), "Cards: {}", cs)
        }

        let input = include_str!("../../examples/2023-12-07.pair.txt");
        let (_, hands) = parse::parse(input).unwrap();
        for (cs, _) in hands {
            assert_eq!(Ok(HandKind::Pair), hand_kind(cs), "Cards: {}", cs)
        }

        let input = EXAMPLE;
        let expected = Ok("5905".to_string());
        let actual = part_b(input);
        assert_eq!(expected, actual);
    }
}

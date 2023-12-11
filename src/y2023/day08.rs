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

use std::collections::HashMap;
use itertools::Itertools;
use itertools::FoldWhile::{Done, Continue};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to parse input string: {0}")]
    InputParseError(String),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    L,
    R,
}

pub fn part_a(input: &str) -> Result<String, String> {
    let (directions, tree) = match parse::parse(input) {
        Ok((_, (d, t))) => (d, t),
        Err(why) => return Err(Error::InputParseError(why.to_string()).to_string()),
    };

    let mut i = 0;
    let mut step = "AAA";
    let mut step_count = 1;
    loop {
        step = match directions[i] {
            Direction::L => tree.get(step).unwrap().0,
            Direction::R => tree.get(step).unwrap().1,
        };

        if step == "ZZZ" {
            break;
        }

        i = (i + 1) % directions.len();
        step_count += 1;
    }

    Ok(step_count.to_string())
}

fn start_positions<'a>(tree: &HashMap<&'a str, (&'a str, &'a str)>) -> Vec<&'a str> {
    tree.keys()
        .into_iter()
        .filter(|&&s| s.ends_with("A"))
        .map(|&s| s)
        .collect()
}

fn steps<'a>(start: &'a str, directions: &Vec<Direction>, tree: &HashMap<&'a str, (&'a str, &'a str)>) -> usize {
    directions.iter().cycle().fold_while((start, 0), |(node, steps), &d|{
        let next_node = match d {
            Direction::L => tree.get(node).unwrap().0,
            Direction::R => tree.get(node).unwrap().1,
        };

        if next_node.ends_with("Z") {
            Done((next_node, steps + 1))
        } else {
            Continue((next_node, steps + 1))
        }
    }).into_inner().1
    
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

fn gcd(a: usize, b: usize) -> usize {
    let mut min = a.min(b);
    let mut max = a.max(b);

    loop {
        match max % min {
            0 => return min,
            r => {
                max = min;
                min = r;
            }
        }
    }

}

pub fn part_b(input: &str) -> Result<String, String> {
    let (directions, tree) = match parse::parse(input) {
        Ok((_, (d, t))) => (d, t),
        Err(why) => return Err(Error::InputParseError(why.to_string()).to_string()),
    };

    let positions = start_positions(&tree);

    let ans = positions.iter().map(|&p| steps(p, &directions, &tree)).reduce(|a,b| lcm(a, b)).unwrap();
    Ok(format!("{}", ans))
}

mod parse {
    use super::*;
    use std::collections::HashMap;

    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{alphanumeric1, char, newline, space0, space1},
        combinator::{all_consuming, map, opt, value},
        multi::{many1, separated_list1},
        sequence::{delimited, separated_pair, terminated},
        IResult,
    };

    pub fn parse(input: &str) -> IResult<&str, (Vec<Direction>, HashMap<&str, (&str, &str)>)> {
        all_consuming(terminated(
            separated_pair(
                terminated(
                    many1(alt((
                        value(Direction::L, char('L')),
                        value(Direction::R, char('R')),
                    ))),
                    newline,
                ),
                newline,
                map(
                    separated_list1(
                        newline,
                        separated_pair(
                            alphanumeric1,
                            delimited(space1, tag("="), space1),
                            delimited(
                                tag("("),
                                separated_pair(
                                    alphanumeric1,
                                    delimited(space0, tag(","), space0),
                                    alphanumeric1,
                                ),
                                tag(")"),
                            ),
                        ),
                    ),
                    |lines| lines.into_iter().collect::<HashMap<&str, (&str, &str)>>(),
                ),
            ),
            opt(newline),
        ))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = include_str!("../../examples/2023-12-08.1.txt");
        let expected = Ok("2".to_string());
        let actual = part_a(input);
        assert_eq!(expected, actual);

        let input = include_str!("../../examples/2023-12-08.2.txt");
        let expected = Ok("6".to_string());
        let actual = part_a(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = include_str!("../../examples/2023-12-08.3.txt");
        let expected = Ok("6".to_string());
        let actual = part_b(input);
        assert_eq!(expected, actual);
    }
}

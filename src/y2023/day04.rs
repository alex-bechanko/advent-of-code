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

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{all_consuming, map_res, opt},
    multi::separated_list1,
    sequence::{delimited, pair, preceded, terminated, tuple},
};

pub fn part_a(input: &str) -> Result<String, String> {
    let cards = parse(input)?;

    let ans: u32 = winners(&cards).filter(|&c| c != 0).map(|c| 2u32.pow(c - 1)).sum();

    Ok(ans.to_string())
}

pub fn part_b(input: &str) -> Result<String, String> {
    let cards = parse(input)?;

    let mut multiplier = vec![1u32 ; cards.len()];
    let wins: Vec<u32> = winners(&cards).collect();


    for (i, wins) in wins.iter().enumerate() {
        for _ in 0..multiplier[i] {
            for j in 1..wins + 1 {
                let ind = i + j as usize;
                multiplier[ind] += 1;
            }
        }
    }

    let ans: u32 = multiplier.iter().sum();
    Ok(ans.to_string())
}

fn winners(cards: &Vec<(u32, Vec<u32>, Vec<u32>)>) -> impl Iterator<Item=u32> + '_ {
    cards.iter().map(
        |(_, wins, nums)|
            nums
                .iter()
                .filter_map(|&n| wins.iter().find(|&&x| x == n))
                .count() as u32
        )
}


fn parse(input: &str) -> Result<Vec<(u32, Vec<u32>, Vec<u32>)>, String> {
    let ans: nom::IResult<&str, Vec<(u32, Vec<u32>, Vec<u32>)>> =
        all_consuming(terminated(separated_list1(newline, card), opt(newline)))(input);

    match ans {
        Ok((_, ans)) => Ok(ans),
        Err(why) => Err(format!("Failed to parse input: {}", why)),
    }
}

fn card(input: &str) -> nom::IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    tuple((
        delimited(pair(tag("Card"), space1), number, pair(tag(":"), space1)),
        separated_list1(space1, number),
        preceded(
            delimited(space1, tag("|"), space1),
            separated_list1(space1, number),
        ),
    ))(input)
}

fn number(input: &str) -> nom::IResult<&str, u32> {
    map_res(digit1, str::parse)(input)
}

#[cfg(test)]
mod test {
    use super::*;
    const EXAMPLE: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_parse() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
        let expected = Ok(vec![
            (
                1,
                vec![41, 48, 83, 86, 17],
                vec![83, 86, 6, 31, 17, 9, 48, 53],
            ),
            (
                2,
                vec![13, 32, 20, 16, 61],
                vec![61, 30, 68, 82, 17, 32, 24, 19],
            ),
        ]);
        let actual = parse(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_winners() {
        let input = parse(EXAMPLE).unwrap();
        let expected: Vec<u32> = vec![4, 2, 2, 1, 0, 0];
        let actual: Vec<u32> = winners(&input).collect();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_a() {
        let input = EXAMPLE;
        let expected = Ok("13".to_string());
        let actual = part_a(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = EXAMPLE;
        let expected = Ok("30".to_string());
        let actual = part_b(input);
        assert_eq!(expected, actual);
    }
}

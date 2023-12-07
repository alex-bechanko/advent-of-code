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

pub fn part_a(input: &str) -> Result<String, String> {
    let (times, distances) = parse::part_a(input)?;
    let num_races = times.len();

    // find button push times such that (time - button) * button > distance
    let mut acc = 1;
    for i in 0..num_races {
        let t = times[i];
        let d = distances[i];

        let mut count = 0;
        for b in 0..t - 1 {
            if (t - b) * b > d {
                count += 1;
            }
        }

        acc *= count;
    }

    Ok(acc.to_string())
}

pub fn part_b(input: &str) -> Result<String, String> {
    let (time, distance) = parse::part_b(input)?;

    let mut count = 0;
    for button in 1..time - 1 {
        if (time - button) * button > distance {
            count += 1;
        }
    }

    Ok(count.to_string())
}

mod parse {
    use nom::bytes::complete::tag;
    use nom::character::complete::{digit1, newline, one_of, space1};
    use nom::combinator::{all_consuming, map_res, opt};
    use nom::multi::{many1, separated_list1};
    use nom::sequence::{pair, preceded, separated_pair, terminated};
    use nom::IResult;

    pub fn part_a(input: &str) -> Result<(Vec<u32>, Vec<u32>), String> {
        let res: IResult<&str, (Vec<u32>, Vec<u32>)> = all_consuming(terminated(
            separated_pair(
                preceded(
                    pair(tag("Time:"), space1),
                    separated_list1(space1, map_res(digit1, str::parse)),
                ),
                newline,
                preceded(
                    pair(tag("Distance:"), space1),
                    separated_list1(space1, map_res(digit1, str::parse)),
                ),
            ),
            opt(newline),
        ))(input);

        match res {
            Ok((_, r)) => Ok(r),
            Err(why) => Err(format!("Failed to parse input: {}", why)),
        }
    }

    pub fn part_b(input: &str) -> Result<(u64, u64), String> {
        let res: IResult<&str, (u64, u64)> = all_consuming(terminated(
            separated_pair(
                preceded(
                    pair(tag("Time:"), space1),
                    number_with_kerning,
                ),
                newline,
                preceded(
                    pair(tag("Distance:"), space1),
                    number_with_kerning,
                ),
            ),
            opt(newline),
        ))(input);

        match res {
            Ok((_, r)) => Ok(r),
            Err(why) => Err(format!("Failed to parse input: {}", why)),
        }
    }

    fn number_with_kerning(input: &str) -> IResult<&str, u64> {
        map_res(
            many1(terminated(one_of("0123456789"), opt(space1))),
            |cs| cs.into_iter().collect::<String>().parse::<u64>()
            )(input)
    }

    #[cfg(test)]
    mod tests {
        use super::number_with_kerning;

        fn test_number_with_kerning() {
            let input = "9999   9999   1081   1025";
            let expected = Ok(("", 250133010811025));
            let actual = number_with_kerning(input);
            assert_eq!(expected, actual);

        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/2023-12-06.1.txt");

    #[test]
    fn test_part_a() {
        let input = EXAMPLE;
        let expected = Ok("288".to_string());
        let actual = part_a(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = EXAMPLE;
        let expected = Ok("71503".to_string());
        let actual = part_b(input);
        assert_eq!(expected, actual);
    }
}

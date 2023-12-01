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
    let elves = parse(input)?;

    let answer = elves.iter().map(|elf| elf.iter().sum::<u32>()).max();

    match answer {
        None => return Err(String::from("No elves found in input")),
        Some(a) => return Ok(a.to_string()),
    }
}

pub fn part_b(input: &str) -> Result<String, String> {
    let elves = parse(input)?;

    let mut answer = elves
        .iter()
        .map(|elf| elf.iter().sum::<u32>())
        .collect::<Vec<u32>>();
    answer.sort_by(|a, b| b.cmp(a));

    Ok(answer.iter().take(3).sum::<u32>().to_string())
}

fn parse(input: &str) -> Result<Vec<Vec<u32>>, String> {
    match parse::elf_group(input) {
        Err(why) => Err(format!("Failed to parse input: {}", why)),
        Ok(("", out)) => Ok(out),
        Ok((_, _)) => Err(format!("Failed ot parse input: Leftover input found")),
    }
}

mod parse {
    use nom::branch::alt;
    use nom::character::complete::{newline, u32};
    use nom::combinator::{eof, value};
    use nom::multi::many1;
    use nom::sequence::terminated;

    pub fn elf_group(input: &str) -> nom::IResult<&str, Vec<Vec<u32>>> {
        many1(terminated(elf, terminator))(input)
    }

    fn elf(input: &str) -> nom::IResult<&str, Vec<u32>> {
        many1(terminated(food, terminator))(input)
    }

    fn food(input: &str) -> nom::IResult<&str, u32> {
        u32(input)
    }

    fn terminator(input: &str) -> nom::IResult<&str, u32> {
        alt((value(0, newline), value(0, eof)))(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";
        let expected = Ok(String::from("24000"));
        let actual = part_a(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";
        let expected = Ok(String::from("45000"));
        let actual = part_b(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse() {
        let input = "1000\n2000\n3000\n";
        let expected = Ok(vec![vec![1000, 2000, 3000]]);
        let actual = parse(input);
        assert_eq!(expected, actual);

        let input = "1000\n2000\n3000";
        let expected = Ok(vec![vec![1000, 2000, 3000]]);
        let actual = parse(input);
        assert_eq!(expected, actual);

        let input = "1000\n2000\n3000\n\n4000";
        let expected = Ok(vec![vec![1000, 2000, 3000], vec![4000]]);
        let actual = parse(input);
        assert_eq!(expected, actual);

        let input = "1000\n2000\n3000\n\n4000\n";
        let expected = Ok(vec![vec![1000, 2000, 3000], vec![4000]]);
        let actual = parse(input);
        assert_eq!(expected, actual);

        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let expected = Ok(vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]);
        let actual = parse(input);
        assert_eq!(expected, actual);

        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n";
        let expected = Ok(vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ]);
        let actual = parse(input);
        assert_eq!(expected, actual);
    }
}

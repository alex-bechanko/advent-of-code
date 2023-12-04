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
    let games = match parse::games(input) {
        Ok(("", gs)) => gs,
        _ => return Err("Failed to parse input".to_string()),
    };

    let bag = (12, 13, 14);

    let ans = games
        .iter()
        .filter(|&game| is_game_possible(&game.1, bag))
        .map(|game| game.0)
        .sum::<u32>();

    Ok(ans.to_string())
}

fn is_game_possible(rolls: &Vec<DiceRoll>, bag: DiceRoll) -> bool {
    rolls
        .iter()
        .map(|(r, g, b)| r <= &bag.0 && g <= &bag.1 && b <= &bag.2)
        .fold(true, |acc, v| acc && v)
}

pub fn part_b(input: &str) -> Result<String, String> {
    let games = match parse::games(input) {
        Ok(("", gs)) => gs,
        _ => return Err("Failed to parse input".to_string()),
    };

    let ans = games.iter().map(|(_, rolls)| power(minimum_dice(&rolls))).sum::<u32>();

    Ok(ans.to_string())
}

fn minimum_dice(rolls: &Vec<DiceRoll>) -> DiceRoll {
    rolls.iter().fold((0,0,0), |(x,y,z), &(a, b ,c)| (x.max(a), y.max(b), z.max(c)))
}

fn power(roll: DiceRoll) -> u32 {
    roll.0 * roll.1 * roll.2
}

type DiceRoll = (u32, u32, u32);
type Game = (u32, Vec<DiceRoll>);

mod parse {
    use nom::{
        branch::alt,
        bytes::complete::tag,
        character::complete::{newline, u32},
        combinator::{eof, map, opt, value},
        multi::{fold_many1, many1},
        sequence::{delimited, separated_pair, terminated, tuple},
        IResult,
    };

    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    enum Color {
        Red,
        Green,
        Blue,
    }

    pub fn games(input: &str) -> IResult<&str, Vec<super::Game>> {
        many1(terminated(game, alt((newline, value('\n', eof)))))(input)
    }

    pub fn game(input: &str) -> IResult<&str, super::Game> {
        tuple((game_id, many1(dice_roll)))(input)
    }

    pub fn game_id(input: &str) -> IResult<&str, u32> {
        delimited(tag("Game "), u32, tag(": "))(input)
    }

    pub fn dice_roll(input: &str) -> IResult<&str, (u32, u32, u32)> {
        let zero = || (0, 0, 0);
        let add = |(r1, g1, b1), (r2, g2, b2)| (r1 + r2, g1 + g2, b1 + b2);

        terminated(
            fold_many1(colored_dice, zero, add),
            opt(alt((tag("; "), value("; ", eof)))),
        )(input)
    }

    pub fn colored_dice(input: &str) -> IResult<&str, (u32, u32, u32)> {
        map(
            terminated(
                separated_pair(
                    u32,
                    tag(" "),
                    alt((
                        value(Color::Red, tag("red")),
                        value(Color::Green, tag("green")),
                        value(Color::Blue, tag("blue")),
                    )),
                ),
                opt(tag(", ")),
            ),
            |dice| match dice {
                (x, Color::Red) => (x, 0, 0),
                (x, Color::Green) => (0, x, 0),
                (x, Color::Blue) => (0, 0, x),
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n";

    #[test]
    fn test_part_a() {
        let input = EXAMPLE;
        let expected = Ok("8".to_string());
        let actual = part_a(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = EXAMPLE;
        let expected = Ok("2286".to_string());
        let actual = part_b(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse() {
        //game_id
        let input = "Game 132: ";
        let expected = Ok(("", 132));
        let actual = parse::game_id(input);
        assert_eq!(expected, actual);

        let input = "Game 12: ";
        let expected = Ok(("", 12));
        let actual = parse::game_id(input);
        assert_eq!(expected, actual);

        //colored_dice
        let input = "23 red";
        let expected = Ok(("", (23, 0, 0)));
        let actual = parse::colored_dice(input);
        assert_eq!(expected, actual);

        let input = "23 red, ";
        let expected = Ok(("", (23, 0, 0)));
        let actual = parse::colored_dice(input);
        assert_eq!(expected, actual);

        //handroll
        let input = "23 red, 45 blue, 12 green";
        let expected = Ok(("", (23, 12, 45)));
        let actual = parse::dice_roll(input);
        assert_eq!(expected, actual);

        let input = "14 green, 2 red, 99 blue";
        let expected = Ok(("", (2, 14, 99)));
        let actual = parse::dice_roll(input);
        assert_eq!(expected, actual);

        //game
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Ok(("", (1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)])));
        let actual = parse::game(input);
        assert_eq!(expected, actual);

        //games
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue";
        let expected = Ok((
            "",
            vec![
                (1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
                (2, vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
            ],
        ));
        let actual = parse::games(input);
        assert_eq!(expected, actual);

        let input = EXAMPLE;
        let expected = Ok((
            "",
            vec![
                (1, vec![(4, 0, 3), (1, 2, 6), (0, 2, 0)]),
                (2, vec![(0, 2, 1), (1, 3, 4), (0, 1, 1)]),
                (3, vec![(20, 8, 6), (4, 13, 5), (1, 5, 0)]),
                (4, vec![(3, 1, 6), (6, 3, 0), (14, 3, 15)]),
                (5, vec![(6, 3, 1), (1, 2, 2)]),
            ],
        ));
        let actual = parse::games(input);
        assert_eq!(expected, actual);
    }
}

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
    let answer = input.lines().filter_map(parse::part_a_number).sum::<u32>();

    //let answer = calibration_values.iter().sum::<u32>();

    Ok(answer.to_string())
}

pub fn part_b(input: &str) -> Result<String, String> {
    let answer = input.lines().filter_map(parse::part_b_number).sum::<u32>();
    
    Ok(answer.to_string())
}

mod parse {
    const DIGITS: [&str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    const DIGITS_WORDS: [&str; 20] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
        "five", "six", "seven", "eight", "nine",
    ];

    pub fn part_a_number(line: &str) -> Option<u32> {
        let left = find(line, &DIGITS).map(|n| n as u32)?;
        let right = rfind(line, &DIGITS).map(|n| n as u32)?;

        Some(left * 10 + right)
    }

    pub fn part_b_number(line: &str) -> Option<u32> {
        let left = find(line, &DIGITS_WORDS).map(|n| (n as u32) % 10)?;
        let right = rfind(line, &DIGITS_WORDS).map(|n| (n as u32) % 10)?;

        Some(left * 10 + right)
    }

    pub fn find(line: &str, words: &[&str]) -> Option<usize> {
        words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| line.find(w).map(|j| (i, j)))
            .min_by(|x,y| x.1.cmp(&y.1))
            .map(|x| x.0)
    }

    pub fn rfind(line: &str, words: &[&str]) -> Option<usize> {
        words
            .iter()
            .enumerate()
            .filter_map(|(i, w)| line.rfind(w).map(|j| (i, j)))
            .min_by(|x,y| y.1.cmp(&x.1))
            .map(|x| x.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_a() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntre7uchet\n";
        let expected = Ok(String::from("142"));
        let actual = part_a(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen\n";
        let expected = Ok("281".to_string());
        let actual = part_b(input);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_find() {
        let input = "1abc2";
        let words = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let expected = Some(1);
        let actual = parse::find(input, &words);
        assert_eq!(expected, actual);

        let input = "two1nine";
        let words = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let expected = Some(2);
        let actual = parse::find(input, &words);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_parse_rfind() {
        let input = "1abc2";
        let words = vec!["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
        let expected = Some(2);
        let actual = parse::rfind(input, &words);
        assert_eq!(expected, actual);

        let input = "two1nine";
        let words = vec![
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let expected = Some(9);
        let actual = parse::rfind(input, &words);
        assert_eq!(expected, actual);
    }
}

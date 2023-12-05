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

use std::{str::FromStr, u128};

use thiserror::Error;

pub fn part_a(input: &str) -> Result<String, String> {
    let almanac = parse(input)?;

    let order = vec![
        AlmanacItem::Seed,
        AlmanacItem::Soil,
        AlmanacItem::Fertilizer,
        AlmanacItem::Water,
        AlmanacItem::Light,
        AlmanacItem::Temperature,
        AlmanacItem::Humidity,
        AlmanacItem::Location,
    ];

    let locations = almanac
        .seeds
        .iter()
        .map(|&s| {
            order
                .windows(2)
                .fold(Ok(s), |acc, w| acc.and_then(|a| almanac.get(w[0], w[1], a)))
        }).collect::<Result<Vec<u128>, AlmanacError>>().map_err(|e| e.to_string())?;

    let &ans = locations.iter().min().ok_or(format!("Empty seeds array"))?;

    Ok(ans.to_string())
}

pub fn part_b(input: &str) -> Result<String, String> {
    let almanac = parse(input)?;

    let order = vec![
        AlmanacItem::Seed,
        AlmanacItem::Soil,
        AlmanacItem::Fertilizer,
        AlmanacItem::Water,
        AlmanacItem::Light,
        AlmanacItem::Temperature,
        AlmanacItem::Humidity,
        AlmanacItem::Location,
    ];

    let mut ans = u128::MAX;
    for sw in almanac.seeds.chunks(2) {
        let (start, length) = (sw[0], sw[1]);

        for s in start..start+length {

            // get the location
            let mut acc = s;
            for ow in order.windows(2) {
                let (src, dest) = (ow[0], ow[1]);
                acc = almanac.get(src, dest, acc).map_err(|e| e.to_string())?;

            }

            ans = ans.min(acc);
        }
    }

    Ok(ans.to_string())
}

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Almanac {
    seeds: Vec<u128>,
    maps: Vec<AlmanacRange>,
}

impl Almanac {
    fn get(&self, src: AlmanacItem, dest: AlmanacItem, value: u128) -> Result<u128, AlmanacError> {
        let m = self
            .maps
            .iter()
            .find(|arange| arange.src == src && arange.dest == dest)
            .ok_or(AlmanacError::ItemNotFound(src, dest))?;

        Ok(m.get(value))
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
enum AlmanacError {
    #[error("Failed to find map for {0:?} -> {1:?}")]
    ItemNotFound(AlmanacItem, AlmanacItem),
}

#[derive(Debug, PartialEq, PartialOrd)]
struct AlmanacRange {
    src: AlmanacItem,
    dest: AlmanacItem,
    ranges: Vec<AlmanacRangeItem>,
}

impl AlmanacRange {
    fn get(&self, value: u128) -> u128 {
        let v = self
            .ranges
            .iter()
            .find(|rng| rng.contains(value))
            .and_then(|rng| rng.get(value));
        match v {
            None => value,
            Some(x) => x,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct AlmanacRangeItem {
    src_start: u128,
    dest_start: u128,
    length: u128,
}

impl AlmanacRangeItem {
    fn contains(&self, value: u128) -> bool {
        value >= self.src_start && value <= (self.src_start + self.length - 1)
    }

    fn get(&self, value: u128) -> Option<u128> {
        if self.contains(value) {
            Some(self.dest_start + value - self.src_start)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum AlmanacItem {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl FromStr for AlmanacItem {
    type Err = ParseAlmanacError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "seed" => Ok(AlmanacItem::Seed),
            "soil" => Ok(AlmanacItem::Soil),
            "fertilizer" => Ok(AlmanacItem::Fertilizer),
            "water" => Ok(AlmanacItem::Water),
            "light" => Ok(AlmanacItem::Light),
            "temperature" => Ok(AlmanacItem::Temperature),
            "humidity" => Ok(AlmanacItem::Humidity),
            "location" => Ok(AlmanacItem::Location),
            _ => Err(ParseAlmanacError::ParseItemFailure),
        }
    }
}

#[derive(Debug, Error)]
enum ParseAlmanacError {
    #[error("Failed to parse mapping type")]
    ParseItemFailure,
}

fn parse(input: &str) -> Result<Almanac, String> {
    match nom::combinator::all_consuming(parse::almanac)(input) {
        Ok((_, a)) => Ok(a),
        Err(_) => Err(format!("Failed to parse almanac")),
    }
}

mod parse {
    use nom::{
        bytes::complete::tag,
        character::complete::{alpha1, digit1, newline, space0, space1},
        combinator::{map_res, opt},
        multi::separated_list1,
        sequence::{delimited, pair, separated_pair, terminated},
        IResult,
    };

    use super::{Almanac, AlmanacItem, AlmanacRange, AlmanacRangeItem};

    pub fn almanac(input: &str) -> IResult<&str, Almanac> {
        let (input, seeds) = terminated(seeds, newline)(input)?;
        let (input, maps) = terminated(
            separated_list1(pair(newline, newline), almanac_range),
            opt(newline),
        )(input)?;

        let almanac = Almanac { seeds, maps };

        Ok((input, almanac))
    }

    fn seeds(input: &str) -> IResult<&str, Vec<u128>> {
        delimited(
            pair(tag("seeds:"), space1),
            separated_list1(space1, map_res(digit1, str::parse)),
            newline,
        )(input)
    }

    fn almanac_range(input: &str) -> IResult<&str, AlmanacRange> {
        let (input, (src, dest)) = terminated(
            separated_pair(almanac_item, tag("-to-"), almanac_item),
            pair(space1, tag("map:\n")),
        )(input)?;

        let (input, ranges) = separated_list1(newline, almanac_range_item)(input)?;

        let rng = AlmanacRange { src, dest, ranges };

        Ok((input, rng))
    }

    fn almanac_range_item(input: &str) -> IResult<&str, AlmanacRangeItem> {
        let (input, dest_start) = terminated(map_res(digit1, str::parse), space1)(input)?;
        let (input, src_start) = terminated(map_res(digit1, str::parse), space1)(input)?;
        let (input, length) = terminated(map_res(digit1, str::parse), space0)(input)?;

        let item = AlmanacRangeItem {
            src_start,
            dest_start,
            length,
        };

        Ok((input, item))
    }

    fn almanac_item(input: &str) -> IResult<&str, AlmanacItem> {
        map_res(alpha1, str::parse)(input)
    }

    #[cfg(test)]
    mod test {
        use super::*;
        const EXAMPLE: &str = include_str!("../../examples/2023-12-05.1.txt");

        #[test]
        fn test_seeds() {
            let input = "seeds: 79 14 55 13\n";
            let expected = Ok(("", vec![79, 14, 55, 13]));
            let actual = seeds(input);

            assert_eq!(expected, actual);
        }

        #[test]
        fn test_almanac_range() {
            let input = "seed-to-soil map:\n50 98 2\n52 50 48";
            let expected = Ok((
                "",
                AlmanacRange {
                    src: AlmanacItem::Seed,
                    dest: AlmanacItem::Soil,
                    ranges: vec![
                        AlmanacRangeItem {
                            src_start: 98,
                            dest_start: 50,
                            length: 2,
                        },
                        AlmanacRangeItem {
                            src_start: 50,
                            dest_start: 52,
                            length: 48,
                        },
                    ],
                },
            ));
            let actual = almanac_range(input);
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_almanac_item() {
            let input = "seed";
            let expected = Ok(("", AlmanacItem::Seed));
            let actual = almanac_item(input);
            assert_eq!(expected, actual);

            let input = "soil";
            let expected = Ok(("", AlmanacItem::Soil));
            let actual = almanac_item(input);
            assert_eq!(expected, actual);

            let input = "fertilizer";
            let expected = Ok(("", AlmanacItem::Fertilizer));
            let actual = almanac_item(input);
            assert_eq!(expected, actual);

            let input = "water";
            let expected = Ok(("", AlmanacItem::Water));
            let actual = almanac_item(input);
            assert_eq!(expected, actual);

            let input = "light";
            let expected = Ok(("", AlmanacItem::Light));
            let actual = almanac_item(input);
            assert_eq!(expected, actual);

            let input = "temperature";
            let expected = Ok(("", AlmanacItem::Temperature));
            let actual = almanac_item(input);
            assert_eq!(expected, actual);

            let input = "humidity";
            let expected = Ok(("", AlmanacItem::Humidity));
            let actual = almanac_item(input);
            assert_eq!(expected, actual);

            let input = "location";
            let expected = Ok(("", AlmanacItem::Location));
            let actual = almanac_item(input);
            assert_eq!(expected, actual);
        }

        #[test]
        fn test_almanac() {
            use pretty_assertions::assert_eq;
            let input = EXAMPLE;
            let expected = Ok((
                "",
                Almanac {
                    seeds: vec![79, 14, 55, 13],
                    maps: vec![
                        AlmanacRange {
                            src: AlmanacItem::Seed,
                            dest: AlmanacItem::Soil,
                            ranges: vec![
                                AlmanacRangeItem {
                                    src_start: 98,
                                    dest_start: 50,
                                    length: 2,
                                },
                                AlmanacRangeItem {
                                    src_start: 50,
                                    dest_start: 52,
                                    length: 48,
                                },
                            ],
                        },
                        AlmanacRange {
                            src: AlmanacItem::Soil,
                            dest: AlmanacItem::Fertilizer,
                            ranges: vec![
                                AlmanacRangeItem {
                                    src_start: 15,
                                    dest_start: 0,
                                    length: 37,
                                },
                                AlmanacRangeItem {
                                    src_start: 52,
                                    dest_start: 37,
                                    length: 2,
                                },
                                AlmanacRangeItem {
                                    src_start: 0,
                                    dest_start: 39,
                                    length: 15,
                                },
                            ],
                        },
                        AlmanacRange {
                            src: AlmanacItem::Fertilizer,
                            dest: AlmanacItem::Water,
                            ranges: vec![
                                AlmanacRangeItem {
                                    src_start: 53,
                                    dest_start: 49,
                                    length: 8,
                                },
                                AlmanacRangeItem {
                                    src_start: 11,
                                    dest_start: 0,
                                    length: 42,
                                },
                                AlmanacRangeItem {
                                    src_start: 0,
                                    dest_start: 42,
                                    length: 7,
                                },
                                AlmanacRangeItem {
                                    src_start: 7,
                                    dest_start: 57,
                                    length: 4,
                                },
                            ],
                        },
                        AlmanacRange {
                            src: AlmanacItem::Water,
                            dest: AlmanacItem::Light,
                            ranges: vec![
                                AlmanacRangeItem {
                                    src_start: 18,
                                    dest_start: 88,
                                    length: 7,
                                },
                                AlmanacRangeItem {
                                    src_start: 25,
                                    dest_start: 18,
                                    length: 70,
                                },
                            ],
                        },
                        AlmanacRange {
                            src: AlmanacItem::Light,
                            dest: AlmanacItem::Temperature,
                            ranges: vec![
                                AlmanacRangeItem {
                                    src_start: 77,
                                    dest_start: 45,
                                    length: 23,
                                },
                                AlmanacRangeItem {
                                    src_start: 45,
                                    dest_start: 81,
                                    length: 19,
                                },
                                AlmanacRangeItem {
                                    src_start: 64,
                                    dest_start: 68,
                                    length: 13,
                                },
                            ],
                        },
                        AlmanacRange {
                            src: AlmanacItem::Temperature,
                            dest: AlmanacItem::Humidity,
                            ranges: vec![
                                AlmanacRangeItem {
                                    src_start: 69,
                                    dest_start: 0,
                                    length: 1,
                                },
                                AlmanacRangeItem {
                                    src_start: 0,
                                    dest_start: 1,
                                    length: 69,
                                },
                            ],
                        },
                        AlmanacRange {
                            src: AlmanacItem::Humidity,
                            dest: AlmanacItem::Location,
                            ranges: vec![
                                AlmanacRangeItem {
                                    src_start: 56,
                                    dest_start: 60,
                                    length: 37,
                                },
                                AlmanacRangeItem {
                                    src_start: 93,
                                    dest_start: 56,
                                    length: 4,
                                },
                            ],
                        },
                    ],
                },
            ));
            let actual = almanac(input);
            assert_eq!(expected, actual);
        }
    }
}

// seed -> soil -> fertilizer -> water -> light -> temp -> humidity -> location

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = include_str!("../../examples/2023-12-05.1.txt");

    #[test]
    fn test_part_a() {
        let input = EXAMPLE;
        let expected = Ok("35".to_string());
        let actual = part_a(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = EXAMPLE;
        let expected = Ok("46".to_string());
        let actual = part_b(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_almanac_range_item() {
        let item = AlmanacRangeItem {
            src_start: 98,
            dest_start: 50,
            length: 2,
        };

        let input = 98;
        let expected = true;
        let actual = item.contains(input);
        assert_eq!(expected, actual, "item.contains({})", input);
        let expected = Some(50);
        let actual = item.get(input);
        assert_eq!(expected, actual, "item.get({})", input);

        let input = 99;
        let expected = true;
        let actual = item.contains(input);
        assert_eq!(expected, actual, "item.contains({})", input);
        let expected = Some(51);
        let actual = item.get(input);
        assert_eq!(expected, actual, "item.get({})", input);

        let input = 100;
        let expected = false;
        let actual = item.contains(input);
        assert_eq!(expected, actual, "item.contains({})", input);
        let expected = None;
        let actual = item.get(input);
        assert_eq!(expected, actual, "item.get({})", input);

        let input = 97;
        let expected = false;
        let actual = item.contains(input);
        assert_eq!(expected, actual, "item.contains({})", input);
        let expected = None;
        let actual = item.get(input);
        assert_eq!(expected, actual, "item.get({})", input);
    }

    #[test]
    fn test_almanac_range() {
        let rng = AlmanacRange {
            src: AlmanacItem::Seed,
            dest: AlmanacItem::Soil,
            ranges: vec![
                AlmanacRangeItem {
                    src_start: 98,
                    dest_start: 50,
                    length: 2,
                },
                AlmanacRangeItem {
                    src_start: 50,
                    dest_start: 52,
                    length: 48,
                },
            ],
        };

        let input = 98;
        let expected = 50;
        let actual = rng.get(input);
        assert_eq!(expected, actual);

        let input = 97;
        let expected = 99;
        let actual = rng.get(input);
        assert_eq!(expected, actual);

        let input = 145;
        let expected = 145;
        let actual = rng.get(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_almanac() {
        let almanac = parse(EXAMPLE).unwrap();
        let seed: u128 = 79;

        let expected = Ok(81);
        let actual = almanac.get(AlmanacItem::Seed, AlmanacItem::Soil, seed);
        assert_eq!(expected, actual);

        let soil = actual.unwrap();
        let expected = Ok(81);
        let actual = almanac.get(AlmanacItem::Soil, AlmanacItem::Fertilizer, soil);
        assert_eq!(expected, actual);

        let fertilizer = actual.unwrap();
        let expected = Ok(81);
        let actual = almanac.get(AlmanacItem::Fertilizer, AlmanacItem::Water, fertilizer);
        assert_eq!(expected, actual);

        let water = actual.unwrap();
        let expected = Ok(74);
        let actual = almanac.get(AlmanacItem::Water, AlmanacItem::Light, water);
        assert_eq!(expected, actual);

        let light = actual.unwrap();
        let expected = Ok(78);
        let actual = almanac.get(AlmanacItem::Light, AlmanacItem::Temperature, light);
        assert_eq!(expected, actual);

        let temp = actual.unwrap();
        let expected = Ok(78);
        let actual = almanac.get(AlmanacItem::Temperature, AlmanacItem::Humidity, temp);
        assert_eq!(expected, actual);

        let humidity = actual.unwrap();
        let expected = Ok(82);
        let actual = almanac.get(AlmanacItem::Humidity, AlmanacItem::Location, humidity);
        assert_eq!(expected, actual);
    }
}

fn bordered(input: &str) -> Option<String> {
    let line_length = input.find('\n')?;

    let bordered_input: String = input
        .lines()
        .map(|l| format!(".{}.", l))
        .collect::<Vec<String>>()
        .join("\n");
    let top_bottom_border = ".".repeat(line_length + 2);

    Some(format!(
        "{}\n{}\n{}",
        top_bottom_border, bordered_input, top_bottom_border
    ))
}

fn scan_numbers(start: usize, prev: &[u8], cur: &[u8], next: &[u8]) -> Vec<u32> {
    let ps = (start - 1..start + 2)
        .filter(|&i| prev[i].is_ascii_digit())
        .map(|x| number(x, prev));
    let cs = (start - 1..start + 2)
        .filter(|&i| cur[i].is_ascii_digit())
        .map(|x| number(x, cur));
    let ns = (start - 1..start + 2)
        .filter(|&i| next[i].is_ascii_digit())
        .map(|x| number(x, next));

    std::collections::HashSet::<_>::from_iter(ps.chain(cs).chain(ns))
        .iter()
        .map(|x| x.1)
        .collect()
}

fn number(i: usize, line: &[u8]) -> (usize, u32) {
    let mut num: Vec<u8> = vec![line[i]];

    let mut cursor = i + 1;
    while line[cursor].is_ascii_digit() {
        num.push(line[cursor]);
        cursor += 1;
    }

    let mut cursor = i - 1;
    while line[cursor].is_ascii_digit() {
        num.insert(0, line[cursor]);
        cursor -= 1;
    }

    (
        cursor + 1,
        std::str::from_utf8(&num).unwrap().parse().unwrap(),
    )
}

fn scan_part_numbers(
    input: &str,
    is_part_identifier: &dyn Fn(u8) -> bool,
    part_number_fold: &dyn Fn(&Vec<u32>) -> u32,
) -> Result<String, String> {
    let input = bordered(input).ok_or("Failed to add border around input")?;

    let mut total = 0;

    for window in input.lines().collect::<Vec<&str>>().windows(3) {
        let prev = window[0].as_bytes();
        let cur = window[1].as_bytes();
        let next = window[2].as_bytes();

        for (j, &c) in cur.iter().enumerate() {
            if !is_part_identifier(c) {
                continue;
            }

            let nums = scan_numbers(j, prev, cur, next);

            total += part_number_fold(&nums);
        }
    }

    Ok(total.to_string())
}

pub fn part_a(input: &str) -> Result<String, String> {
    scan_part_numbers(
        input,
        &|x: u8| !(x.is_ascii_digit() || x == b'.'),
        &|vs: &Vec<u32>| vs.iter().sum::<u32>(),
    )
}

pub fn part_b(input: &str) -> Result<String, String> {
    scan_part_numbers(input, &|x: u8| x == b'*', &|vs: &Vec<u32>| {
        if vs.len() == 2 {
            vs.iter().fold(1, |acc, x| acc * x)
        } else {
            0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";

    #[test]
    fn test_part_a() {
        let input = EXAMPLE;
        let expected = Ok("4361".to_string());
        let actual = part_a(input);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_b() {
        let input = EXAMPLE;
        let expected = Ok("467835".to_string());
        let actual = part_b(input);
        assert_eq!(expected, actual);
    }
}

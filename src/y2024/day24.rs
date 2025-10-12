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

use std::collections::HashMap;

struct Adder<'a> {
    labels: Vec<&'a str>,
    expressions: Vec<Expr<'a>>,

    #[allow(dead_code)]
    num_bits: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Expr<'a> {
    Value(bool),
    Operation(Op, &'a str, &'a str),
}

fn parse(input: &str) -> Option<Adder> {
    let mut labels = vec![];
    let mut expressions = vec![];
    let (initial_values, assignments) = input.split_once("\n\n")?;

    let mut num_bits = 0;
    for e in initial_values.lines() {
        let (name, value) = e.split_once(": ")?;
        let value = value.parse::<u8>().ok()? > 0;

        expressions.push(Expr::Value(value));
        labels.push(name);

        num_bits += 1;
    }

    num_bits /= 2;

    for e in assignments.lines() {
        let (expr, name) = e.split_once(" -> ")?;

        let mut expr = expr.split_whitespace();
        let (left, op, right) = (expr.next()?, expr.next()?, expr.next()?);
        let expr = match op {
            "AND" => Expr::Operation(Op::And, left, right),
            "OR" => Expr::Operation(Op::Or, left, right),
            "XOR" => Expr::Operation(Op::Xor, left, right),
            _ => return None,
        };

        labels.push(name);
        expressions.push(expr);
    }

    Some(Adder {
        labels,
        expressions,
        num_bits,
    })
}

fn evaluate(adder: &Adder, cache: &mut HashMap<String, bool>, var: &str) -> Result<bool, String> {
    if let Some(val) = cache.get(var).copied() {
        return Ok(val);
    }

    let index = adder
        .labels
        .iter()
        .enumerate()
        .find(|(_, l)| **l == var)
        .map(|(i, _)| i)
        .ok_or_else(|| format!("Failed to find variable {var}"))?;

    match adder.expressions[index] {
        Expr::Value(v) => Ok(v),
        Expr::Operation(op, lhs_var, rhs_var) => {
            let lhs = evaluate(adder, cache, lhs_var)?;
            let rhs = evaluate(adder, cache, rhs_var)?;
            let expr_value = match op {
                Op::Or => lhs | rhs,
                Op::And => lhs & rhs,
                Op::Xor => lhs ^ rhs,
            };
            cache.insert(var.to_string(), expr_value);
            Ok(expr_value)
        }
    }
}

fn get_value(adder: &Adder, var_prefix: &str) -> Result<u64, String> {
    let mut output: Vec<&str> = adder
        .labels
        .iter()
        .filter(|k| k.starts_with(var_prefix))
        .copied()
        .collect();
    output.sort_unstable();

    let mut cache = HashMap::new();
    let mut num = 0u64;
    for (i, var) in output.into_iter().enumerate() {
        let value = evaluate(adder, &mut cache, var)?;
        num += (value as u64) << i;
    }

    Ok(num)
}

/// # Panics
pub fn part1(input: &str) -> u64 {
    let adder = parse(input).expect("Failed to parse input");
    get_value(&adder, "z").unwrap()
}

/// # Panics
pub fn part2(_input: &str) -> String {
    "Not implemented".into()
}

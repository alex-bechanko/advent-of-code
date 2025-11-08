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

use core::panic;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
};

// precalculate input and output gate names
static X_GATES: [&str; 100] = [
    "x00", "x01", "x02", "x03", "x04", "x05", "x06", "x07", "x08", "x09", "x10", "x11", "x12",
    "x13", "x14", "x15", "x16", "x17", "x18", "x19", "x20", "x21", "x22", "x23", "x24", "x25",
    "x26", "x27", "x28", "x29", "x30", "x31", "x32", "x33", "x34", "x35", "x36", "x37", "x38",
    "x39", "x40", "x41", "x42", "x43", "x44", "x45", "x46", "x47", "x48", "x49", "x50", "x51",
    "x52", "x53", "x54", "x55", "x56", "x57", "x58", "x59", "x60", "x61", "x62", "x63", "x64",
    "x65", "x66", "x67", "x68", "x69", "x70", "x71", "x72", "x73", "x74", "x75", "x76", "x77",
    "x78", "x79", "x80", "x81", "x82", "x83", "x84", "x85", "x86", "x87", "x88", "x89", "x90",
    "x91", "x92", "x93", "x94", "x95", "x96", "x97", "x98", "x99",
];

static Y_GATES: [&str; 100] = [
    "y00", "y01", "y02", "y03", "y04", "y05", "y06", "y07", "y08", "y09", "y10", "y11", "y12",
    "y13", "y14", "y15", "y16", "y17", "y18", "y19", "y20", "y21", "y22", "y23", "y24", "y25",
    "y26", "y27", "y28", "y29", "y30", "y31", "y32", "y33", "y34", "y35", "y36", "y37", "y38",
    "y39", "y40", "y41", "y42", "y43", "y44", "y45", "y46", "y47", "y48", "y49", "y50", "y51",
    "y52", "y53", "y54", "y55", "y56", "y57", "y58", "y59", "y60", "y61", "y62", "y63", "y64",
    "y65", "y66", "y67", "y68", "y69", "y70", "y71", "y72", "y73", "y74", "y75", "y76", "y77",
    "y78", "y79", "y80", "y81", "y82", "y83", "y84", "y85", "y86", "y87", "y88", "y89", "y90",
    "y91", "y92", "y93", "y94", "y95", "y96", "y97", "y98", "y99",
];

static Z_GATES: [&str; 100] = [
    "z00", "z01", "z02", "z03", "z04", "z05", "z06", "z07", "z08", "z09", "z10", "z11", "z12",
    "z13", "z14", "z15", "z16", "z17", "z18", "z19", "z20", "z21", "z22", "z23", "z24", "z25",
    "z26", "z27", "z28", "z29", "z30", "z31", "z32", "z33", "z34", "z35", "z36", "z37", "z38",
    "z39", "z40", "z41", "z42", "z43", "z44", "z45", "z46", "z47", "z48", "z49", "z50", "z51",
    "z52", "z53", "z54", "z55", "z56", "z57", "z58", "z59", "z60", "z61", "z62", "z63", "z64",
    "z65", "z66", "z67", "z68", "z69", "z70", "z71", "z72", "z73", "z74", "z75", "z76", "z77",
    "z78", "z79", "z80", "z81", "z82", "z83", "z84", "z85", "z86", "z87", "z88", "z89", "z90",
    "z91", "z92", "z93", "z94", "z95", "z96", "z97", "z98", "z99",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Op {
    And,
    Or,
    Xor,
}

impl Op {
    fn eval(self, a: bool, b: bool) -> bool {
        match self {
            Self::And => a & b,
            Self::Or => a | b,
            Self::Xor => a ^ b,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Expr<'a>(Op, &'a str, &'a str);

type Cache<'a> = HashMap<&'a str, bool>;

#[derive(Clone, Debug)]
struct Adder<'a> {
    size: usize,
    var_to_expr: HashMap<&'a str, Expr<'a>>,
}

impl<'a> Adder<'a> {
    fn eval_with_visitors(
        &self,
        visited: &mut HashSet<&'a str>,
        cache: &mut Cache<'a>,
        var: &'a str,
    ) -> Option<bool> {
        if let Some(v) = cache.get(var).copied() {
            return Some(v);
        } else if var.starts_with("x") || var.starts_with("y") {
            return Some(false);
        } else if visited.contains(var) {
            return None;
        }
        visited.insert(var);

        let Expr(op, lhs, rhs) = self.var_to_expr.get(var).copied()?;
        let lhs = self.eval_with_visitors(visited, cache, lhs)?;
        let rhs = self.eval_with_visitors(visited, cache, rhs)?;
        let v = op.eval(lhs, rhs);
        cache.insert(var, v);

        Some(v)
    }

    fn eval(&self, cache: &mut Cache<'a>, var: &'a str) -> Option<bool> {
        let mut visited = HashSet::new();
        self.eval_with_visitors(&mut visited, cache, var)
    }

    fn dependencies(&self, var: &'a str) -> Vec<&'a str> {
        let mut deps = vec![];
        let mut walk = VecDeque::new();
        walk.push_back(var);

        while let Some(d) = walk.pop_front() {
            if d.starts_with("x") || d.starts_with("y") {
                // only add labels that can be swapped, skip the others
                continue;
            }

            deps.push(d);
            let Expr(_, a, b) = self
                .var_to_expr
                .get(d)
                .unwrap_or_else(|| panic!("No expression for {d}"));

            walk.push_back(a);
            walk.push_back(b);
        }

        deps
    }

    fn swap(mut self, gate1: &'a str, gate2: &'a str) -> Self {
        let expr1 = self.var_to_expr[gate1];
        let expr2 = self.var_to_expr[gate2];

        self.var_to_expr.insert(gate1, expr2);
        self.var_to_expr.insert(gate2, expr1);

        self
    }

    fn test(&self, step: usize) -> bool {
        if step < self.size {
            let tests = vec![
                (vec![(X_GATES[step], false), (Y_GATES[step], false)], false),
                (vec![(X_GATES[step], true), (Y_GATES[step], false)], true),
                (vec![(X_GATES[step], false), (Y_GATES[step], true)], true),
            ];

            for (cache, step_test) in tests {
                let mut cache = HashMap::from_iter(cache);
                match self.eval(&mut cache, Z_GATES[step]) {
                    None => return false,
                    Some(x) if x != step_test => return false,
                    _ => {}
                }
            }
        }
        if step > 0 {
            // test carry bit
            let mut cache =
                HashMap::from_iter(vec![(X_GATES[step - 1], true), (Y_GATES[step - 1], true)]);
            match self.eval(&mut cache, Z_GATES[step]) {
                None => return false,
                Some(x) if !x => return false,
                _ => {}
            }
        }

        true
    }
}

fn repair<'a>(
    adder: Adder<'a>,
    step: usize,
    swaps: &mut Vec<(&'a str, &'a str)>,
    max_swaps: usize,
) -> bool {
    if step > adder.size {
        return true;
    }

    // check this `step`'s bit, if it works then continue to the next
    if adder.test(step) {
        return repair(adder, step + 1, swaps, max_swaps);
    } else if max_swaps == 0 {
        return false;
    }

    // testing failed, so we need to try swapping gates related to this step's adder
    // it's assumed that the previous steps are ok
    let deps = adder.dependencies(Z_GATES[step]);
    let deps: HashSet<&str> = HashSet::from_iter(deps);

    let ok_deps = if step != 0 {
        HashSet::from_iter(adder.dependencies(Z_GATES[step - 1]))
    } else {
        HashSet::new()
    };

    let swappable_deps = deps.difference(&ok_deps).copied();

    let fixed_z_gates: HashSet<&str> = HashSet::from_iter(Z_GATES[0..step].iter().copied());
    let invalid_swaps = ok_deps.union(&fixed_z_gates).copied().collect();

    let valid_swaps: HashSet<&str> = HashSet::from_iter(adder.var_to_expr.keys().copied());
    let valid_swaps: HashSet<&str> = valid_swaps.difference(&invalid_swaps).copied().collect();

    for gate1 in swappable_deps {
        for gate2 in valid_swaps.clone() {
            let fixed_adder = adder.clone().swap(gate1, gate2);
            if !fixed_adder.test(step) {
                // failed test, try another pair to swap
                continue;
            }

            // test passed, lets try to repair the rest
            swaps.push((gate1, gate2));
            if repair(fixed_adder, step + 1, swaps, max_swaps - 1) {
                // adder was repaired with these gates swapped
                // add them to swaps and then return
                return true;
            }
            swaps.pop();
        }
    }

    // tried all possible gate combos but none fixed the adder is it is,
    false
}

fn parse(input: &str) -> Option<(Cache<'_>, Adder<'_>)> {
    let (initials, expressions) = input.split_once("\n\n")?;

    let mut cache: HashMap<&str, bool> = HashMap::new();
    for l in initials.lines() {
        let (n, v) = l.split_once(": ")?;
        let v: usize = v.parse().ok()?;
        let v: bool = v > 0;

        cache.insert(n, v);
    }
    let size = cache.len() / 2;

    let mut var_to_expr = HashMap::new();
    for l in expressions.lines() {
        let (e, n) = l.split_once(" -> ")?;
        let mut e = e.split_whitespace();
        let (lhs, op, rhs) = (e.next()?, e.next()?, e.next()?);
        let op = match op {
            "AND" => Op::And,
            "OR" => Op::Or,
            "XOR" => Op::Xor,
            _ => return None,
        };
        let e = Expr(op, lhs, rhs);
        var_to_expr.insert(n, e);
    }

    let adder = Adder { size, var_to_expr };

    Some((cache, adder))
}

/// # Panics
pub fn part1(input: &str) -> usize {
    let (mut cache, adder) = parse(input).expect("Failed to parse input");
    let zs = &Z_GATES[0..adder.size];

    let mut ans = 0usize;
    for (i, var) in zs.iter().enumerate() {
        let v = adder
            .eval(&mut cache, var)
            .unwrap_or_else(|| panic!("Failed to evaluate z{i:02} bit"));
        ans += (v as usize) << i;
    }

    ans
}

/// # Panics
pub fn part2(input: &str) -> String {
    let (_, adder) = parse(input).expect("Failed to parse input");

    let mut swaps = vec![];
    let fixable = repair(adder, 0, &mut swaps, 4);
    if !fixable {
        panic!("Adder provided by input not fixable in 4 swaps");
    }
    let mut ans = vec![];
    for (a, b) in swaps {
        ans.push(a);
        ans.push(b);
    }

    ans.sort_unstable();
    ans.join(",")
}

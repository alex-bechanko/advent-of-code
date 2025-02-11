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

use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
enum Combo {
    Literal(u64),
    RegisterA,
    RegisterB,
    RegisterC,
}

impl Combo {
    fn into_value(self, register: &Register<u64>) -> u64 {
        match self {
            Combo::Literal(x) => x,
            Combo::RegisterA => register.a,
            Combo::RegisterB => register.b,
            Combo::RegisterC => register.c,
        }
    }
}

impl TryFrom<u64> for Combo {
    type Error = ProgramError;
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Combo::Literal(0)),
            1 => Ok(Combo::Literal(1)),
            2 => Ok(Combo::Literal(2)),
            3 => Ok(Combo::Literal(3)),
            4 => Ok(Combo::RegisterA),
            5 => Ok(Combo::RegisterB),
            6 => Ok(Combo::RegisterC),
            _ => Err(ProgramError::InvalidComboOperand),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Adv(Combo),
    Bxl(u64),
    Bst(Combo),
    Jnz(usize),
    Bxc,
    Out(Combo),
    Bdv(Combo),
    Cdv(Combo),
}
use Instruction::{Adv, Bdv, Bst, Bxc, Bxl, Cdv, Jnz, Out};

impl TryFrom<(u64, u64)> for Instruction {
    type Error = ProgramError;
    fn try_from((instr, operand): (u64, u64)) -> Result<Self, Self::Error> {
        match instr {
            0 => operand.try_into().map(Adv),
            1 => Ok(Bxl(operand)),
            2 => operand.try_into().map(Bst),
            3 => Ok(Jnz(
                usize::try_from(operand).map_err(|_| ProgramError::ParseError)?
            )),
            4 => Ok(Bxc),
            5 => operand.try_into().map(Out),
            6 => operand.try_into().map(Bdv),
            7 => operand.try_into().map(Cdv),
            _ => Err(ProgramError::InvalidInstruction),
        }
    }
}

#[derive(Debug)]
enum ProgramError {
    InvalidInstruction,
    InvalidComboOperand,
    OddInstructionCout,
    ParseError,
}

#[derive(Debug)]
struct Register<T> {
    a: T,
    b: T,
    c: T,
}

struct Program(Vec<Instruction>);

impl TryFrom<Vec<u64>> for Program {
    type Error = ProgramError;
    fn try_from(value: Vec<u64>) -> Result<Self, Self::Error> {
        if value.len() % 2 == 1 {
            return Err(ProgramError::OddInstructionCout);
        }

        let instructions = (0..value.len() / 2)
            .map(|x| Instruction::try_from((value[2 * x], value[2 * x + 1])))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Program(instructions))
    }
}

fn parse(input: &str) -> Result<(Register<u64>, Vec<u64>), ProgramError> {
    let (register_a, rest) = input.split_once('\n').ok_or(ProgramError::ParseError)?;
    let a: u64 = register_a
        .strip_prefix("Register A: ")
        .and_then(|x| x.parse().ok())
        .ok_or(ProgramError::ParseError)?;

    let (register_b, rest) = rest.split_once('\n').ok_or(ProgramError::ParseError)?;
    let b: u64 = register_b
        .strip_prefix("Register B: ")
        .and_then(|x| x.parse().ok())
        .ok_or(ProgramError::ParseError)?;

    let (register_c, rest) = rest.split_once("\n\n").ok_or(ProgramError::ParseError)?;
    let c: u64 = register_c
        .strip_prefix("Register C: ")
        .and_then(|x| x.parse().ok())
        .ok_or(ProgramError::ParseError)?;

    let register = Register { a, b, c };

    let instructions = rest
        .trim()
        .strip_prefix("Program: ")
        .ok_or(ProgramError::ParseError)?;
    let instructions: Vec<u64> = instructions
        .split(',')
        .map(|r| r.parse().ok())
        .collect::<Option<Vec<_>>>()
        .ok_or(ProgramError::ParseError)?;

    Ok((register, instructions))
}

fn run(mut register: Register<u64>, instructions: &[Instruction]) -> Vec<u64> {
    let mut iptr = 0usize;
    let mut out = vec![];
    while let Some(&instr) = instructions.get(iptr) {
        match instr {
            Adv(c) => register.a >>= u32::try_from(c.into_value(&register)).unwrap(),
            Bxl(x) => register.b ^= x,
            Bst(c) => register.b = c.into_value(&register) % 8,
            Jnz(_) => {} // handled later
            Bxc => register.b ^= register.c,
            Out(c) => out.push(c.into_value(&register) % 8),
            Bdv(c) => register.b = register.a >> u32::try_from(c.into_value(&register)).unwrap(),
            Cdv(c) => register.c = register.a >> u32::try_from(c.into_value(&register)).unwrap(),
        }

        iptr += 1;

        if let (Jnz(x), true) = (instr, register.a != 0) {
            iptr = x;
        }
    }

    out
}

/// # Panics
pub fn part1(input: &str) -> String {
    let (registers, instructions) = parse(input).expect("Failed to parse input");
    let Program(instructions) = instructions
        .try_into()
        .expect("Invalid instructions provided");
    run(registers, &instructions)
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// # Panics
pub fn part2(input: &str) -> u64 {
    let (_, instructions) = parse(input).expect("Failed to parse input");
    let Program(program) = instructions
        .clone()
        .try_into()
        .expect("Failed to compile instructions");

    let mut stack = vec![(instructions.len() - 1, 0)];
    let mut min_solution = None;
    while let Some((out_index, init_value)) = stack.pop() {
        for i in 0..8 {
            let value = (init_value << 3) + i;
            let r = Register {
                a: value,
                b: 0,
                c: 0,
            };

            let out = run(r, &program);

            let matches = (out_index..instructions.len())
                .enumerate()
                .all(|(a, b)| out.get(a) == instructions.get(b));
            if matches {
                if out_index == 0 {
                    min_solution = min_solution.or(Some(value)).map(|m| m.min(value));
                } else {
                    stack.push((out_index - 1, value));
                }
            }
        }
    }

    min_solution.unwrap()
}

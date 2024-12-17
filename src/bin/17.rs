use core::{fmt, panic};
use std::{fmt::Debug, u64};

use itertools::Itertools;
use num_traits::pow;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

advent_of_code::solution!(17);

#[derive(PartialEq, Eq, Clone, Debug)]
enum Instructions {
    ADV(u8),
    BXL(u8),
    BST(u8),
    JNZ(usize),
    BXC(u8),
    OUT(u8),
    BDV(u8),
    CDV(u8),
}

fn parse_digits_from_string(input: &str) -> u32 {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u32>()
        .expect("to be good integer")
}

fn parse_instructions(input: &str) -> (Vec<u64>, Vec<Instructions>) {
    let input_numbers = input.replace("Program: ", "");
    let mut input_string = vec![];
    let out = input_numbers
        .split(",")
        .tuples()
        .map(|(opcode, operand)| {
            let operand_num = operand.parse::<u8>().unwrap();
            input_string.push(opcode.parse::<u64>().unwrap());
            input_string.push(operand_num as u64);
            match opcode {
                "0" => Instructions::ADV(operand_num),
                "1" => Instructions::BXL(operand_num),
                "2" => Instructions::BST(operand_num),
                "3" => Instructions::JNZ(operand_num as usize),
                "4" => Instructions::BXC(operand_num),
                "5" => Instructions::OUT(operand_num),
                "6" => Instructions::BDV(operand_num),
                "7" => Instructions::CDV(operand_num),
                _ => panic!("Invalid instructions!!: {},{}", opcode, operand_num),
            }
        })
        .collect_vec();
    (input_string, out)
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines = input.lines();
    let reg_a = parse_digits_from_string(lines.next().unwrap());

    let (_, instructions) = parse_instructions(lines.nth(3)?);
    Some(run(&instructions, reg_a as u64).iter().join(","))
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();

    let (ins, instructions) = parse_instructions(lines.nth(4)?);
    let final_reg_a = (0u64..=u64::MAX).into_par_iter().find_first(|&i| {
        let out: Vec<u64> = run(&instructions, i);
        out == ins
    });

    final_reg_a
}

fn run(ins: &Vec<Instructions>, reg_a_orig: u64) -> Vec<u64> {
    let mut reg_a = reg_a_orig;
    let mut reg_b = 0u64;
    let mut reg_c = 0u64;
    let mut i = 0usize;
    let mut out = vec![];

    fn map_combo_operand(op: u8, reg_a: u64, reg_b: u64, reg_c: u64) -> u64 {
        match op {
            0..=3 => op as u64,
            4 => reg_a,
            5 => reg_b,
            6 => reg_c,
            _ => panic!("Invalid combo operand, {}", op),
        }
    };

    while i < ins.len() {
        let current_ins = &ins[i];
        i += 1;
        match current_ins {
            Instructions::ADV(o) => {
                reg_a /= pow(2, map_combo_operand(*o, reg_a, reg_b, reg_c) as usize) as u64
            }
            Instructions::BXL(o) => {
                reg_b ^= *o as u64;
            }
            Instructions::BST(o) => {
                reg_b = map_combo_operand(*o, reg_a, reg_b, reg_c) % 8;
            }
            Instructions::JNZ(o) => {
                if reg_a != 0 {
                    i = *o;
                }
            }
            Instructions::BXC(_) => {
                reg_b ^= reg_c;
            }
            Instructions::OUT(o) => {
                out.push(map_combo_operand(*o, reg_a, reg_b, reg_c) % 8);
            }
            Instructions::BDV(o) => {
                reg_b = reg_a / pow(2, map_combo_operand(*o, reg_a, reg_b, reg_c) as usize) as u64
            }
            Instructions::CDV(o) => {
                reg_c = reg_a / pow(2, map_combo_operand(*o, reg_a, reg_b, reg_c) as usize) as u64
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("5,7,3,0".to_string()));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(117440));
    }
}

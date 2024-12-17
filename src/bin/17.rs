use core::panic;
use std::{fmt::Debug, u64};

use itertools::Itertools;
use num_traits::pow;

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

fn parse_digits_from_string(input: &str) -> u64 {
    input
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u64>()
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
    let (expected_output, instructions) = parse_instructions(lines.nth(4)?);

    let mut result = u64::MAX;

    let mut inputs = (0..8).collect::<Vec<_>>();
    let output_size = instructions.len() * 2;

    // Based on https://topaz.github.io/paste/#XQAAAQAYEwAAAAAAAAA6nMjJFD6Qz6l42wPnwN2pT342vOnq8+8016BvPrIGSiW1PbAILA7IGl3spWYQSB50taWynBnclF9Q0Cy6bn1sRjPWUduirAXRlTyXW5R75vEEmAknHp6Rdoqiya/Y+55alFmuozcsg40WMEtt6QyduuNZk/SEUsFQTAG+JL/YbjDr5TnKUOdPxSUvL9T+/NuXxaBCcQkPFO+zKZcF4Z8zQ9G9wofYJTpWf/rMjGzHnbjB2wE2Sy+YkBBvOK+zohisHs8kOU9A7zrWv4xh9bK7W6CS2Utdqu5MBegh264VgQSk2OAf1vy9rR8wwoGj01kA5T7C6Hhq4Fcte12xyn6+x0Q41y2nV2+cRDihKWYjtAjk4r1TJBoVTwa5ho88sSBxmGDPyPxk0Wl6eZbrG6D/dSv8+yB3kmFX9HjRkzCAjSPXn8juXL/qYA+UI82PMTA/pWFUD1geKzIdWnGe73EcPwStP5n3tpx590Jn5/J277XN5NcPpMImQth72yyLUwdg3+TLsqFY1+NEahY/kGj5pPraXQaG5vf7CoS1Tnrgzcfzd1wZiNzg3sGjKq0MhweCAC74piRrXsAoNZVkQlPM2GJ4rhxffX9fQwAYXrb+FJ8PKhaS4DO5J3bwWMldY/tEcMT013F4IetifjCxMgPNyBtxhGEUkxwDV4caYD92nwakBzuEnBT3g4ua1oB8AHI0zBdcpcRVtH/aNnarU9JTXXsauLFAx8yxbhKvaLqHo76yZQpfumiCvfYRw9C6DbKOJorNvjuHt7n2btuNOqV/lps24UiosHTbiiQ4ZaV1U9qvSPpVmoIe0pC3gErG71FqqJrL0ZFJvM7/EOdSR/IXLmutAYhAfHo0EDU40GqoLQyFasQ3TAW7dXsHKcmSUbMbeN3n48y9JnleJwnPmZKukDyMByzy3wd/0gQu1W/PZe+h82UH3P/AYKYBY/V2Kj9kYfWE0DWioW8yC7TXLyNVZ6i+rgyKU3IdvIl/My1E7M6Vea1H+hnC9T+2EfqY2vz/LhFLC3M48nqpmV7uZVvuQ1dBVZnc8H6w2MDwewQCarGGFKDxeY5mJi/Foeilr1kC24QRacls6pQenENIlNa9ef+VNRzRlt4EeZlJeesOQov2A67j9Y6hWRmT53kjdSxoaHdwEG+qy7D8fOrNLD/mGsoRzQW6BaAepOFpT5WTOOHheDiJO98jD5wtGpL7+IaTSYhv5O+MohxVhq4G8X9ID2GpmWeaktXy5aI9ZRXFNIG9IcduY3eP/1OiaabOdlAo89flZoSekoWGDYWUzjyDvXkD5Geq2jIUwEgJp6Ml8gXCZ/7C0mMadcmwvGLYivR9mV6sP5vUS42RaE/Dq4vf1vECBRVBBleFhoQMJtHTtCn0nyu+nRvxNzQa//Gd3xNB0j7Hm1714SRELQSirkDZ6WQwh43gIaSECLU/uNpA1dRgs+OG3n0z0GJvNFzd+PpSzYcuK+ik7qoiOrIRhCM5D/4CP+GwiF5TQP8qquveo5PIWAl8zDA2IvWJ83NdZhbJ9Oj67HpMa/Mak5b/emgY8RrTpNKL8LTb91oGwuDP+aURTEuvhUoKARdxNemzsHMHbWyOp+uuRUirlLcbjSX+tguiUUNt0n8AIQKROntfYoTY10t0QGk9igBCg6rNK7GquqBnGZ+/XualhpIAmTKoJ49fRwZHm/GZRvfSgwy96mYUdStFaIDAyEHLZrf39eYwr+geX7rgDQdjxy5K7aYHZhbyDxzcwhOmGXTSeW3iS9sNTHSNjN/Ym4jfG6rxNyDln14Az0eqmdOE6O3+CDXeyi8I2UqpB4N6wm2ncoarlsk7sRA4w2EEazd/Yn4bBU4dVXw5ySrABRadPM+H2+XmLFv38qwj5yUnTDBqYwgUvOSWqQRVPNPxr88askElfz1GHm3ZZl6qD+UihW3BIyeCKuxDTBOt2p/3esHbwGo3wXByizvt8eOta13A1UwaWe60SYoSXCXnh+emUOAC4w6SYcFFi1z/+CYQtg==
    for j in 0..output_size {
        let mut next = vec![];

        // check each input to see if it generates the correct output
        for num in &inputs {
            // reuse device by resetting internal values and check for current number
            let out = run(&instructions, *num);

            // if the generated output matches the suboutput so far
            if expected_output[output_size - 1 - j..] == out {
                // final output length reached, get minimum viable value
                if out.len() == output_size {
                    result = std::cmp::min(result, *num);
                }
                // generate all other numbers that result in the same output because of integer division
                for k in 0..=8 {
                    if (num * 8 + k) / 8 == *num {
                        next.push(num * 8 + k);
                    }
                }
            }
        }

        // prepare next run with all generated numbers
        inputs = next;
    }
    Some(result)
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

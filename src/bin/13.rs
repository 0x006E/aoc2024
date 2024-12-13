advent_of_code::solution!(13);

use itertools::Itertools;
use regex::Regex;

lazy_static::lazy_static! {
    static ref RE_POINTS: Regex = Regex::new(r"Button [A|B]: X\+(\d+), Y\+(\d+)").unwrap();
    static ref RE_PRICE: Regex = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
}

fn parse_inputs(inputs: &[&str]) -> Vec<i32> {
    inputs
        .iter()
        .map(|&input| input.trim().parse::<i32>().expect("Invalid number format"))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split("\n\n")
            .map(|test| {
                let ins = test.split("\n").collect_vec();
                let [a1_str, a2_str]: [&str; 2] = RE_POINTS
                    .captures(ins[0])
                    .expect("Should be a valid point line")
                    .extract()
                    .1;
                let [b1_str, b2_str] = RE_POINTS
                    .captures(ins[1])
                    .expect("Should be a valid point line")
                    .extract()
                    .1;
                let [c1_str, c2_str] = RE_PRICE
                    .captures(ins[2])
                    .expect("Should be a valid price line")
                    .extract()
                    .1;

                if let [a1, a2, b1, b2, c1, c2] =
                    parse_inputs(&[a1_str, a2_str, b1_str, b2_str, c1_str, c2_str])[..]
                {
                    let det = a1 * b2 - a2 * b1;
                    let det_x = c1 * b2 - b1 * c2;
                    let det_y = a1 * c2 - a2 * c1;
                    let x = det_x / det;
                    let y = det_y / det;

                    if (x * a1 + b1 * y) == c1 && (x * a2 + b2 * y) == c2 {
                        return x * 3 + y;
                    }
                }
                0
            })
            .sum::<i32>() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u128> {
    Some(
        input
            .split("\n\n")
            .map(|test| {
                let ins = test.split("\n").collect_vec();
                let [a1_str, a2_str]: [&str; 2] = RE_POINTS
                    .captures(ins[0])
                    .expect("Should be a valid point line")
                    .extract()
                    .1;
                let [b1_str, b2_str] = RE_POINTS
                    .captures(ins[1])
                    .expect("Should be a valid point line")
                    .extract()
                    .1;
                let [c1_str, c2_str] = RE_PRICE
                    .captures(ins[2])
                    .expect("Should be a valid price line")
                    .extract()
                    .1;

                if let [a1, a2, b1, b2, c1, c2] =
                    parse_inputs(&[a1_str, a2_str, b1_str, b2_str, c1_str, c2_str])
                        .iter()
                        .map(|e| *e as i128)
                        .collect_vec()[..]
                {
                    let c1 = 10_000_000_000_000 + c1;
                    let c2 = 10_000_000_000_000 + c2;
                    let det = a1 * b2 - a2 * b1;
                    let det_x = c1 * b2 - b1 * c2;
                    let det_y = a1 * c2 - a2 * c1;
                    let x = det_x / det;
                    let y = det_y / det;

                    if (x * a1 + b1 * y) == c1 && (x * a2 + b2 * y) == c2 {
                        return x * 3 + y;
                    }
                }
                0
            })
            .sum::<i128>() as u128,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(u128, Vec<u128>)> {
    let mut v = vec![];
    for line in input.lines() {
        let mut values = line
            .split_terminator(&[':', ' '][..])
            .filter(|v| !v.trim().is_empty())
            .map(|v| v.parse::<u128>().expect("Numbers should be parseable"))
            .collect::<Vec<u128>>();
        v.push((values.remove(0), values));
    }
    v
}

fn concat(a: u128, b: u128) -> u128 {
    a * 10u128.pow(b.ilog10() + 1) + b
}

fn do_calculation(
    nums: &[u128],
    index: usize,
    current_result: u128,
    need_concat_operator: bool,
    target: u128,
) -> bool {
    if index == nums.len() - 1 {
        return current_result == target;
    }
    let add_result = do_calculation(
        nums,
        index + 1,
        current_result + nums[index + 1],
        need_concat_operator,
        target,
    );
    let multiply_result = do_calculation(
        nums,
        index + 1,
        current_result * nums[index + 1],
        need_concat_operator,
        target,
    );
    let mut final_result = add_result || multiply_result;
    if need_concat_operator {
        final_result = final_result
            || do_calculation(
                nums,
                index + 1,
                concat(current_result, nums[index + 1]),
                need_concat_operator,
                target,
            );
    }
    final_result
}

pub fn part_one(input: &str) -> Option<u128> {
    let values = parse_input(input);
    let sum = values
        .par_iter()
        .filter(|(target, v)| do_calculation(v, 0, v[0], false, *target))
        .map(|v| v.0)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u128> {
    let values = parse_input(input);
    let sum = values
        .par_iter()
        .filter(|(target, v)| do_calculation(v, 0, v[0], true, *target))
        .map(|v| v.0)
        .sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

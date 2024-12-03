use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::{Fancy, Sorted, SpaceSeperated};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u64>, Vec<u64>) {
    input.space_seperated().collect::<Fancy<_>>().reveal()
}

#[aoc(day1, part1)]
pub fn solve_part1((xs, ys): &(Vec<u64>, Vec<u64>)) -> u64 {
    xs.sorted_unstable()
        .iter()
        .zip(&ys.sorted_unstable())
        .map(|(x, y)| x.abs_diff(*y))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2((xs, ys): &(Vec<u64>, Vec<u64>)) -> u64 {
    let factors = ys
        .sorted_unstable()
        .chunk_by(|a, b| a == b)
        .map(|chunk| (chunk[0], chunk.len() as u64))
        .collect::<HashMap<_, _>>();

    xs.iter().map(|x| *x * factors.get(x).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    const INPUT1: &str = include_str!("../input/2024/day1_example1.txt");

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 11);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT1)), 31);
    }
}

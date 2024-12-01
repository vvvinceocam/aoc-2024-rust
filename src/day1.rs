use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u64>, Vec<u64>) {
    let hint = input.len() / 7;
    let mut xs = Vec::with_capacity(hint);
    let mut ys = Vec::with_capacity(hint);

    let mut numbers = input
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().unwrap());

    while let (Some(x), Some(y)) = (numbers.next(), numbers.next()) {
        xs.push(x);
        ys.push(y);
    }

    (xs, ys)
}

#[aoc(day1, part1)]
pub fn solve_part1((xs, ys): &(Vec<u64>, Vec<u64>)) -> u64 {
    let mut xs = xs.clone();
    let mut ys = ys.clone();

    xs.sort_unstable();
    ys.sort_unstable();

    xs.iter().zip(ys.iter()).map(|(x, y)| x.abs_diff(*y)).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2((xs, ys): &(Vec<u64>, Vec<u64>)) -> u64 {
    let mut ys = ys.clone();
    ys.sort_unstable();

    let factors = ys
        .chunk_by(|a, b| a == b)
        .map(|chunk| (chunk[0], chunk.len() as u64))
        .collect::<HashMap<_, _>>();

    xs.iter().map(|x| *x * factors.get(x).unwrap_or(&0)).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator, solve_part1, solve_part2};

    static INPUT1: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 11);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT1)), 31);
    }
}

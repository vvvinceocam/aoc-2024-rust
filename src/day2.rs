use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};

use crate::utils::space_seperated_to_vec;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Vec<i64>> {
    input.lines().map(space_seperated_to_vec).collect()
}

fn is_safe(report: &[i64]) -> bool {
    use Ordering::*;
    report
        .windows(2)
        .map(|ab| ab[0] - ab[1])
        .try_fold(Equal, |order, diff| {
            if (-3..=-1).contains(&diff) && order != Less {
                Some(Greater)
            } else if (1..=3).contains(&diff) && order != Greater {
                Some(Less)
            } else {
                None
            }
        })
        .is_some()
}

#[aoc(day2, part1)]
fn solve_part1(reports: &[Vec<i64>]) -> usize {
    reports.iter().filter(|report| is_safe(report)).count()
}

#[aoc(day2, part2)]
fn solve_part2(reports: &[Vec<i64>]) -> usize {
    reports
        .iter()
        .filter(|report| {
            (0..report.len())
                .map(|i| {
                    let mut report = report.to_vec();
                    report.remove(i);
                    report
                })
                .any(|report| is_safe(&report))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("../input/2024/day2_example1.txt");

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(&input_generator(INPUT1)), 2);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(&input_generator(INPUT1)), 4);
    }
}

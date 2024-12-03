use aoc_runner_derive::aoc;

use crate::utils::tails;

fn parse_mul(input: &str) -> usize {
    let end = input[..].find(')').unwrap();
    let Some((a, b)) = (input[4..end]).split_once(',') else {
        return 0;
    };
    let (Ok(a), Ok(b)) = (str::parse::<usize>(a), str::parse::<usize>(b)) else {
        return 0;
    };
    a * b
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    tails(input).fold(0, |sum, tail| {
        if tail.starts_with("mul(") {
            sum + parse_mul(tail)
        } else {
            sum
        }
    })
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    tails(input)
        .fold((true, 0), |(enabled, sum), tail| {
            if tail.starts_with("mul(") && enabled {
                (enabled, sum + parse_mul(tail))
            } else if tail.starts_with("do()") {
                (true, sum)
            } else if tail.starts_with("don't()") {
                (false, sum)
            } else {
                (enabled, sum)
            }
        })
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = include_str!("../input/2024/day3_example1.txt");
    const INPUT2: &str = include_str!("../input/2024/day3_example2.txt");

    #[test]
    fn solver_part1_match_example() {
        assert_eq!(solve_part1(INPUT1), 161);
    }

    #[test]
    fn solver_part2_match_example() {
        assert_eq!(solve_part2(INPUT2), 48);
    }
}

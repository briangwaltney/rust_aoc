
use std::ops::RangeInclusive;

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn part_one(input: &str) -> usize {
    let lines: Vec<Vec<Vec<i32>>> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split('-')
                        .map(|limit| limit.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    // println!("{:?}", lines);

    let contain_count = lines
        .iter()
        .map(|line| match line.as_slice() {
            [first, second] => {
                let first = first.as_slice();
                let second = second.as_slice();
                let first_contains = first[0] <= second[0] && first[1] >= second[1];
                let second_contains = second[0] <= first[0] && second[1] >= first[1];
                first_contains || second_contains
            }
            _ => false,
        })
        .filter(|x| x == &true)
        .count();

    contain_count
}

fn part_two(input: &str) -> usize {
    let lines: Vec<Vec<Vec<i32>>> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split('-')
                        .map(|limit| limit.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    // println!("{:?}", lines);

    let contain_count = lines
        .iter()
        .map(|line| match line.as_slice() {
            [first, second] => {
                let first = first.as_slice();
                let second = second.as_slice();
                let first_contains = (first[0] <= second[0] && first[1] >= second[0])
                    || (first[1] >= second[1] && first[0] <= second[1]);
                let second_contains = (second[0] <= first[0] && second[1] >= first[0])
                    || (second[1] >= first[1] && second[0] <= first[1]);

                first_contains || second_contains
            }
            _ => false,
        })
        .filter(|x| x == &true)
        .count();

    contain_count
}

fn parse_numbers(input: &str) -> IResult<&str, RangeInclusive<u32>> {
    let (input, start) = complete::u32(input)?;
    let (input, _) = complete::char('-')(input)?;
    let (input, end) = complete::u32(input)?;

    Ok((input, start..=end))
}

fn parse_line(input: &str) -> IResult<&str, (RangeInclusive<u32>, RangeInclusive<u32>)> {
    let (input, first) = parse_numbers(input)?;
    let (input, _) = complete::char(',')(input)?;
    let (input, second) = parse_numbers(input)?;

    Ok((input, (first, second)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<(RangeInclusive<u32>, RangeInclusive<u32>)>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn part_one_better(input: &str) -> usize {
    let (_, lines) = parse_input(input).unwrap();

    let contain_count = lines
        .iter()
        .filter(|(a, b)| {
            a.clone().into_iter().all(|num| b.contains(&num))
                || b.clone().into_iter().all(|num| a.contains(&num))
        })
        .count();

    contain_count
}

fn part_two_better(input: &str) -> usize {
    let (_, lines) = parse_input(input).unwrap();

    let contain_count = lines
        .iter()
        .filter(|(a, b)| {
            a.clone().into_iter().any(|num| b.contains(&num))
                || b.clone().into_iter().any(|num| a.contains(&num))
        })
        .count();

    contain_count
}
pub fn input() {
    let input = include_str!("../inputs/2022/2022_day_4.txt");

    println!("Part one: {}", part_one_better(input));
    println!("Part two: {}", part_two_better(input));
}

//tests

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]

    fn test_part_one() {
        assert_eq!(part_one_better(TEST_INPUT), 2);
    }

    #[test]

    fn test_part_two() {
        assert_eq!(part_two_better(TEST_INPUT), 4);
    }
}

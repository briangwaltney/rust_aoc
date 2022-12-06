#![allow(unused)]

use nom::{character::complete, IResult};

fn part_one(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    //split each line into a vector of integers
    let mut lines_int: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        let mut line_int: Vec<i32> = Vec::new();
        for c in line.chars() {
            line_int.push(c.to_digit(10).unwrap() as i32);
        }
        lines_int.push(line_int);
    }

    let length = lines_int[0].len();

    let mut i = 0;
    let mut most_common: Vec<i32> = Vec::new();
    let mut least_common: Vec<i32> = Vec::new();

    while i < length {
        let mut chars: Vec<i32> = Vec::new();
        for line in &lines_int {
            chars.push(line[i]);
        }

        let ones = chars.iter().filter(|&x| *x == 1).count();
        let zeros = chars.iter().filter(|&x| *x == 0).count();

        match ones > zeros {
            true => {
                most_common.push(1);
                least_common.push(0);
            }
            false => {
                most_common.push(0);
                least_common.push(1);
            }
        }

        i += 1;
    }

    most_common.reverse();
    least_common.reverse();

    let mut most_binary = 0;
    let mut least_binary = 0;

    for (i, num) in most_common.iter().enumerate() {
        most_binary += num * 2_i32.pow(i as u32);
    }

    for (i, num) in least_common.iter().enumerate() {
        least_binary += num * 2_i32.pow(i as u32);
    }

    most_binary * least_binary
}

fn find_most_common(input: &Vec<&str>, position: usize) -> char {
    let mut chars: Vec<char> = Vec::new();
    for line in input {
        chars.push(line.chars().nth(position).unwrap());
    }

    let ones = chars.iter().filter(|&x| *x == '1').count();
    let zeros = chars.iter().filter(|&x| *x == '0').count();

    // this greater than or equal was the key. I was using > and it was failing
    match ones >= zeros {
        true => '1',
        false => '0',
    }
}

fn filter_list(input: Vec<&str>, position: usize, value: char) -> Vec<&str> {
    input
        .into_iter()
        .filter(|&x| x.chars().nth(position).unwrap() == value)
        .collect()
}

fn part_two(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut oxygen = lines.clone();
    let mut co2 = lines.clone();

    //split each line into a vector of integers
    let length = lines[0].len();

    let mut i = 0;
    while oxygen.len() > 2 {
        let most_common = find_most_common(&oxygen, i);
        oxygen = filter_list(oxygen, i, most_common);
        i += 1;
    }

    oxygen = filter_list(oxygen, i, '1');
    let oxy_string = oxygen[0].to_string();

    i = 0;
    while co2.len() > 2 {
        let most_common = find_most_common(&co2, i);
        let mut least_common: char;

        match most_common {
            '1' => least_common = '0',
            '0' => least_common = '1',
            _ => panic!("Invalid character"),
        }

        co2 = filter_list(co2, i, least_common);
        i += 1;
    }

    co2 = filter_list(co2, 2, '0');

    let co2_string = co2[0].to_string();

    convert_to_decimal(&oxy_string) * convert_to_decimal(&co2_string)
}

// convert a string of 1s and 0s to a decimal number

fn convert_to_decimal(input: &str) -> i32 {
    let mut decimal = 0;
    for (i, num) in input.chars().rev().enumerate() {
        decimal += num.to_digit(10).unwrap() as i32 * 2_i32.pow(i as u32);
    }
    decimal
}

pub fn input() {
    let input = include_str!("../inputs/2021/2021_day_3.txt");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}

// tests
const TEST_INPUT: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 198);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 230);
    }
}

#![allow(unused)]

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

fn part_two(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

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

    0
}

fn main() {
    let input = include_str!("../inputs/2021/2021_day_3.txt");

    // println!("Part one: {}", part_one(&input));
    // println!("Part two: {}", part_two(&input));
}

// tests

#[cfg(test)]

mod tests {
    use super::*;
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

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&TEST_INPUT), 198);
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        assert_eq!(part_two(&TEST_INPUT), 230);
    }
}

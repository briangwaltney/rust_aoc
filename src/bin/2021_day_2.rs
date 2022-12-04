#![allow(unused)]

// This one was much easier for me. Didn't have to look anything up, but copilot did help me with the parsing.

fn part_one(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut horizontal = 0;
    let mut vertical = 0;

    for line in lines {
        let mut words: Vec<&str> = line.split_whitespace().collect();
        let quat = words[1].parse::<i32>().unwrap();
        match words[0] {
            "forward" => horizontal += quat,
            "up" => vertical += quat,
            "down" => vertical -= quat,
            _ => println!("Invalid input"),
        }
    }
    horizontal * vertical.abs()
}

fn part_two(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();

    let mut horizontal = 0;
    let mut vertical = 0;
    let mut aim = 0;

    for line in lines {
        let mut words: Vec<&str> = line.split_whitespace().collect();
        let quat = words[1].parse::<i32>().unwrap();
        match words[0] {
            "forward" => {
                horizontal += quat;
                vertical += aim * quat;
            }
            "up" => aim -= quat,
            "down" => aim += quat,
            _ => println!("Invalid input"),
        }
    }
    (horizontal * vertical).abs()
}
fn main() {
    let input = include_str!("../inputs/2021/2021_day_2.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

// tests

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_INPUT: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]

    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 150);
    }

    #[test]

    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 900);
    }
}

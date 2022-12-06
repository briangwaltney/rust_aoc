#![allow(unused_variables, dead_code, unused_imports)]

use std::collections::HashSet;

use nom::{
    character::complete::{self, newline},
    multi::{many_till, separated_list1},
    IResult,
};

pub fn input() {
    let input = include_str!("../inputs/2022/2022_day_6.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    // split input into individual chars
    let chars: Vec<char> = input.chars().collect();

    let mut index = 0;
    let mut dup = false;

    while dup == false && index < chars.len() + 3 {
        let  test_set = &chars[index..=(index + 3)];
        let  uniques: HashSet<char> = HashSet::from_iter(test_set.iter().cloned());

        if uniques.len() == 4 {
            dup = true;
        } else {
            index += 1;
        }
    }

    index + 4
}

fn part_two(input: &str) -> usize {
    // split input into individual chars
    let chars: Vec<char> = input.chars().collect();

    let mut index = 0;
    let mut dup = false;

    while dup == false && index < chars.len() + 13 {
        let  test_set = &chars[index..=(index + 13)];
        let  uniques: HashSet<char> = HashSet::from_iter(test_set.iter().cloned());

        if uniques.len() == 14 {
            dup = true;
        } else {
            index += 1;
        }
    }

    index + 14
}

//tests

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_ONE: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const TEST_TWO: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const TEST_THREE: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const TEST_FOUR: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const TEST_FIVE: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]

    fn test_part_one() {
        assert_eq!(part_one(TEST_ONE), 7);
        assert_eq!(part_one(TEST_TWO), 5);
        assert_eq!(part_one(TEST_THREE), 6);
        assert_eq!(part_one(TEST_FOUR), 10);
        assert_eq!(part_one(TEST_FIVE), 11);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_ONE), 19);
        assert_eq!(part_two(TEST_TWO), 23);
        assert_eq!(part_two(TEST_THREE), 23);
        assert_eq!(part_two(TEST_FOUR), 29);
        assert_eq!(part_two(TEST_FIVE), 26);
    }
}

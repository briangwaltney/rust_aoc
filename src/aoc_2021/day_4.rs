// allow unused 
#![allow(unused, dead_code, unused_imports, unused_variables, unused_mut)]

use std::ops::RangeInclusive;

use nom::{
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};


fn parse_board(input: &str) -> IResult<&str, Vec<u32>> {
  // let lines = input.lines().collect::<Vec<&str>>();
  //   let (input, first) = complete::u32(lines)?;
  //   println!("first: {:?}", first);

    Ok((input, vec![0]))
}


fn part_one_better(input: &str) -> usize {
    let mut boards = input.split("\n\n").collect::<Vec<&str>>();
    let calls = boards
        .remove(0)
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();


    println!("{:?}", calls);
    println!("{:?}", boards);
    println!("{:?}", parse_board(boards[0]));

    0
}

fn part_two_better(_input: &str) -> usize {
    0
}

pub fn input() {
    let input = include_str!("../inputs/2021/2021_day_4.txt");

    println!("Part one: {}", part_one_better(input));
    println!("Part two: {}", part_two_better(input));
}

//tests

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    #[test]
    #[ignore]

    fn test_part_one() {
        assert_eq!(part_one_better(TEST_INPUT), 4512);
    }

    #[test]
    #[ignore]

    fn test_part_two() {
        assert_eq!(part_two_better(TEST_INPUT), 4);
    }
}

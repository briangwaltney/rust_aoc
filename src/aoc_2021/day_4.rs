// allow unused
#![allow(unused, dead_code, unused_imports, unused_variables, unused_mut)]

use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

fn is_bingo(board: &Vec<Vec<u32>>) -> bool {
    let mut bingo = false;

    // check rows
    for row in board {
        if row.iter().all(|x| *x == 0) {
            bingo = true;
        }
    }

    // check columns
    for i in 0..5 {
        if board.iter().all(|x| x[i] == 0) {
            bingo = true;
        }
    }

    bingo
}

fn part_one_better(input: &str) -> u32 {
    let mut boards = input.split("\n\n").collect::<Vec<&str>>();
    let calls = boards
        .remove(0)
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // split each board at line breaks and then into vectors of integers
    let mut boards = boards
        .iter()
        .map(|x| {
            x.split("\n")
                .map(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect::<Vec<Vec<Vec<u32>>>>();

    let mut solve: Vec<Vec<u32>> = Vec::new();
    let mut solved: bool = false;
    let mut last_call: u32 = 0;

    for call in calls {
        if solved {
            break;
        }
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for num in row.iter_mut() {
                    if *num == call {
                        *num = 0;
                    }
                }
            }
            if is_bingo(board) {
                solve = board.clone();
                solved = true;
                last_call = call;
                break;
            }
        }
    }

    let remaining_sum = solve.iter().map(|x| x.iter().sum::<u32>()).sum::<u32>();

    remaining_sum * last_call
}

fn part_two_better(input: &str) -> u32 {
    let mut boards = input.split("\n\n").collect::<Vec<&str>>();
    let calls = boards
        .remove(0)
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // split each board at line breaks and then into vectors of integers
    let mut boards = boards
        .iter()
        .map(|x| {
            x.split("\n")
                .map(|x| {
                    x.split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>()
                })
                .collect::<Vec<Vec<u32>>>()
        })
        .collect::<Vec<Vec<Vec<u32>>>>();

    let mut solve: Vec<Vec<u32>> = Vec::new();
    let mut solved = 0;
    let mut last_call: u32 = 0;
    let length = boards.len();

    for call in calls {
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for num in row.iter_mut() {
                    if *num == call {
                        *num = 0;
                    }
                }
            }

            // test all boards for bingo and get length of unsolved boards
        }
        let unsolved = &boards
            .iter()
            .filter(|x| !is_bingo(x))
            .collect::<Vec<&Vec<Vec<u32>>>>();

        if unsolved.len() == 1 {
            solve = unsolved[0].clone();
        }

        if unsolved.len() == 0 {
            last_call = call;
            break;
        }
    }

    //print last call


    let remaining_sum = solve.iter().map(|x| x.iter().sum::<u32>()).sum::<u32>()
    - last_call;

    remaining_sum * last_call
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

    fn test_part_one() {
        assert_eq!(part_one_better(TEST_INPUT), 4512);
    }

    #[test]

    fn test_part_two() {
        assert_eq!(part_two_better(TEST_INPUT), 1924);
    }
}

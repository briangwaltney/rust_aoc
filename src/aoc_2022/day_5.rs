#![allow(unused_variables, dead_code, unused_imports)]

use nom::{
    character::complete::{self, newline},
    multi::{many_till, separated_list1},
    IResult,
};

pub fn input() {
    let start = include_str!("../inputs/2022/2022_day_5_1.txt");
    let input = include_str!("../inputs/2022/2022_day_5_2.txt");

    println!("Part one: {}", part_one(start, input));
    println!("Part two: {}", part_two(start, input));
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, count) = many_till(complete::anychar, complete::u32)(input)?;
    let (input, from) = many_till(complete::anychar, complete::u32)(input)?;
    let (input, to) = many_till(complete::anychar, complete::u32)(input)?;

    Ok((input, vec![count.1, from.1 - 1, to.1 - 1]))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, lines) = separated_list1(newline, parse_line)(input)?;
    Ok((input, lines))
}

fn part_one(starting_position: &str, input: &str) -> String {
    let (_, moves) = parse_input(input).unwrap();
    let start: Vec<Vec<char>> = starting_position
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let length = start[0].len();
    let mut cols: Vec<Vec<&char>> = vec![];
    let mut single_col: Vec<&char> = vec![];
    let mut i = 1;
    while i < length {
        for j in 0..start.len() {
            if &start[j][i] != &' ' {
                single_col.push(&start[j][i]);
            }
        }
        cols.push(single_col);
        single_col = vec![];
        i += 4;
    }

    for mov in moves {
        let count = mov[0];
        let from = mov[1];
        let to = mov[2];
        let moved = cols[from as usize]
            .drain(0..(count as usize))
            .rev()
            .collect::<Vec<&char>>();

        cols[to as usize].splice(0..0, moved);
    }
    let mut result: Vec<char> = vec![];

    // take the first element of each column and add to result

    for col in cols {
        result.push(*col[0]);
    }

    result.iter().collect::<String>()
}

fn part_two(starting_position: &str, input: &str) -> String {
    let (_, moves) = parse_input(input).unwrap();
    let start: Vec<Vec<char>> = starting_position
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let length = start[0].len();
    let mut cols: Vec<Vec<&char>> = vec![];
    let mut single_col: Vec<&char> = vec![];
    let mut i = 1;
    while i < length {
        for j in 0..start.len() {
            if &start[j][i] != &' ' {
                single_col.push(&start[j][i]);
            }
        }
        cols.push(single_col);
        single_col = vec![];
        i += 4;
    }

    for mov in moves {
        let count = mov[0];
        let from = mov[1];
        let to = mov[2];
        let moved = cols[from as usize]
            .drain(0..(count as usize))
            .collect::<Vec<&char>>();

        cols[to as usize].splice(0..0, moved);
    }
    let mut result: Vec<char> = vec![];

    // take the first element of each column and add to result

    for col in cols {
        result.push(*col[0]);
    }

    result.iter().collect::<String>()
}

//tests

#[cfg(test)]

mod tests {

    use super::*;

    const STARTING_POSITION: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
";

    const TEST_INPUT: &str = "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]

    fn test_part_one() {
        assert_eq!(part_one(STARTING_POSITION, TEST_INPUT), "CMZ");
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(STARTING_POSITION, TEST_INPUT), "MCD");
    }
}

#![allow(unused_variables, dead_code, unused_imports)]

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, newline},
    multi::{many_till, separated_list1},
    sequence::separated_pair,
    IResult,
};

pub fn input() {
    let input = include_str!("../inputs/2022/2022_day_8.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[derive(Debug)]
struct Tree {
    height: u32,
    left_neighbors: Vec<u32>,
    right_neighbors: Vec<u32>,
    top_neighbors: Vec<u32>,
    bottom_neighbors: Vec<u32>,
}

enum Direction {
    Left,
    Right,
    Top,
    Bottom,
}

fn get_score(tree: &Tree, direction: Direction) -> u32 {
    let mut score = 0;
    let neighbors = match direction {
        Direction::Left => &tree.left_neighbors,
        Direction::Right => &tree.right_neighbors,
        Direction::Top => &tree.top_neighbors,
        Direction::Bottom => &tree.bottom_neighbors,
    };
    for neighbor in neighbors {
        if neighbor < &tree.height {
            score += 1;
        } else {
            score += 1;
            break;
        }
    }
    score
}

impl Tree {
    fn get_view_score(&self) -> u32 {
        let right = get_score(&self, Direction::Right);
        let left = get_score(&self, Direction::Left);
        let top = get_score(&self, Direction::Top);
        let bottom = get_score(&self, Direction::Bottom);
        right * left * top * bottom
    }
    fn is_visible(&self) -> bool {
        let max_right = self.right_neighbors.iter().max();
        let max_left = self.left_neighbors.iter().max();
        let max_top = self.top_neighbors.iter().max();
        let max_bottom = self.bottom_neighbors.iter().max();

        let maxes = vec![max_right, max_left, max_top, max_bottom];

        for max in maxes {
            match max {
                Some(max) => {
                    if max < &self.height {
                        return true;
                    }
                }
                None => return true,
            }
        }

        return false;
    }
}

fn find_neighbors(input: &Vec<Vec<u32>>, row: usize, col: usize) -> Tree {
    let width = input[0].len();
    let height = input.len();
    let entire_column = input.iter().map(|row| row[col]).collect::<Vec<u32>>();
    let entire_row = input[row].clone();

    let right_neighbors = entire_row
        .iter()
        .skip(col + 1)
        .cloned()
        .collect::<Vec<u32>>();

    let left_neighbors = entire_row.iter().take(col).cloned().rev().collect::<Vec<u32>>();

    let top_neighbors = entire_column
        .iter()
        .take(row)
        .cloned()
        .rev()
        .collect::<Vec<u32>>();

    let bottom_neighbors = entire_column
        .iter()
        .skip(row + 1)
        .cloned()
        .collect::<Vec<u32>>();

    Tree {
        height: input[row][col],
        left_neighbors,
        right_neighbors,
        top_neighbors,
        bottom_neighbors,
    }
}

fn part_one(input: &str) -> u32 {
    let farm = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    // collect all visible trees into vector
    let mut visible_trees = Vec::new();
    for row in 0..farm.len() {
        for col in 0..farm[0].len() {
            let tree = find_neighbors(&farm, row, col);
            if tree.is_visible() {
                visible_trees.push(tree);
            }
        }
    }

    visible_trees.len() as u32
}

fn part_two(input: &str) -> u32 {
    let farm = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();



    let mut max = 0;

    for row in 0..farm.len() {
        for col in 0..farm[0].len() {
            let tree = find_neighbors(&farm, row, col);
            let score = tree.get_view_score();
            if score > max {
                max = score;
            }
        }
    }

    max
}

//tests

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 21);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 8);
    }
}

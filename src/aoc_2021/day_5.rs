// allow unused
#![allow(unused, dead_code, unused_imports, unused_variables, unused_mut)]

use std::ops::RangeInclusive;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

impl Line {
    fn new(p1: Point, p2: Point) -> Self {
        Line { p1, p2 }
    }
    fn length(&self) -> f64 {
        let x = (self.p1.x as f64 - self.p2.x as f64).powi(2);
        let y = (self.p1.y as f64 - self.p2.y as f64).powi(2);
        (x + y).sqrt()
    }

    fn points_covered(&self) -> Vec<Point> {
        let mut points = Vec::new();
        if self.is_horizontal() {
            let min = self.p1.x.min(self.p2.x);
            let max = self.p1.x.max(self.p2.x);
            let range = RangeInclusive::new(min, max);
            for x in range {
                points.push(Point { x, y: self.p1.y });
            }
        } else if self.is_vertical() {
            let min = self.p1.y.min(self.p2.y);
            let max = self.p1.y.max(self.p2.y);
            let range = RangeInclusive::new(min, max);
            for y in range {
                points.push(Point { x: self.p1.x, y });
            }
        } else if self.is_diagonal() {
          // find the direction of the line
          let x_dir = if self.p1.x < self.p2.x { 1 } else { -1 };
          let y_dir = if self.p1.y < self.p2.y { 1 } else { -1 };
          let mut x = self.p1.x;
          let mut y = self.p1.y;
          while x != self.p2.x && y != self.p2.y {
            points.push(Point { x, y });
            x += x_dir;
            y += y_dir;
          }
          points.push(Point { x: self.p2.x, y: self.p2.y })
          
        }
        points
    }

    fn is_vertical(&self) -> bool {
        self.p1.x == self.p2.x
    }

    fn is_diagonal(&self) -> bool {
        (self.p1.x - self.p2.x).abs() == (self.p1.y - self.p2.y).abs()
    }

    fn is_horizontal(&self) -> bool {
        self.p1.y == self.p2.y
    }
}

fn parse_point(input: &str) -> IResult<&str, Point> {
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i32(input)?;
    Ok((input, Point { x, y }))
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    let (input, p1) = parse_point(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, p2) = parse_point(input)?;
    Ok((input, Line::new(p1, p2)))
}

fn parse_input(input: &str) -> IResult<&str, Vec<Line>> {
    separated_list1(newline, parse_line)(input)
}

fn part_one(input: &str) -> usize {
    let mut lines = parse_input(input).unwrap().1;

    lines.retain(|x| x.is_horizontal() || x.is_vertical());

    let flat_points = lines.iter().flat_map(|x| x.points_covered());

    //count the number of times each point appears
    let mut point_counts = flat_points.fold(std::collections::HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    point_counts.iter().filter(|(_, v)| **v > 1).count()
}

fn part_two(input: &str) -> usize {
    let mut lines = parse_input(input).unwrap().1;

    lines.retain(|x| x.is_horizontal() || x.is_vertical() || x.is_diagonal());


    let flat_points = lines.iter().flat_map(|x| x.points_covered());

    //count the number of times each point appears
    let mut point_counts = flat_points.fold(std::collections::HashMap::new(), |mut acc, x| {
        *acc.entry(x).or_insert(0) += 1;
        acc
    });

    // print the counts on a grid based on the point locations
    // let mut grid = vec![vec!['.'; 10]; 10];
    // for (point, count) in &point_counts {
    //     grid[point.y as usize][point.x as usize] = count.to_string().chars().next().unwrap();
    // }
    // for row in grid {
    //     println!("{:?}", row);
    // }

    point_counts.iter().filter(|(_, v)| **v > 1).count()
    // 9
}

pub fn input() {
    let input = include_str!("../inputs/2021/2021_day_5.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

//tests

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]

    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 5);
    }

    #[test]

    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 12);
    }
}

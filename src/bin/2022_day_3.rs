#![allow(unused)]

// completed on my own. It took a couple of hours to figure out the array manipulation, but I got it.
fn main() {
    let input = include_str!("../inputs/2022/2022_day_3.txt");
    let test_input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    fn parse(line: &str) -> Vec<char> {
        line.chars().collect()
    }

    fn char_into_int(c: char) -> i32 {
        match c {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            'i' => 9,
            'j' => 10,
            'k' => 11,
            'l' => 12,
            'm' => 13,
            'n' => 14,
            'o' => 15,
            'p' => 16,
            'q' => 17,
            'r' => 18,
            's' => 19,
            't' => 20,
            'u' => 21,
            'v' => 22,
            'w' => 23,
            'x' => 24,
            'y' => 25,
            'z' => 26,
            'A' => 27,
            'B' => 28,
            'C' => 29,
            'D' => 30,
            'E' => 31,
            'F' => 32,
            'G' => 33,
            'H' => 34,
            'I' => 35,
            'J' => 36,
            'K' => 37,
            'L' => 38,
            'M' => 39,
            'N' => 40,
            'O' => 41,
            'P' => 42,
            'Q' => 43,
            'R' => 44,
            'S' => 45,
            'T' => 46,
            'U' => 47,
            'V' => 48,
            'W' => 49,
            'X' => 50,
            'Y' => 51,
            'Z' => 52,
            _ => 0,
        }
    }

    fn find_duplicates(line_1: &[char], line_2: &[char]) -> Vec<i32> {
        let duplicates: Vec<i32> = line_1
            .iter()
            .filter(|&c| line_2.contains(c))
            .map(|&c| char_into_int(c))
            .collect();

        duplicates
    }

    fn part_one(input: &str) -> i32 {
        // split the input into lines

        let lines: Vec<&str> = input.lines().collect();

        let mut total = lines
            .iter()
            .map(|line| {
                [
                    parse(&line[0..line.len() / 2]),
                    parse(&line[line.len() / 2..line.len()]),
                ]
            })
            .flat_map(|line| {
                let mut duplicates = find_duplicates(&line[0], &line[1]);
                duplicates.sort();
                duplicates.dedup();
                duplicates
            })
            .sum::<i32>();

        total
    }

    fn part_two(input: &str) -> i32 {
        let lines: Vec<&str> = input.lines().collect();

        // split lines into groups of 3
        let mut groups: Vec<Vec<&str>> = Vec::new();

        let result = lines
            .chunks(3)
            .map(|chunk| chunk.to_vec())
            .flat_map(|group| {
                let g_1 = parse(group[0]);
                let g_2 = parse(group[1]);
                let g_3 = parse(group[2]);
                let mut dup_1: Vec<i32> = g_1
                    .iter()
                    .filter(|&c| g_2.contains(c))
                    .filter(|&c| g_3.contains(c))
                    .map(|&c| char_into_int(c))
                    .collect();

                dup_1.sort();
                dup_1.dedup();
                dup_1
            })
            .sum::<i32>();

        result
    }

    assert_eq!(part_one(test_input), 157);
    assert_eq!(part_two(test_input), 70);
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

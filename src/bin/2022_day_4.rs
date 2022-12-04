fn part_one(input: &str) -> usize {
    let lines: Vec<Vec<Vec<i32>>> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split('-')
                        .map(|limit| limit.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    // println!("{:?}", lines);

    let contain_count = lines
        .iter()
        .map(|line| match line.as_slice() {
            [first, second] => {
                let first = first.as_slice();
                let second = second.as_slice();
                let first_contains = first[0] <= second[0] && first[1] >= second[1];
                let second_contains = second[0] <= first[0] && second[1] >= first[1];
                first_contains || second_contains
            }
            _ => false,
        })
        .filter(|x| x == &true)
        .count();

    contain_count
}

fn part_two(input: &str) -> usize {
    let lines: Vec<Vec<Vec<i32>>> = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|range| {
                    range
                        .split('-')
                        .map(|limit| limit.parse::<i32>().unwrap())
                        .collect()
                })
                .collect()
        })
        .collect();

    // println!("{:?}", lines);

    let contain_count = lines
        .iter()
        .map(|line| match line.as_slice() {
            [first, second] => {
                let first = first.as_slice();
                let second = second.as_slice();
                let first_contains = (first[0] <= second[0] && first[1] >= second[0])
                    || (first[1] >= second[1] && first[0] <= second[1]);
                let second_contains = (second[0] <= first[0] && second[1] >= first[0])
                    || (second[1] >= first[1] && second[0] <= first[1]);

                first_contains || second_contains
            }
            _ => false,
        })
        .filter(|x| x == &true)
        .count();

    contain_count
}

fn main() {
    let input = include_str!("../inputs/2022/2022_day_4.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

//tests

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]

    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 2);
    }

    #[test]

    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), 4);
    }
}

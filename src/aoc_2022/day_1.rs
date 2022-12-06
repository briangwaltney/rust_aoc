fn part_one(input: &str) -> u32 {
    let ans = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max();

    ans.unwrap()
}

fn part_two(input: &str) -> u32 {
    let mut part2 = input
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    part2.sort_by(|a, b| b.cmp(a));

    part2.iter().take(3).sum::<u32>()
}

pub fn input() {
    let input = include_str!("../inputs/2022/2022_day_1.txt");
    println!("Part one: {:?}", part_one(input));
    println!("Part two: {:?}", part_two(input));
}

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_ONE: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]

    fn test_part_one() {
        assert_eq!(part_one(TEST_ONE), 24000);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_ONE), 45000);
    }
}

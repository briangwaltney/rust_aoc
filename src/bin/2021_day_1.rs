fn main() {
    let input = include_str!("../inputs/2021/2021_day_1.txt");
    let test_input = "199
200
208
210
200
207
240
269
260
263";

    fn count_inc(numbers: &Vec<i32>) -> i32 {
        let mut total = 0;

        for i in 1..numbers.len() {
            if numbers[i] > numbers[i - 1] {
                total += 1
            }
        }
        total
    }

    fn part_one(input: &str) -> i32 {
        // convert input to a vector of integers
        let numbers: Vec<i32> = input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        count_inc(&numbers)
    }

    fn part_two(input: &str) -> i32 {
        let mut numbers: Vec<i32> = input
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        for i in 0..numbers.len() - 2 {
            let sum: i32 = numbers[i] + numbers[i + 1] + numbers[i + 2];
            numbers[i] = sum;
        }

        // truncate the last two elements from the end of numbers and

        numbers.truncate(numbers.len() - 2);

        count_inc(&numbers)
    }

    assert_eq!(part_one(test_input), 7);
    assert_eq!(part_two(test_input), 5);
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

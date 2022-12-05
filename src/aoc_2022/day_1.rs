pub fn run() {
    let text =
        std::fs::read_to_string("./src/inputs/2022/2022_day_1.txt").expect("Error reading file");

    let part1 = text
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();

    let mut part2 = text
        .split("\n\n")
        .map(|block| {
            block
                .lines()
                .map(|item| item.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>();

    part2.sort_by(|a, b| b.cmp(a));

    println!("{:?}", part1);
    println!("{:?}", part2.iter().take(3).sum::<u32>());
}

use std::str::FromStr;
enum Move {
    Rock,
    Paper,
    Scissors,
}

enum Res {
    Win,
    Lose,
    Draw,
}

impl Res {
    fn calc_total(&self, other: &Move) -> u32 {
        match self {
            Res::Win => match other {
                Move::Rock => 6 + &Move::Paper.use_score(),
                Move::Paper => 6 + &Move::Scissors.use_score(),
                Move::Scissors => 6 + &Move::Rock.use_score(),
            },
            Res::Lose => match other {
                Move::Rock => Move::Scissors.use_score(),
                Move::Paper => Move::Rock.use_score(),
                Move::Scissors => Move::Paper.use_score(),
            },
            Res::Draw => match other {
                Move::Rock => 3 + &Move::Rock.use_score(),
                Move::Paper => 3 + &Move::Paper.use_score(),
                Move::Scissors => 3 + &Move::Scissors.use_score(),
            },
        }
    }
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(format!("Invalid move: {}", s)),
        }
    }
}

impl Move {
    fn convert_to_win(&self) -> Res {
        match self {
            Move::Rock => Res::Lose,
            Move::Paper => Res::Draw,
            Move::Scissors => Res::Win,
        }
    }

    fn use_score(&self) -> u32 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }

    fn win_lose(&self, other: &Move) -> u32 {
        match self {
            Move::Rock => match other {
                Move::Rock => 3,
                Move::Paper => 0,
                Move::Scissors => 6,
            },
            Move::Paper => match other {
                Move::Rock => 6,
                Move::Paper => 3,
                Move::Scissors => 0,
            },
            Move::Scissors => match other {
                Move::Rock => 0,
                Move::Paper => 6,
                Move::Scissors => 3,
            },
        }
    }
}

fn part_one(input: &str) -> u32 {
    let part_one = &input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(' ')
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            moves
        })
        .map(|moves| -> u32 { moves[1].use_score() + moves[1].win_lose(&moves[0]) })
        .sum::<u32>();

    *part_one

}

fn part_two(input: &str) -> u32 {
    let part_two = &input
        .lines()
        .map(|line| {
            let moves: Vec<Move> = line
                .split(' ')
                .map(|s| s.parse::<Move>().unwrap())
                .collect();
            moves
        })
        // the Move for [1] needs to be converted to win or loss
        .map(|moves| -> u32 {
            let res = moves[1].convert_to_win();
            res.calc_total(&moves[0])
        })
        .sum::<u32>();

        *part_two

}

pub fn input() {
    // read text from 2.txt in this directory
    let input = include_str!("../inputs/2022/2022_day_2.txt");
    println!("Part one: {:?}", part_one(input));
    println!("Part two: {:?}", part_two(input));

}

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_ONE: &str = "A Y
B X
C Z";

    #[test]

    fn test_part_one() {
        assert_eq!(part_one(TEST_ONE), 15);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_ONE), 12);
    }
}

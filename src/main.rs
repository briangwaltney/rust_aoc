#![feature(iter_intersperse)]
mod aoc_2022;
mod aoc_2021;



fn main() {

// prompt user for input
    // println!("Which year would you like to run?");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input).unwrap();
    // let year = input.trim().parse::<u32>().unwrap();

    // match year {
    //     2022 => {
    //         crate::aoc_2022::run();
    //     }
    //     2021 => {
    //         crate::aoc_2021::run();
    //     }
    //     _ => println!("Invalid year"),
    // }

    






    println!("2022:");
    crate::aoc_2022::run();
    // println!("---------------------");
    // println!("2021:");
    // crate::aoc_2021::run();
}

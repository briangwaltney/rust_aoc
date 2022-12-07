#![feature(iter_intersperse)]
mod aoc_2022;
mod aoc_2021;



fn main() {
    println!("2022:");
    crate::aoc_2022::run();
    println!("---------------------");
    println!("2021:");
    crate::aoc_2021::run();
}

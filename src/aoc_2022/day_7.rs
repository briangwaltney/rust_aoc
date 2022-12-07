#![allow(unused_variables, dead_code, unused_imports)]

use std::collections::{BTreeMap, BTreeSet, HashSet};

use nom::{
    branch::alt,
    bytes::complete::{is_a, tag},
    character::complete::{self, alpha1, newline},
    multi::{many_till, separated_list1},
    sequence::separated_pair,
    IResult,
};

pub fn input() {
    let input = include_str!("../inputs/2022/2022_day_7.txt");

    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

#[derive(Debug)]
enum Operation<'a> {
    Cd(Cd<'a>),
    Ls(Vec<Files<'a>>),
}

#[derive(Debug)]
enum Cd<'a> {
    Root,
    Up,
    Down(&'a str),
}

#[derive(Debug)]
enum Files<'a> {
    Dir(&'a str),
    File { name: &'a str, size: u32 },
}

fn file(input: &str) -> IResult<&str, Files> {
    let (input, (size, name)) = separated_pair(
        nom::character::complete::u32,
        tag(" "),
        is_a("qwertyuiopasdfghjklzxcvbnm."),
    )(input)?;
    Ok((input, Files::File { size, name }))
}
fn directory(input: &str) -> IResult<&str, Files> {
    let (input, _) = tag("dir ")(input)?;
    let (input, name) = alpha1(input)?;
    Ok((input, Files::Dir(name)))
}
fn ls(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ ls")(input)?;
    let (input, _) = newline(input)?;
    let (input, files) = separated_list1(newline, alt((file, directory)))(input)?;
    Ok((input, Operation::Ls(files)))
}
fn cd(input: &str) -> IResult<&str, Operation> {
    let (input, _) = tag("$ cd ")(input)?;
    let (input, dir) = alt((tag(".."), alpha1, tag("/")))(input)?;
    let op = match dir {
        "/" => Operation::Cd(Cd::Root),
        ".." => Operation::Cd(Cd::Up),
        name => Operation::Cd(Cd::Down(name)),
    };
    Ok((input, op))
}
fn commands(input: &str) -> IResult<&str, Vec<Operation>> {
    let (input, cmd) = separated_list1(newline, alt((ls, cd)))(input)?;

    Ok((input, cmd))
}

#[derive(Debug)]
struct File<'a> {
    name: &'a str,
    size: u32,
}

fn part_one(input: &str) -> u32 {
    let cmds = commands(input).unwrap().1;
    let mut directories: BTreeMap<String, Vec<File>> =
        BTreeMap::new();
    let mut context: Vec<&str> = vec![];

    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                context.push("");
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
            }
            Operation::Cd(Cd::Down(name)) => {
                context.push(name);
            }
            Operation::Ls(files) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .intersperse("/")
                            .collect::<String>(),
                    )
                    .or_insert(vec![]);
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            directories
                                .entry(
                                    context
                                        .iter()
                                        .cloned()
                                        .intersperse("/")
                                        .collect::<String>(
                                        ),
                                )
                                .and_modify(|vec| {
                                    vec.push(File {
                                        size: *size,
                                        name,
                                    });
                                });
                        }
                        Files::Dir(_) => (),
                    }
                }
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let size = files
            .iter()
            .map(|File { size, .. }| size)
            .sum::<u32>();
        for i in 0..dirs.len() {
            sizes
                .entry(
                    (&dirs[0..=i])
                        .iter()
                        .cloned()
                        .intersperse("/")
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }
    sizes
        .iter()
        .filter(|(_, &size)| size < 100000)
        .map(|(_, size)| size)
        .sum::<u32>()
}

fn part_two(input: &str) -> String {
    let cmds = commands(input).unwrap().1;
    let mut directories: BTreeMap<String, Vec<File>> =
        BTreeMap::new();
    let mut context: Vec<&str> = vec![];

    for command in cmds.iter() {
        match command {
            Operation::Cd(Cd::Root) => {
                context.push("");
            }
            Operation::Cd(Cd::Up) => {
                context.pop();
            }
            Operation::Cd(Cd::Down(name)) => {
                context.push(name);
            }
            Operation::Ls(files) => {
                directories
                    .entry(
                        context
                            .iter()
                            .cloned()
                            .intersperse("/")
                            .collect::<String>(),
                    )
                    .or_insert(vec![]);
                for file in files.iter() {
                    match file {
                        Files::File { size, name } => {
                            directories
                                .entry(
                                    context
                                        .iter()
                                        .cloned()
                                        .intersperse("/")
                                        .collect::<String>(
                                        ),
                                )
                                .and_modify(|vec| {
                                    vec.push(File {
                                        size: *size,
                                        name,
                                    });
                                });
                        }
                        Files::Dir(_) => (),
                    }
                }
            }
        }
    }

    let mut sizes: BTreeMap<String, u32> = BTreeMap::new();
    for (path, files) in directories.iter() {
        let dirs = path.split("/").collect::<Vec<&str>>();
        let size = files
            .iter()
            .map(|File { size, .. }| size)
            .sum::<u32>();
        for i in 0..dirs.len() {
            sizes
                .entry(
                    (&dirs[0..=i])
                        .iter()
                        .cloned()
                        .intersperse("/")
                        .collect::<String>(),
                )
                .and_modify(|v| *v += size)
                .or_insert(size);
        }
    }

    let total_size = 70_000_000;
    let needed_space = 30_000_000;

    let used_space = sizes.get("").unwrap();

    let current_free_space = total_size - used_space;
    let need_to_free_at_least =
        needed_space - current_free_space;

    let mut valid_dirs = sizes
        .iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, size)| size)
        .collect::<Vec<&u32>>();

    valid_dirs.sort();
    valid_dirs.iter().next().unwrap().to_string()
}

//tests

#[cfg(test)]

mod tests {

    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(TEST_INPUT), 95437);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(TEST_INPUT), "24933642".to_string());
    }
}

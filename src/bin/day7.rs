use aoc2022::get_input_file;
use std::{
    cmp::min,
    io::{BufRead, BufReader},
};

fn main() {
    let file = get_input_file();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let input: Vec<_> = lines.collect::<Result<_, _>>().unwrap();

    println!("{}", part_one(input.clone()));
    println!("{}", part_two(input));
}

fn part_one(input: Vec<String>) -> u32 {
    let mut out = 0;

    let mut stack = Vec::new();

    for line in input {
        match line {
            _ if line.starts_with('$') => {
                let cmd = &line[2..];
                match cmd {
                    _ if cmd.starts_with("cd") => {
                        let p = &cmd[3..];
                        if p == ".." {
                            let v = stack.pop().unwrap();
                            if v <= 100_000 {
                                out += v;
                            }
                        }
                    }
                    _ if cmd.starts_with("ls") => {
                        stack.push(0);
                    }
                    _ => panic!(),
                }
            }
            _ if line.starts_with("dir") => {}
            _ => {
                let size: u32 = line.split(' ').next().unwrap().parse().unwrap();
                for dir in &mut stack {
                    *dir += size;
                }
            }
        }
    }

    for dir in stack.into_iter() {
        if dir <= 100_000 {
            out += dir;
        }
    }

    out
}

#[test]
fn test_part_one() {
    let input = "\
$ cd /
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
7214296 k
";

    let input = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    assert_eq!(part_one(input), 95437);
}

fn part_two(input: Vec<String>) -> u32 {
    let mut dirs = Vec::new();

    let mut stack = Vec::new();

    for line in input {
        match line {
            _ if line.starts_with('$') => {
                let cmd = &line[2..];
                match cmd {
                    _ if cmd.starts_with("cd") => {
                        let p = &cmd[3..];
                        if p == ".." {
                            let v = stack.pop().unwrap();
                            dirs.push(v);
                        }
                    }
                    _ if cmd.starts_with("ls") => {
                        stack.push(0);
                    }
                    _ => panic!(),
                }
            }
            _ if line.starts_with("dir") => {}
            _ => {
                let size: u32 = line.split(' ').next().unwrap().parse().unwrap();
                for dir in &mut stack {
                    *dir += size;
                }
            }
        }
    }

    dirs.extend(stack.into_iter().rev());

    let disk_space = 70_000_000;
    let used = dirs.last().unwrap();
    let unused = disk_space - used;

    let required = 30_000_000;
    let delta = required - unused;

    let mut target_dir = None;
    for dir in dirs {
        if dir < delta {
            continue;
        }
        target_dir = match target_dir {
            Some(curr) => Some(min(curr, dir)),
            None => Some(dir),
        };
    }

    target_dir.unwrap()
}

#[test]
fn test_part_two() {
    let input = "\
$ cd /
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
7214296 k
";

    let input = input
        .split('\n')
        .filter(|x| !x.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    assert_eq!(part_two(input), 24933642);
}

// this one is interesting
// I almost got baited to build a directory tree, then realize I just need the
// directory sizes

use aoc2022::get_input_file;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file = get_input_file();
    let reader = BufReader::new(file);

    // can have O(1) space but unnecessary and might bite me later on in part two
    let mut elves = Vec::<u32>::new();
    elves.push(0);

    for line in reader.lines() {
        let line = line.unwrap();

        match line.as_str() {
            "" => {
                elves.push(0);
            }
            c => {
                let c: u32 = c.parse().unwrap();
                let i = elves.len() - 1;
                elves[i] += c;
            }
        }
    }

    let max = elves.iter().max().unwrap();
    println!("{}", max);
}

fn part_two() {
    let file = get_input_file();
    let reader = BufReader::new(file);

    let mut elves = Vec::<u32>::new();
    elves.push(0);

    for line in reader.lines() {
        let line = line.unwrap();

        match line.as_str() {
            "" => {
                elves.push(0);
            }
            c => {
                let c: u32 = c.parse().unwrap();
                let i = elves.len() - 1;
                elves[i] += c;
            }
        }
    }

    // yep, almost got baited
    elves.sort_unstable();
    elves.reverse();
    let top3: u32 = elves.iter().take(3).sum();
    println!("{}", top3);
}

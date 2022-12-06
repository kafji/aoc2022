use aoc2022::get_input_file;
use std::{collections::HashSet, io::Read};

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut file = get_input_file();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let pos = find_marker(&buf.chars().collect::<Vec<_>>(), 4);
    println!("{}", pos)
}

fn find_marker(buf: &[char], len: usize) -> usize {
    for i in 0..=(buf.len() - len) {
        let mut set = HashSet::new();
        for j in 0..len {
            let c = buf[i + j];
            set.insert(c);
        }
        if set.len() == len {
            return i + len;
        }
    }
    panic!()
}

#[test]
fn test_find_marker() {
    assert_eq!(
        find_marker(
            &"mjqjpqmgbljsphdztnvjfqwrcgsmlb".chars().collect::<Vec<_>>(),
            4
        ),
        7
    );
}

fn part_two() {
    let mut file = get_input_file();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let pos = find_marker(&buf.chars().collect::<Vec<_>>(), 14);
    println!("{}", pos)
}

// huh, this one is surprisingly easy

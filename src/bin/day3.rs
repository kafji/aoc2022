use aoc2022::get_input_file;
use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
    mem,
};

fn main() {
    part_one();
    part_two();
}

fn item_prio(c: char) -> u8 {
    match c {
        'a'..='z' => 1 + c as u8 - 97,
        'A'..='Z' => 27 + c as u8 - 65,
        _ => panic!(),
    }
}

fn part_one() {
    let file = get_input_file();
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let n = line.len();

        let mut first_half = HashSet::new();
        first_half.extend(line.chars().take(n / 2));

        for c in line.chars().skip(n / 2) {
            if first_half.contains(&c) {
                let prio = item_prio(c);
                sum += prio as u32;
                break;
            }
        }
    }

    println!("{}", sum);
}

fn find_common_element_inner(
    v1: &[u8],
    v2: &[u8],
    v3: &[u8],
    mut i1: usize,
    mut i2: usize,
    mut i3: usize,
) -> Option<char> {
    let c1 = v1[i1];
    let c2 = v2[i2];
    let c3 = v3[i3];

    if c1 == c2 && c2 == c3 {
        return Some(c1 as _);
    }

    i3 += 1;
    if i3 > v3.len() - 1 {
        i3 = 0;
        i2 += 1;
        if i2 > v2.len() - 1 {
            i2 = 0;
            i1 += 1;
            if i1 > v1.len() - 1 {
                return None;
            }
        }
    }

    find_common_element_inner(v1, v2, v3, i1, i2, i3)
}

fn find_common_element<'a>(v1: &'a str, v2: &'a str, v3: &'a str) -> Option<char> {
    // remove duplicate elements from each vectors
    let v1 = HashSet::<u8>::from_iter(v1.bytes());
    let v2 = HashSet::<u8>::from_iter(v2.bytes());
    let v3 = HashSet::<u8>::from_iter(v3.bytes());
    let mut v1 = Vec::from_iter(v1);
    let mut v2 = Vec::from_iter(v2);
    let mut v3 = Vec::from_iter(v3);

    // sort vectors
    if v2.len() < v1.len() {
        mem::swap(&mut v1, &mut v2);
    }
    if v3.len() < v2.len() {
        if v3.len() < v1.len() {
            mem::swap(&mut v2, &mut v3);
            mem::swap(&mut v1, &mut v2);
        } else {
            mem::swap(&mut v2, &mut v3);
        }
    }

    find_common_element_inner(&v1, &v2, &v3, 0, 0, 0)
}

#[test]
fn test_common_element() {
    let el = find_common_element("xbc", "dxf", "ghx");
    assert_eq!(el, Some('x'));

    let el = find_common_element("abc", "dxf", "ghx");
    assert_eq!(el, None);

    let el = find_common_element(
        "vJrwpWtwJgWrhcsFMMfFFhFp",
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
        "PmmdzqPrVvPwwTWBwg",
    );
    assert_eq!(el, Some('r'));

    let el = find_common_element(
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
        "ttgJtRGJQctTZtZT",
        "CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    assert_eq!(el, Some('Z'));

    let el = find_common_element(
        "gtZDjBcmpcDgpZcmmbgtdtqmCGVCGGsvhCFCCqvmCMMM",
        "JrhfzfLTNfJhPnhQnfzHfCFFQFSGvMFCGQFsQSMSVs",
        "TllTRrfNNlfzwhtZBZgtRDBp",
    );
    assert_eq!(el, Some('h'));
}

fn part_two() {
    let file = get_input_file();
    let reader = BufReader::new(file);

    let mut sum: u32 = 0;

    let mut lines = reader.lines();
    loop {
        let v1 = match lines.next() {
            Some(line) => line.unwrap(),
            None => break,
        };
        let v2 = lines.next().unwrap().unwrap();
        let v3 = lines.next().unwrap().unwrap();

        let badge = find_common_element(&v1, &v2, &v3).expect("found no common element");

        sum += item_prio(badge) as u32;
    }

    println!("{}", sum);
}

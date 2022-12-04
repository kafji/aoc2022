use aoc2022::get_input_file;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
    part_two();
}

/// Inclusive range.
#[derive(PartialEq, Clone, Copy, Debug, Default)]
struct Sections {
    from: u8,
    until: u8,
}

impl Sections {
    #[cfg(test)]
    fn new(f: u8, u: u8) -> Self {
        Sections { from: f, until: u }
    }

    fn len(&self) -> u8 {
        1 + self.until - self.from
    }
}

enum ParserState {
    ReadFrom(String),
    ReadUntil(String),
}

fn parse_line(s: &str) -> (Sections, Sections) {
    // writing this parser takes a longer time than I had expected, maybe I
    // should use regex next time...

    let mut buf = [None; 4];

    let mut state = ParserState::ReadFrom(Default::default());

    for c in s.chars() {
        let new_state = match state {
            ParserState::ReadFrom(mut b) => match c {
                '0'..='9' => {
                    b.push(c);
                    ParserState::ReadFrom(b)
                }
                '-' => {
                    let num = b.parse().unwrap();

                    if let None = buf[0] {
                        buf[0] = Some(num);
                    } else {
                        buf[2] = Some(num);
                    }

                    ParserState::ReadUntil(Default::default())
                }
                _ => panic!(),
            },
            ParserState::ReadUntil(mut b) => match c {
                '0'..='9' => {
                    b.push(c);
                    ParserState::ReadUntil(b)
                }
                ',' => {
                    let num = b.parse().unwrap();

                    if let None = buf[1] {
                        buf[1] = Some(num);
                    } else {
                        buf[3] = Some(num);
                    }

                    ParserState::ReadFrom(Default::default())
                }
                _ => panic!(),
            },
        };
        state = new_state;
    }

    // eof
    if let ParserState::ReadUntil(b) = state {
        let num = b.parse().unwrap();
        buf[3] = Some(num);
    }

    let fp = Sections {
        from: buf[0].unwrap(),
        until: buf[1].unwrap(),
    };

    let sp = Sections {
        from: buf[2].unwrap(),
        until: buf[3].unwrap(),
    };

    (fp, sp)
}

fn part_one() {
    // In how many assignment pairs does one range fully contain the other?

    let file = get_input_file();
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (fp, sp) = parse_line(&line);

        if fully_contain(&fp, &sp) {
            count += 1;
        }
    }

    println!("{}", count);
}

fn fully_contain<'a>(mut s1: &'a Sections, mut s2: &'a Sections) -> bool {
    if s2.len() > s1.len() {
        std::mem::swap(&mut s1, &mut s2);
    }

    // ---====----
    // ----==-----
    //
    // ---====----
    // ---====----
    if s1.from <= s2.from && s1.until >= s2.until {
        return true;
    }

    false
}

#[test]
fn test_fully_contain() {
    assert_eq!(
        true,
        fully_contain(&Sections::new(6, 6), &Sections::new(4, 6))
    );

    assert_eq!(
        false,
        fully_contain(&Sections::new(2, 4), &Sections::new(6, 8))
    );
}

fn overlap<'a>(mut s1: &'a Sections, mut s2: &'a Sections) -> bool {
    if s1.from > s2.from {
        std::mem::swap(&mut s1, &mut s2);
    }

    // --==---
    // ---==--
    //
    // ---==---
    // ---==---
    if s2.from <= s1.until {
        return true;
    }

    false
}

#[test]
fn test_overlap() {
    assert_eq!(true, overlap(&Sections::new(5, 7), &Sections::new(7, 9)));
    assert_eq!(true, overlap(&Sections::new(2, 8), &Sections::new(3, 7)));

    assert_eq!(false, overlap(&Sections::new(1, 4), &Sections::new(5, 8)));
}

fn part_two() {
    let file = get_input_file();
    let reader = BufReader::new(file);

    let mut count = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let (fp, sp) = parse_line(&line);

        if overlap(&fp, &sp) {
            count += 1;
        }
    }

    println!("{}", count);
}

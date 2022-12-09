use aoc2022::get_input_file;
use std::{
    cmp::Ordering,
    collections::HashSet,
    io::{BufRead, BufReader},
};

fn main() {
    let moves = read_input();

    part_one(moves.clone());

    println!("{}", part_two(moves));
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
struct Point(i32 /* x */, i32 /* y */);

impl Point {
    fn apply(self, d: Direction) -> Self {
        let Self(mut x, mut y) = self;
        match d {
            Direction::Up => y += 1,
            Direction::Down => y -= 1,
            Direction::Left => x -= 1,
            Direction::Right => x += 1,
        }
        Self(x, y)
    }

    fn follow(self, h: Self) -> Self {
        let t = self;

        let d = t.distance(&h);

        match d {
            (_, _, x) if x <= 1 => self,
            (Some(h), Some(v), _) => t.apply(h).apply(v),
            (Some(h), None, _) => t.apply(h),
            (None, Some(v), _) => t.apply(v),
            (None, None, _) => self,
        }
    }

    fn distance(&self, other: &Self) -> (Option<Direction>, Option<Direction>, u32) {
        let horizontally = {
            match (other.0 - self.0).cmp(&0) {
                Ordering::Less => Direction::Left.into(),
                Ordering::Equal => None,
                Ordering::Greater => Direction::Right.into(),
            }
        };

        let vertically = {
            match (other.1 - self.1).cmp(&0) {
                Ordering::Less => Direction::Down.into(),
                Ordering::Equal => None,
                Ordering::Greater => Direction::Up.into(),
            }
        };

        // I googled this.
        let x = other.0 - self.0;
        let y = other.1 - self.1;
        let d = ((x.pow(2) + y.pow(2)) as f64).sqrt();

        (horizontally, vertically, d as _)
    }
}

#[test]
fn test_distance() {
    let h = Point(1, 1);
    let t = Point(1, 1);
    assert_eq!(t.distance(&h), (None, None, 0));

    let h = Point(1, 1);
    let t = Point(0, 0);
    assert_eq!(
        t.distance(&h),
        (Direction::Right.into(), Direction::Up.into(), 1)
    );

    let h = Point(2, 0);
    let t = Point(0, 0);
    assert_eq!(t.distance(&h), (Direction::Right.into(), None, 2));

    let h = Point(2, 1);
    let t = Point(0, 0);
    assert_eq!(
        t.distance(&h),
        (Direction::Right.into(), Direction::Up.into(), 2)
    );
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Move {
    direction: Direction,
    times: u16,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn read_move(s: &str) -> Move {
    let d = match &s[0..1] {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!(),
    };
    let t = s[2..].parse().unwrap();
    Move {
        direction: d,
        times: t,
    }
}

fn read_input() -> Vec<Move> {
    let input = get_input_file();
    let reader = BufReader::new(input);
    reader.lines().map(|x| read_move(&x.unwrap())).collect()
}

fn part_one(moves: Vec<Move>) {
    let mut h = Point(0, 0);
    let mut t = h;

    let mut visited = HashSet::new();
    visited.insert(t);

    for Move { direction, times } in moves {
        for _ in 1..=times {
            h = h.apply(direction);
            t = t.follow(h);
            visited.insert(t);
        }
    }

    println!("{}", visited.len());

    // hell yea! first try!
}

#[derive(Debug, Clone, Copy, Default)]
struct Rope {
    // T87654321H
    q: [Point; 10],
}

impl Rope {
    fn tail(&self) -> Point {
        self.q[0]
    }

    fn apply(&mut self, d: Direction) {
        let h = self.q[9];
        let h = h.apply(d);
        self.q[9] = h;

        for i in (0..9).rev() {
            self.q[i] = self.q[i].follow(self.q[i + 1]);
        }
    }
}

fn part_two(moves: Vec<Move>) -> u32 {
    let mut rope = Rope::default();

    let mut visited = HashSet::new();
    visited.insert(rope.tail());

    for Move { direction, times } in moves {
        for _ in 1..=times {
            // {
            //     let mut visited = visited.clone();
            //     println!();
            //     for y in -20..20 {
            //         for x in -20..20 {
            //             if visited.contains(&Point(x, y)) {
            //                 visited.remove(&Point(x, y));
            //                 print!("#");
            //             } else {
            //                 print!(".");
            //             }
            //         }
            //         println!();
            //     }
            // }

            rope.apply(direction);
            visited.insert(rope.tail());
        }
    }

    visited.len() as _

    // a couple of tries, need to improve my reading comprehension, xd
}

#[test]
fn test_part_two() {
    let input = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
    let moves = input.lines().map(|x| read_move(x)).collect();

    let count = part_two(moves);

    assert_eq!(count, 36);
}

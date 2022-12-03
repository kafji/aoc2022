use aoc2022::get_input_file;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Clone, Copy, Debug)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

fn parse_guide(s: &str) -> (Shape, Shape) {
    let mut cs = s.chars();

    let f = cs.next().unwrap();
    let against = match f {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissor,
        _ => panic!(),
    };

    // separator
    cs.next();

    let s = cs.next().unwrap();
    let with = match s {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissor,
        _ => panic!(),
    };

    (against, with)
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Outcome {
    Won,
    Draw,
    Lost,
}

fn get_outcome(with: Shape, against: Shape) -> Outcome {
    use Outcome::*;
    use Shape::*;

    match (against, with) {
        (Rock, Paper) => Won,
        (Rock, Scissor) => Lost,

        (Paper, Rock) => Lost,
        (Paper, Scissor) => Won,

        (Scissor, Rock) => Won,
        (Scissor, Paper) => Lost,

        _ => Draw,
    }
}

fn outcome_score(outcome: Outcome) -> i32 {
    match outcome {
        Outcome::Won => 6,
        Outcome::Draw => 3,
        Outcome::Lost => 0,
    }
}

fn shape_score(shape: Shape) -> i32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    }
}

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let file = get_input_file();

    let reader = BufReader::new(file);

    let mut score = 0;

    for guide in reader.lines() {
        let guide = guide.unwrap();

        let (against, with): (Shape, Shape) = parse_guide(&guide);

        let outcome = get_outcome(with, against);
        score += outcome_score(outcome);

        score += shape_score(with);
    }

    println!("{}", score);
}

fn parse_guide2(s: &str) -> (Shape, Outcome) {
    let mut cs = s.chars();

    let f = cs.next().unwrap();
    let against = match f {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissor,
        _ => panic!(),
    };

    // separator
    cs.next();

    let s = cs.next().unwrap();
    let expected = match s {
        'X' => Outcome::Lost,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Won,
        _ => panic!(),
    };

    (against, expected)
}

fn shape_for(outcome: Outcome, against: Shape) -> Shape {
    match outcome {
        Outcome::Won => match against {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        },
        Outcome::Draw => against,
        Outcome::Lost => match against {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        },
    }
}

fn part_two() {
    let file = get_input_file();

    let reader = BufReader::new(file);

    let mut score = 0;

    for guide in reader.lines() {
        let guide = guide.unwrap();

        let (against, outcome) = parse_guide2(&guide);

        score += outcome_score(outcome);

        let with = shape_for(outcome, against);
        score += shape_score(with);
    }

    println!("{}", score);
}

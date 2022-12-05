use aoc2022::get_input_file;
use parser::read_input;
use std::io::Read;

fn main() {
    part_one();
    part_two();
}

fn part_one() {
    let mut file = get_input_file();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let buf = buf.chars().collect::<Vec<_>>();
    let (mut stacks, moves) = read_input(&buf);

    for Move { qty, src, dst } in moves {
        for _ in 1..=qty {
            let src = &mut stacks[src - 1];

            let c = src.pop().unwrap();

            let dst = &mut stacks[dst - 1];

            dst.push(c);
        }
    }

    let mut tops = String::new();
    for mut s in stacks {
        let c = s.pop().unwrap();
        tops.push(c);
    }

    println!("{}", tops);
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Move {
    pub qty: u32,
    pub src: usize,
    pub dst: usize,
}

impl Move {
    pub fn new(qty: u32, src: usize, dst: usize) -> Self {
        Self { qty, src, dst }
    }
}

mod parser {
    use super::Move;

    pub type Stacks = Vec<Vec<char>>;

    fn read_stacks(buf: &[char]) -> (Stacks, usize) {
        // count how many columns incld. newline
        let mut cols = 0;
        loop {
            if buf[cols] == '\n' {
                cols += 1;
                break;
            }
            cols += 1;
        }

        // find bottom / stack num row
        let mut i = 0;
        loop {
            if buf[i] == '\n' && buf[i + 1] != '[' {
                i += 1;
                break;
            }
            i += 1;
        }

        let mut stacks = Vec::new();

        while buf[i] != '\n' {
            if buf[i] != ' ' {
                stacks.push(Vec::new());

                // read stack from bottom to top
                let mut x = i;
                while x > cols {
                    x -= cols;
                    let c = buf[x] as _;
                    if c == ' ' {
                        continue;
                    }
                    stacks.last_mut().unwrap().push(c);
                }
            }

            i += 1;
        }

        let len = i + 1;

        (stacks, len)
    }

    fn read_moves(buf: &[char]) -> Vec<Move> {
        let mut moves = Vec::new();

        let mut i = 0;

        loop {
            if i >= buf.len() {
                break;
            }

            // read move keyword
            i += 4;

            // read whitespace separator
            i += 1;

            // read qty
            let qty = {
                let (n, l) = read_num(buf, i);
                i += l;
                n as _
            };

            // read whitespace separator
            i += 1;

            // read from keyword
            i += 4;

            // read whitespace separator
            i += 1;

            // read src
            let src = {
                let (n, l) = read_num(buf, i);
                i += l;
                n as _
            };

            // read whitespace separator
            i += 1;

            // read to keyword
            i += 2;

            // read whitespace separator
            i += 1;

            // read dst
            let dst = {
                let (n, l) = read_num(buf, i);
                i += l;
                n as _
            };

            // read newline
            i += 1;

            moves.push(Move { qty, src, dst });
        }

        moves
    }

    fn read_num(buf: &[char], pos: usize) -> (i32, usize) {
        let mut i = pos;

        let mut num = String::new();

        loop {
            let c = buf[i];
            if c < '0' || c > '9' {
                break;
            }
            num.push(c);
            i += 1;
        }

        let num = num.parse().unwrap();
        let len = i - pos;

        (num, len)
    }

    pub fn read_input(buf: &[char]) -> (Stacks, Vec<Move>) {
        let (stacks, len) = read_stacks(buf);

        // newline sep
        let pos = len + 1;

        let moves = read_moves(&buf[pos..]);

        (stacks, moves)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_read_stacks() {
            let s = "\
[D]        
[N] [C]    
[Z] [M] [P]
 1   2   3 
";
            let stacks = read_stacks(&s.chars().collect::<Vec<_>>());
            assert_eq!(
                stacks,
                (
                    vec![vec!['Z', 'N', 'D',], vec!['M', 'C',], vec!['P',],],
                    s.len()
                )
            );
        }

        #[test]
        fn test_read_moves() {
            let s = "\
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";
            let moves = read_moves(&s.chars().collect::<Vec<_>>());
            assert_eq!(
                moves,
                vec![
                    Move::new(1, 2, 1),
                    Move::new(3, 1, 3),
                    Move::new(2, 2, 1),
                    Move::new(1, 1, 2),
                ]
            );
        }
    }
}

fn part_two() {
    let mut file = get_input_file();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let buf = buf.chars().collect::<Vec<_>>();
    let (mut stacks, moves) = read_input(&buf);

    for Move { qty, src, dst } in moves {
        let mut tmp = Vec::new();

        let src = &mut stacks[src - 1];
        for _ in 1..=qty {
            let c = src.pop().unwrap();
            tmp.push(c);
        }

        let dst = &mut stacks[dst - 1];
        while !tmp.is_empty() {
            let c = tmp.pop().unwrap();
            dst.push(c);
        }
    }

    let mut tops = String::new();
    for mut s in stacks {
        let c = s.pop().unwrap();
        tops.push(c);
    }

    println!("{}", tops);
}

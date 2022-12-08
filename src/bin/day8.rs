use aoc2022::get_input_file;
use std::io::{BufRead, BufReader};

fn main() {
    part_one();
    part_two();
}

fn grid() -> Vec<Vec<u32>> {
    let file = get_input_file();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|x| x.unwrap())
        .map(|line| line.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_one() {
    let grid = grid();

    let col_len = grid[0].len();
    let row_len = grid.len();

    let mut visibles = col_len * 2 + row_len * 2 - 4;

    for row in 1..=row_len - 2 {
        for col in 1..=col_len - 2 {
            if is_tree_visible(&grid, col, row) {
                visibles += 1;
            }
        }
    }

    println!("{}", visibles);
}

type Grid = Vec<Vec<u32>>;

fn is_visible_from_top(grid: &Vec<Vec<u32>>, col: usize, row: usize) -> bool {
    let height = grid[row][col];

    for i in 0..row {
        let v = grid[i][col];
        if v >= height {
            return false;
        }
    }
    return true;
}

fn is_visible_from_bottom(grid: &Vec<Vec<u32>>, col: usize, row: usize) -> bool {
    let height = grid[row][col];

    let row_len = grid.len();

    for i in row + 1..row_len {
        let v = grid[i][col];
        if v >= height {
            return false;
        }
    }
    return true;
}

fn is_visible_from_left(grid: &Vec<Vec<u32>>, col: usize, row: usize) -> bool {
    let height = grid[row][col];

    for i in 0..col {
        let v = grid[row][i];
        if v >= height {
            return false;
        }
    }
    return true;
}

fn is_visible_from_right(grid: &Vec<Vec<u32>>, col: usize, row: usize) -> bool {
    let height = grid[row][col];

    let col_len = grid[0].len();

    for i in col + 1..col_len {
        let v = grid[row][i];
        if v >= height {
            return false;
        }
    }
    return true;
}

fn is_tree_visible(grid: &Grid, col: usize, row: usize) -> bool {
    is_visible_from_top(grid, col, row)
        || is_visible_from_bottom(grid, col, row)
        || is_visible_from_left(grid, col, row)
        || is_visible_from_right(grid, col, row)
}

#[test]
fn test_is_tree_visible() {
    let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(is_tree_visible(&grid, 1, 1), true);

    let grid = vec![vec![0, 1, 0], vec![0, 1, 1], vec![0, 0, 0]];
    assert_eq!(is_tree_visible(&grid, 1, 1), true);

    let grid = vec![vec![0, 1, 0], vec![1, 1, 1], vec![0, 1, 0]];
    assert_eq!(is_tree_visible(&grid, 1, 1), false);
}

fn top_visible_trees(grid: &Vec<Vec<u32>>, col: usize, row: usize) -> u32 {
    let height = grid[row][col];

    let mut count = 0;

    for i in (0..row).rev() {
        let v = grid[i][col];
        count += 1;
        if v >= height {
            break;
        }
    }

    return count;
}

fn bottom_visible_trees(grid: &Vec<Vec<u32>>, col: usize, row: usize) -> u32 {
    let height = grid[row][col];

    let mut count = 0;

    let row_len = grid.len();

    for i in row + 1..row_len {
        let v = grid[i][col];
        count += 1;
        if v >= height {
            break;
        }
    }

    return count;
}

fn left_visible_trees(grid: &Vec<Vec<u32>>, col: usize, row: usize) -> u32 {
    let height = grid[row][col];

    let mut count = 0;

    for i in (0..col).rev() {
        let v = grid[row][i];
        count += 1;
        if v >= height {
            break;
        }
    }

    return count;
}

fn right_visible_trees(grid: &Vec<Vec<u32>>, col: usize, row: usize) -> u32 {
    let height = grid[row][col];

    let mut count = 0;

    let col_len = grid[0].len();

    for i in col + 1..col_len {
        let v = grid[row][i];
        count += 1;
        if v >= height {
            break;
        }
    }

    return count;
}

fn scenic_score(grid: &Grid, col: usize, row: usize) -> u32 {
    top_visible_trees(grid, col, row)
        * bottom_visible_trees(grid, col, row)
        * left_visible_trees(grid, col, row)
        * right_visible_trees(grid, col, row)
}

#[test]
fn test_scenic_score() {
    let grid = vec![vec![0, 0, 0], vec![0, 1, 0], vec![0, 0, 0]];
    assert_eq!(scenic_score(&grid, 1, 1), 1);

    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    assert_eq!(scenic_score(&grid, 2, 1), 1);

    let grid = vec![
        vec![0, 0, 0, 0, 0],
        vec![0, 0, 1, 0, 0],
        vec![0, 0, 0, 0, 0],
    ];
    assert_eq!(scenic_score(&grid, 2, 1), 4);
}

fn part_two() {
    let grid = grid();

    let col_len = grid[0].len();
    let row_len = grid.len();

    let mut max_score = 0;

    for row in 0..row_len {
        for col in 0..col_len {
            let score = scenic_score(&grid, col, row);
            max_score = std::cmp::max(max_score, score);
        }
    }

    println!("{}", max_score);
}

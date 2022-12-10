/*
(Debug)
🎄 Part 1 🎄
1698 (elapsed: 2.39ms)
🎄 Part 2 🎄
672280 (elapsed: 4.21ms)

(Release)
🎄 Part 1 🎄
1698 (elapsed: 115.85µs)
🎄 Part 2 🎄
672280 (elapsed: 127.84µs)
 */
use std::cmp;

macro_rules! Str2Tree {
    ($row:expr, $str:expr) => {
        $str.chars()
            .enumerate()
            .map(|(i, v)| Tree::new($row, i, v.to_digit(10).expect("Invalid digit") as u8))
            .collect::<Vec<_>>()
    };
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Tree {
    pub row: usize,
    pub col: usize,
    pub value: u8,
    pub visible: bool,
}

impl Tree {
    pub fn new(row: usize, col: usize, value: u8) -> Self {
        Tree {
            row,
            col,
            value,
            visible: false,
        }
    }

    pub fn scenic_score(&self, matrix: &Matrix) -> u32 {
        let max_row: usize = matrix.len();
        let max_col: usize = matrix[0].len();
        if !self.visible
            || self.row == 0
            || self.row == max_row - 1
            || self.col == max_col - 1
            || self.col == 0
        {
            return 0;
        }

        let right = {
            let mut view = 0_u32;
            for col in self.col + 1..max_col {
                if matrix[self.row][col].value >= self.value {
                    view += 1;
                    break;
                }
                view += 1;
            }
            view
        };
        let left = {
            let mut view = 0_u32;
            for col in (0..self.col).rev() {
                if matrix[self.row][col].value >= self.value {
                    view += 1;
                    break;
                }
                view += 1;
            }
            view
        };
        let down = {
            let mut view = 0_u32;
            for item in matrix.iter().take(max_row).skip(self.row + 1) {
                if item[self.col].value >= self.value {
                    view += 1;
                    break;
                }
                view += 1;
            }
            view
        };
        let up = {
            let mut view = 0_u32;
            for row in (0..self.row).rev() {
                if matrix[row][self.col].value >= self.value {
                    view += 1;
                    break;
                }
                view += 1;
            }
            view
        };
        up * right * left * down
    }
}

pub type Matrix = Vec<Vec<Tree>>;

pub fn scan_left2right(matrix: &mut Matrix) {
    let max_row: usize = matrix.len();
    let max_col: usize = matrix[0].len();
    for item in matrix.iter_mut().take(max_row - 1).skip(1) {
        let mut max_level: u8 = item[0].value;
        item[0].visible = true;
        for item in item.iter_mut().take(max_col - 1).skip(1) {
            if item.value > max_level {
                max_level = item.value;
                item.visible = true;
            }
        }
    }
}

pub fn scan_right2left(matrix: &mut Matrix) {
    let max_row: usize = matrix.len();
    let max_col: usize = matrix[0].len();
    (1..max_row - 1).rev().for_each(|row| {
        let mut max_level: u8 = matrix[row][max_col - 1].value;
        matrix[row][max_col - 1].visible = true;
        for col in (1..max_col - 1).rev() {
            if matrix[row][col].value > max_level {
                max_level = matrix[row][col].value;
                matrix[row][col].visible = true;
            }
        }
    });
}

pub fn scan_top2bottom(matrix: &mut Matrix) {
    let line_size: usize = matrix[0].len();
    let line_iter = matrix.iter_mut();

    let mut greater: Vec<u8> = Vec::new();
    greater.resize(line_size, 0);

    let mut first: bool = true;
    for line in line_iter {
        for col in 0..line_size {
            if first || line[col].value > greater[col] {
                line[col].visible = true;
                greater[col] = line[col].value
            }
        }
        first = false;
    }
}

pub fn scan_bottom2top(matrix: &mut Matrix) {
    let line_size: usize = matrix[0].len();
    let line_iter = matrix.iter_mut().rev();

    let mut greater: Vec<u8> = Vec::new();
    greater.resize(line_size, 0);

    let mut first: bool = true;
    for line in line_iter {
        for col in 0..line_size {
            if first || line[col].value > greater[col] {
                line[col].visible = true;
                greater[col] = line[col].value
            }
        }
        first = false;
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<Tree>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| Str2Tree!(row, line))
        .collect();

    scan_left2right(&mut matrix);
    scan_right2left(&mut matrix);
    scan_top2bottom(&mut matrix);
    scan_bottom2top(&mut matrix);
    Some(matrix.iter().fold(0, |acc, l| {
        acc + l
            .iter()
            .fold(0, |acc, v| if v.visible { acc + 1 } else { acc })
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut matrix: Vec<Vec<Tree>> = input
        .lines()
        .enumerate()
        .map(|(row, line)| Str2Tree!(row, line))
        .collect();

    scan_left2right(&mut matrix);
    scan_right2left(&mut matrix);
    scan_top2bottom(&mut matrix);
    scan_bottom2top(&mut matrix);
    Some(matrix.iter().fold(0, |acc, l| {
        cmp::max(
            acc,
            l.iter()
                .fold(0, |acc, v| cmp::max(v.scenic_score(&matrix), acc)),
        )
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}

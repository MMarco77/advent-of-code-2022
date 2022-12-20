/*
// Clone
(Release)
🎄 Part 1 🎄
5960 (elapsed: 3.05ms)
🎄 Part 2 🎄
2149 (elapsed: 12.16ms
(Debug)
🎄 Part 1 🎄
5960 (elapsed: 8.74ms)
🎄 Part 2 🎄
2327 (elapsed: 18.83ms)

// HashSet
(Release)
🎄 Part 1 🎄
5960 (elapsed: 1.21ms)
🎄 Part 2 🎄
2149 (elapsed: 2.17ms)
(Debug)
🎄 Part 1 🎄
5960 (elapsed: 8.64ms)
🎄 Part 2 🎄
2149 (elapsed: 10.35ms

// Vec
(Release)
🎄 Part 1 🎄
5960 (elapsed: 556.11µs)
🎄 Part 2 🎄
2149 (elapsed: 704.98µs)
(Debug)
🎄 Part 1 🎄
5960 (elapsed: 7.61ms)
🎄 Part 2 🎄
2149 (elapsed: 10.06ms)
*/

use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Pos {
    pub row: i32,
    pub col: i32,
}

struct Rope {
    head: Pos,
    tail_list: Vec<Pos>,
    historic: Vec<Pos>,
}

impl Rope {
    pub fn new(count: usize) -> Self {
        Rope {
            head: Pos { row: 0, col: 0 },
            tail_list: vec![Pos { row: 0, col: 0 }; count],
            historic: vec![Pos { row: 0, col: 0 }],
        }
    }

    fn update(head: &Pos, tail: &mut Pos) {
        if !(head.col.abs_diff(tail.col) <= 1 && head.row.abs_diff(tail.row) <= 1) {
            // Move
            match (head.col.abs_diff(tail.col), head.row.abs_diff(tail.row)) {
                // Diag
                (2, 2) | (1, 2) | (2, 1) => {
                    if head.col > tail.col {
                        tail.col += 1;
                    } else {
                        tail.col -= 1;
                    }
                    if head.row > tail.row {
                        tail.row += 1;
                    } else {
                        tail.row -= 1;
                    }
                }
                (0, 2) if head.row > tail.row => {
                    tail.row += 1;
                }
                (0, 2) if head.row < tail.row => {
                    tail.row -= 1;
                }
                (0, 2) => {}
                (2, 0) if head.col > tail.col => {
                    tail.col += 1;
                }
                (2, 0) if head.col < tail.col => {
                    tail.col -= 1;
                }
                _ => unreachable!(),
            };
        }
    }

    fn update_tail(&mut self) {
        let mut cur_head = &self.head;
        for cur_tail in self.tail_list.iter_mut() {
            Self::update(cur_head, cur_tail);
            cur_head = cur_tail;
        }

        self.historic
            .push(self.tail_list.iter().last().unwrap().clone());
    }

    pub fn move_up(&mut self, step: u8) {
        // println!("Move head up {step}");
        for _ in 0..step {
            self.head.row += 1;
            self.update_tail()
        }
    }

    pub fn move_down(&mut self, step: u8) {
        // println!("Move head down {step}");
        for _ in 0..step {
            self.head.row -= 1;
            self.update_tail()
        }
    }

    pub fn move_right(&mut self, step: u8) {
        // println!("Move head right {step}");
        for _ in 0..step {
            self.head.col += 1;
            self.update_tail()
        }
    }

    pub fn move_left(&mut self, step: u8) {
        // println!("Move head left {step}");
        for _ in 0..step {
            self.head.col -= 1;
            self.update_tail()
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut rope = Rope::new(1);
    input
        .lines()
        .enumerate()
        .map(|(c, l)| {
            eyes::try_parse!(l, "{} {}", String, u8).unwrap_or_else(|| panic!("Invalid line {c}"))
        })
        .for_each(|(cmd, step)| match cmd.chars().next() {
            Some('R') => rope.move_right(step),
            Some('U') => rope.move_up(step),
            Some('L') => rope.move_left(step),
            Some('D') => rope.move_down(step),
            _ => unreachable!(),
        });

    let uniq_pos: HashSet<Pos> = HashSet::from_iter(rope.historic);
    Some(uniq_pos.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::new(10);
    input
        .lines()
        .enumerate()
        .map(|(c, l)| {
            eyes::try_parse!(l, "{} {}", String, u8).unwrap_or_else(|| panic!("Invalid line {c}"))
        })
        .for_each(|(cmd, step)| match cmd.chars().next() {
            Some('R') => rope.move_right(step),
            Some('U') => rope.move_up(step),
            Some('L') => rope.move_left(step),
            Some('D') => rope.move_down(step),
            _ => unreachable!(),
        });

    let uniq_pos: HashSet<Pos> = HashSet::from_iter(rope.historic);
    Some(uniq_pos.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        //assert_eq!(part_two(&input), Some(36));
        assert_eq!(part_two(&input), Some(1));
    }
}

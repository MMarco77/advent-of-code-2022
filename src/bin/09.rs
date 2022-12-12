/*
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
    historic: HashSet<Pos>,
}

impl Rope {
    pub fn new(count: usize) -> Self {
        Rope {
            head: Pos { row: 0, col: 0 },
            tail_list: vec![Pos { row: 0, col: 0 }; count],
            historic: HashSet::from_iter([Pos { row: 0, col: 0 }]),
        }
    }

    fn update(&mut self, head: &Pos, tail: &Pos) -> Pos {
        let mut new_tail = tail.clone();
        if !(head.col.abs_diff(tail.col) <= 1 && head.row.abs_diff(tail.row) <= 1) {
            // Move
            match (head.col.abs_diff(new_tail.col), head.row.abs_diff(new_tail.row)) {
                // Diag
                (2, 2) | (1, 2) | (2, 1) => {
                    if head.col > new_tail.col {
                        new_tail.col += 1;
                    } else {
                        new_tail.col -= 1;
                    }
                    if head.row > new_tail.row {
                        new_tail.row += 1;
                    } else {
                        new_tail.row -= 1;
                    }
                }
                (0, 2) => {
                    if head.row > new_tail.row {
                        new_tail.row += 1;
                    } else if head.row < new_tail.row {
                        new_tail.row -= 1;
                    }
                }
                (2, 0) => {
                    if head.col > new_tail.col {
                        new_tail.col += 1;
                    } else if head.col < new_tail.col {
                        new_tail.col -= 1;
                    }
                }
                _ => unreachable!(),
            };
        }
        new_tail
    }

    fn update_tail(&mut self) {
        // let cur_head = self.head;
        // while let Some(cur_tail) = self.tail_list.iter().next() {
        //     self.update(&cur_head, &mut cur_tail);
        //     self.historic.insert(cur_tail.clone());
        //     cur_head = cur_tail;
        // }
        let new_tails: Vec<Pos> = self.tail_list.clone().iter().fold(Vec::new(), |acc, tail| {
            let head = if acc.len() == 0 {
                self.head.clone()
            } else {
                acc.iter().last().expect("Faild to collect las pos").clone()
            };

            // Compute new pos
            let mut new_acc = acc.clone();
            new_acc.push(
                self.update(
                    &head, 
                    &tail
                ).clone());
                new_acc
        });

        // Update historic
        self.historic.insert(new_tails.iter().last().unwrap().clone());
        
        // Update new _tail
        self.tail_list = new_tails;
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
        .map(|(c, l)| eyes::try_parse!(l, "{} {}", String, u8).expect(&format!("Invalid line {c}")))
        .for_each(|(cmd, step)| match cmd.chars().next() {
            Some('R') => rope.move_right(step),
            Some('U') => rope.move_up(step),
            Some('L') => rope.move_left(step),
            Some('D') => rope.move_down(step),
            _ => unreachable!(),
        });

    Some(rope.historic.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope::new(10);
    input
        .lines()
        .enumerate()
        .map(|(c, l)| eyes::try_parse!(l, "{} {}", String, u8).expect(&format!("Invalid line {c}")))
        .for_each(|(cmd, step)| match cmd.chars().next() {
            Some('R') => rope.move_right(step),
            Some('U') => rope.move_up(step),
            Some('L') => rope.move_left(step),
            Some('D') => rope.move_down(step),
            _ => unreachable!(),
        });

    Some(rope.historic.len() as u32)
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
        assert_eq!(part_two(&input), Some(36));
    }
}

use std::str::Lines;

use advent_of_code::error::AppResult;
use itertools::{Chunk, Itertools};

/*
Operation => new = old [+|*] [old|(0-9)*]
Test => divisible by [0-9]*
Always 'If true' before 'If false'
*/

#[derive(Debug)]
enum Operation {
    Add(u8),
    Mul(u8),
    Square,
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        if value.trim() == "Operation: new = old * old" {
            Operation::Square
        } else if let Some((op, step)) =
            eyes::try_parse!(value.trim(), "Operation: new = old {} {}", char, u8)
        {
            match op {
                '*' => Operation::Mul(step),
                '+' => Operation::Add(step),
                _ => unreachable!(),
            }
        } else {
            unreachable!()
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u32>,
    operation: Operation,
    test_divider: u8,
    test_true_dst: u8,
    test_false_dst: u8,
}

impl Monkey {
    pub fn from_chunk(monkey_desc: Chunk<Lines>) -> AppResult<Self> {
        let mut items: Vec<u32> = Vec::new();
        let mut operation: Operation = Operation::Add(0);
        let mut test_divider: u8 = 0;
        let mut test_true_dst: u8 = 0;
        let mut test_false_dst: u8 = 0;
        monkey_desc
            .into_iter()
            .enumerate()
            .for_each(|(line, data)| match line {
                0 => {}
                1 => items = Monkey::items_from_str(data),
                2 => operation = Operation::from(data),
                3 => {
                    test_divider = eyes::try_parse!(data.trim(), "Test: divisible by {}", u8)
                        .expect(&format!("Invalid test {}", data))
                }
                4 => {
                    test_true_dst = eyes::try_parse!(data.trim(), "If true: throw to monkey {}", u8)
                        .expect(&format!("Invalid test {}", data))
                }
                5 => {
                    test_false_dst =
                        eyes::try_parse!(data.trim(), "If false: throw to monkey {}", u8)
                            .expect(&format!("Invalid false test case {}", data))
                }
                6 => {}
                _ => unreachable!("Invalid Monkey description {}", line),
            });
        Ok(Self {
            items,
            operation,
            test_divider,
            test_true_dst,
            test_false_dst,
        })
    }

    fn items_from_str(line: &str) -> Vec<u32> {
        let list_str = eyes::try_parse!(line.trim(), "Starting items: {}", String)
            .expect(&format!("Invalid items {}", line));
        list_str
            .split(",")
            .map(|v| -> u32 {
                v.trim()
                    .parse::<u32>()
                    .expect(&format!("Invalid item value '{}'", v))
            })
            .collect::<Vec<_>>()
    }
}

fn process_round(_monkeys: &mut Vec<Monkey>) -> AppResult<()> {
    

    Ok(())
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut monkeys: Vec<Monkey> = Vec::default();
    for monkey_desc in &input.lines().chunks(7) {
        monkeys.push(Monkey::from_chunk(monkey_desc).expect("Invalid Monkey description"))
    }

    // Process round
    let mut stats: Vec<u32> = vec![0_u32; monkeys.len()];
    for _ in 1..=20 {
        // Update stats
        stats = monkeys
            .iter()
            .zip(stats.iter())
            .map(|(m, v)| m.items.len() as u32 + v)
            .collect::<Vec<_>>();

        process_round(&mut monkeys).unwrap()
    }

    stats.sort_by(|a, b| b.cmp(a));
    Some(stats.iter().take(2).product::<u32>())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}

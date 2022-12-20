/*
(Debug)
🎄 Part 1 🎄
TPGVQPFDH (elapsed: 1.34ms)
🎄 Part 2 🎄
DMRDFRHHH (elapsed: 1.39ms)

(Release)
🎄 Part 1 🎄
TPGVQPFDH (elapsed: 150.45µs)
🎄 Part 2 🎄
DMRDFRHHH (elapsed: 156.95µs)
*/

use std::str::Lines;

use advent_of_code::error::{AppError, AppResult};

type Stack = Vec<char>;

fn pull_reverse(src: &mut Stack, count: u8) -> AppResult<Stack> {
    let mut acc: Vec<char> = Vec::new();
    for _ in 0..count {
        acc.push(src.pop().unwrap())
    }
    Ok(acc)
}

fn pull(src: &mut Stack, count: u8) -> AppResult<Stack> {
    let mut acc: Vec<char> = Vec::new();
    for _ in 0..count {
        acc.insert(0, src.pop().unwrap())
    }
    Ok(acc)
}

fn push(dst: &mut Stack, crate_list: &mut Stack) {
    dst.append(crate_list);
}

fn get_stacks(lines_iter: &mut Lines) -> AppResult<Vec<Stack>> {
    let mut stacks: Vec<Stack> = Vec::new();
    for (nbr, line) in lines_iter.enumerate() {
        let max_col = (line.len() + 1) / 4;
        if stacks.is_empty() {
            stacks.resize(max_col, Vec::new());
        }
        for col in 0..max_col {
            let cur_crate = line
                .chars()
                .nth(col * 4 + 1)
                .unwrap_or_else(|| panic!("Invalid string for {} [{}]", line, nbr));
            match cur_crate {
                a if a.is_alphabetic() => {
                    stacks[col].insert(0, a);
                }
                ' ' => {}
                _ => return Ok(stacks),
            }
        }
    }
    Err(AppError::General("NOOP".to_string()))
}

pub fn part_one(input: &str) -> Option<String> {
    let mut lines_iter = input.lines();
    let mut supply_stacks = get_stacks(&mut lines_iter).expect("Failed to collect Stacks");

    // Pass carriage return
    lines_iter.next();

    // Parse order
    lines_iter.for_each(|line| {
        let (count, src, dst) = eyes::try_parse!(line, "move {} from {} to {}", u8, u8, u8)
            .unwrap_or_else(|| panic!("Invalide cmd line '{line}'"));
        let mut stack_path = pull_reverse(&mut supply_stacks[src as usize - 1], count).unwrap();
        push(&mut supply_stacks[dst as usize - 1], &mut stack_path);
    });

    Some(String::from_iter(
        supply_stacks
            .iter()
            .map(|s| s.last().unwrap())
            .collect::<Vec<_>>(),
    ))
}

pub fn part_two(input: &str) -> Option<String> {
    let mut lines_iter = input.lines();
    let mut supply_stacks = get_stacks(&mut lines_iter).expect("Failed to collect Stacks");

    // Pass carriage return
    lines_iter.next();

    // Parse order
    lines_iter.for_each(|line| {
        let (count, src, dst) = eyes::try_parse!(line, "move {} from {} to {}", u8, u8, u8)
            .unwrap_or_else(|| panic!("Invalide cmd line '{line}'"));
        let mut stack_path = pull(&mut supply_stacks[src as usize - 1], count).unwrap();
        push(&mut supply_stacks[dst as usize - 1], &mut stack_path);
    });

    Some(String::from_iter(
        supply_stacks
            .iter()
            .filter_map(|s| s.last())
            .collect::<Vec<_>>(),
    ))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_owned()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some("MCD".to_owned()));
    }
}

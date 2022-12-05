/*
With eyes
---------
🎄 Part 1 🎄
534 (elapsed: 38.88ms)
🎄 Part 2 🎄
841 (elapsed: 36.19ms)

No eyes
-------
🎄 Part 1 🎄
534 (elapsed: 38.38ms)
🎄 Part 2 🎄
841 (elapsed: 33.31ms)
*/

use std::collections::HashSet;

pub fn parse_line(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .flat_map(|spl| spl.split('-'))
        .map(|n| n.parse::<u8>().expect("Failed to parse input"))
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().enumerate().fold(0, |acc, (nbr, line)| {
        // Eyes
        let (ll, lu, rl, ru) = eyes::try_parse!(line, "{}-{},{}-{}", u8, u8, u8, u8)
            .unwrap_or_else(|| panic!("Invalid line {} [{}]", line, nbr));
        // No eyes
        //let (ll, lu, rl, ru) = if let Ok([ll, lu, rl, ru]) = parse_line(line).try_into() { (ll, lu, rl, ru) } else { todo!() };
        let rg1: HashSet<u8> = HashSet::from_iter(ll..=lu);
        let rg2: HashSet<u8> = HashSet::from_iter(rl..=ru);
        if rg1.is_subset(&rg2) || rg1.is_superset(&rg2) {
            acc + 1
        } else {
            acc
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().enumerate().fold(0, |acc, (nbr, line)| {
        // Eyes
        let (ll, lu, rl, ru) = eyes::try_parse!(line, "{}-{},{}-{}", u8, u8, u8, u8)
            .unwrap_or_else(|| panic!("Invalid line {} [{}]", line, nbr));
        // No eyes
        //let (ll, lu, rl, ru) = if let Ok([ll, lu, rl, ru]) = parse_line(line).try_into() { (ll, lu, rl, ru) } else { todo!() };

        let rg1: HashSet<u8> = HashSet::from_iter(ll..=lu);
        let rg2: HashSet<u8> = HashSet::from_iter(rl..=ru);
        if rg1.is_disjoint(&rg2) && rg1.is_disjoint(&rg2) {
            acc
        } else {
            acc + 1
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}

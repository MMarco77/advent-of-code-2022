use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let score: HashMap<String, u32> = HashMap::from_iter([
        ("X".to_owned(), 1),
        ("Y".to_owned(), 2),
        ("Z".to_owned(), 3),
    ]);
    Some(input.lines().fold(0, |acc, line| {
        if let Some((dl, dr)) = eyes::try_parse!(line, "{} {}", String, String) {
            let weight_r = score.get(&dr).unwrap();
            match (dl.as_str(), dr.as_str()) {
                ("A", "Z") | ("B", "X") | ("C", "Y") => acc + weight_r,
                ("A", "Y") | ("B", "Z") | ("C", "X") => acc + 6 + weight_r,
                _ => acc + 3 + weight_r,
            }
        } else {
            0
        }
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    let score: HashMap<String, u32> = HashMap::from_iter([
        ("A".to_owned(), 1),
        ("B".to_owned(), 2),
        ("C".to_owned(), 3)
    ]);
    Some(input.lines().fold(0, |acc, line| {
        if let Some((dl, dr)) = eyes::try_parse!(line, "{} {}", String, String) {
            match (dl.as_str(), dr.as_str()) {
                // Null
                ("A", "Y") | ("B", "Y") | ("C", "Y") => acc + 3 + score.get(&dl).unwrap(),
                // Lost
                ("A", "X") => acc + score.get("C").unwrap(),
                ("B", "X") => acc + score.get("A").unwrap(),
                ("C", "X") => acc + score.get("B").unwrap(),
                // Won
                ("A", "Z") => acc + 6 + score.get("B").unwrap(),
                ("B", "Z") => acc + 6 + score.get("C").unwrap(),
                ("C", "Z") => acc + 6 + score.get("A").unwrap(),
                _ => acc,
            }
        } else {
            0
        }
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

use std::collections::{HashMap, HashSet};

// macro_rules! map_alphabet {
//     ($lower:expr, $upper:expr, $delta:expr) => {
//         HashMap::from_iter(
//             ($lower..=$upper)
//                 .map(|c| c as char)
//                 .filter(|c| c.is_alphabetic())
//                 .collect::<Vec<_>>()
//                 .iter()
//                 .enumerate()
//                 .map(|(i, c)| ((i + $shift) as u32, c.clone()))
//                 .collect::<Vec<_>>(),
//         )
//     };
// }

fn get_alphabetic_map() -> HashMap<char, u32> {
    //let mut mapping: map_alphabet!(b'A', b'Z', 27);
    // let mapping_l: map_alphabet!((b'a'..=b'z'), 1);
    // let mapping = mapping_l.iter().map(|(k, v)| mapping.insert(k, v));
    let mut map1 = HashMap::from_iter(
        (b'A'..=b'Z')
            .map(|c| c as char)
            .filter(|c| c.is_alphabetic())
            .collect::<Vec<_>>()
            .iter()
            .enumerate()
            .map(|(i, c)| (*c, (i + 27) as u32))
            .collect::<Vec<_>>(),
    );
    (b'a'..=b'z')
        .map(|c| c as char)
        .filter(|c| c.is_alphabetic())
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .for_each(|(i, c)| {
            map1.insert(*c, (i + 1) as u32);
        });
    map1
}

pub fn part_one(input: &str) -> Option<u32> {
    let mapping = get_alphabetic_map();
    Some(input.lines().fold(0, |acc, line| {
        let left: HashSet<char> = line.chars().take(line.len() / 2).collect();
        let right: HashSet<char> = line.chars().skip(line.len() / 2).collect();
        left.intersection(&right)
            .into_iter()
            .take(1)
            .map(|f| acc + mapping.get(f).unwrap())
            .collect::<Vec<_>>()[0]
    }))
}

fn get_common_letter(grouped: &[&str]) -> char {
    let first: HashSet<char> = grouped[0].chars().collect();
    let second: HashSet<char> = grouped[1].chars().collect();
    let third: HashSet<char> = grouped[2].chars().collect();
    *first
        .intersection(&second)
        .copied()
        .collect::<HashSet<_>>()
        .intersection(&third)
        .into_iter()
        .take(1)
        .collect::<Vec<_>>()[0]
}

pub fn part_two(input: &str) -> Option<u32> {
    let mapping = get_alphabetic_map();
    let mut grouped: Vec<&str> = Vec::new();

    let result = input.lines().enumerate().fold(0, |acc, (nbr, line)| {
        let mut new_acc = acc;
        if nbr % 3 == 0 && nbr != 0 {
            let common_char = get_common_letter(&grouped);
            grouped.clear();
            new_acc = acc + mapping.get(&common_char).unwrap()
        }

        grouped.push(line);
        new_acc
    });
    Some(if !grouped.is_empty() {
        result + mapping.get(&get_common_letter(&grouped)).unwrap()
    } else {
        result
    })
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    #[ignore]
    fn test_part_one_with_input() {
        let input = advent_of_code::read_file("inputs", 3);
        assert_eq!(part_one(&input), Some(7845));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}

use std::collections::HashSet;

fn get_uniq_chunk_index(line: &str, chunk_size: usize) -> Option<u32>{
    for (count, chunk) in line.chars().collect::<Vec<char>>().windows(chunk_size).enumerate() {
        let chunk_set: HashSet<char> = HashSet::from_iter(chunk.iter().cloned());
        if chunk_set.len() == chunk_size {
            return Some(count as u32 + chunk_size as u32)
        }
    }
    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let line: &str = input.lines().next().unwrap();
    get_uniq_chunk_index(line, 4)
}

pub fn part_two(input: &str) -> Option<u32> {
    let line: &str = input.lines().next().unwrap();
    get_uniq_chunk_index(line, 14)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), Some(7));
    }

    #[test]
    fn test_part_one_bis() {
        let inputs: Vec<String> = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_owned(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned(),
        ];
        for (idx, input) in inputs.iter().enumerate() {
            match idx {
                0 => assert_eq!(part_one(&input), Some(5)),
                1 => assert_eq!(part_one(&input), Some(6)),
                2 => assert_eq!(part_one(&input), Some(10)),
                3 => assert_eq!(part_one(&input), Some(11)),
                _ => unreachable!("Abnormal index"),
            }
        }
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), Some(19));
    }

    #[test]
    fn test_part_two_bis() {
        let inputs: Vec<String> = vec![
            "bvwbjplbgvbhsrlpgdmjqwftvncz".to_owned(),
            "nppdvjthqldpwncqszvftbrmjlhg".to_owned(),
            "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_owned(),
            "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_owned(),
        ];
        for (idx, input) in inputs.iter().enumerate() {
            match idx {
                0 => assert_eq!(part_two(&input), Some(23)),
                1 => assert_eq!(part_two(&input), Some(23)),
                2 => assert_eq!(part_two(&input), Some(29)),
                3 => assert_eq!(part_two(&input), Some(26)),
                _ => unreachable!("Abnormal index"),
            }
        }
    }
    
}

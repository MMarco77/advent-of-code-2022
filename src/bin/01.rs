pub fn part_one(input: &str) -> Option<u32> {
    let mut biggest = 0;
    let final_count = input.lines().fold(0, |acc, line| -> u32 {
        if line.is_empty() {
            if acc > biggest {
                biggest = acc;
            }
            0
        } else {
            acc + line.parse::<u32>().unwrap()
        }
    });
    Some(if biggest > final_count {
        biggest
    } else {
        final_count
    })
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut biggest: Vec<u32> = Vec::new();
    let last_count = input.lines().fold(0, |acc, line| -> u32 {
        if line.is_empty() {
            biggest.push(acc);
            biggest.sort_by(|a, b| b.cmp(a));
            biggest.resize(3, 0);
            0
        } else {
            acc + line.parse::<u32>().unwrap()
        }
    });

    biggest.push(last_count);
    biggest.sort_by(|a, b| b.cmp(a));
    biggest.resize(3, 0);

    Some(biggest.iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    #[ignore]
    fn test_part_one_offical() {
        let input = advent_of_code::read_file("inputs", 1);
        assert_eq!(part_one(&input), Some(68787));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }

    #[test]
    #[ignore]
    fn test_part_two_official() {
        let input = advent_of_code::read_file("inputs", 1);
        assert_eq!(part_two(&input), Some(198041));
    }
}

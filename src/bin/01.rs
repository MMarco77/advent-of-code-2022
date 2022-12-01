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
    let mut biggest: [u32; 3] = [0; 3];
    let mut last_count = input.lines().fold(0, |acc, line| -> u32 {
        if line.is_empty() {
            let mut acc_bis = acc;
            if acc_bis > biggest[2] {
                std::mem::swap(&mut biggest[2], &mut acc_bis);
            }
            if acc_bis > biggest[1] {
                std::mem::swap(&mut biggest[1], &mut acc_bis);
            }
            if acc_bis > biggest[0] {
                biggest[0] = acc_bis;
            }
            0
        } else {
            acc + line.parse::<u32>().unwrap()
        }
    });

    if last_count > biggest[2] {
        std::mem::swap(&mut biggest[2], &mut last_count);
    }
    if last_count > biggest[1] {
        std::mem::swap(&mut biggest[1], &mut last_count);
    }
    if last_count > biggest[0] {
        biggest[0] = last_count;
    }

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
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}

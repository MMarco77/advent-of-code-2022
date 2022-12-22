fn mixing(input: &str, decryption_key: i64, loop_count: u8) -> Option<i64> {
    let nums = input
        .lines()
        .map(|l| {
            decryption_key
                * l.parse::<i64>()
                    .unwrap_or_else(|_| panic!("Invalid value '{}'", l))
        })
        .collect::<Vec<_>>();

    // indexes into nums
    let mut mixed: Vec<_> = (0..nums.len()).collect();
    (0..loop_count).for_each(|_| {
        for (idx, &num) in nums.iter().enumerate() {
            // find mixed that corresponds to the number in nums
            let mixed_idx = mixed.iter().position(|&mix_num| mix_num == idx).unwrap();
            // remove that index from mixed
            mixed.remove(mixed_idx);
            // add num offset to that number and add it back
            let new_mixed_idx = (mixed_idx as i64 + num).rem_euclid(mixed.len() as i64) as usize;
            mixed.insert(new_mixed_idx, idx);
        }
    });

    let zero_idx = nums.iter().position(|&num| num == 0).unwrap();
    let zero_mixed_idx = mixed
        .iter()
        .position(|&mix_num| mix_num == zero_idx)
        .unwrap();

    Some(
        [1000, 2000, 3000]
            .iter()
            .map(|offset| {
                let mixed_idx = (zero_mixed_idx + offset) % mixed.len();
                let nums_idx = mixed[mixed_idx];
                nums[nums_idx]
            })
            .sum(),
    )
}

pub fn part_one(input: &str) -> Option<i64> {
    mixing(input, 1, 1)
}

pub fn part_two(input: &str) -> Option<i64> {
    mixing(input, 811_589_153, 10)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 20);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_one(&input), Some(3));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 20);
        assert_eq!(part_two(&input), Some(1623178306));
    }
}

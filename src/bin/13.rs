use itertools::Itertools;

fn is_right_order(left: &str, right: &str) -> Option<u32> {
    let mut left_iter = left.chars();
    let mut right_iter = right.chars();
    let mut lc = left_iter.next();
    let mut rc = right_iter.next();
    loop {
        match (lc, rc) {
            (Some(']'), Some(']')) | (Some('['), Some('[')) | (Some(','), Some(',')) => {
                lc = left_iter.next();
                rc = right_iter.next();
            }
            (Some(a), Some(b)) if a.is_numeric() && b.is_numeric() => {
                let mut left_nbr = format!("{a}");
                while let Some(c) = left_iter.next() {
                    if c.is_numeric() {
                        left_nbr += &c.to_string();
                    } else {
                        lc = Some(c);
                        break;
                    }
                }

                let mut right_nbr = format!("{b}");
                while let Some(c) = right_iter.next() {
                    if c.is_numeric() {
                        right_nbr += &c.to_string();
                    } else {
                        rc = Some(c);
                        break;
                    }
                }

                let lnbr = left_nbr
                    .parse::<u8>()
                    .expect(&format!("Waitng for data '{}'", left));
                let rnbr = right_nbr
                    .parse::<u8>()
                    .expect(&format!("Waitng for data '{}'", right));
                if lnbr > rnbr {
                    return Some(0);
                } else if rnbr > lnbr {
                    return Some(1);
                }
            }
            // Solo nbr to array
            (Some(a), Some('[')) if a.is_numeric() => {
                let mut new_left = format!("[{a}");
                while let Some(c) = left_iter.next() {
                    if c.is_numeric() {
                        new_left += &c.to_string();
                    } else {
                        lc = Some(c);
                        break;
                    }
                }
                new_left += "]";

                let mut new_right = "[".to_owned();
                while let Some(c) = right_iter.next() {
                    new_right += &c.to_string();
                    if c == ']' {
                        break;
                    }
                }
                if let Some(v) = is_right_order(&new_left, &new_right) {
                    return Some(v);
                }
                rc = right_iter.next();
            }
            (Some('['), Some(b)) if b.is_numeric() => {
                let mut new_right = format!("[{b}");
                while let Some(c) = right_iter.next() {
                    if c.is_numeric() {
                        new_right += &c.to_string();
                    } else {
                        rc = Some(c);
                        break;
                    }
                }
                new_right += "]";

                let mut new_left = "[".to_owned();
                let mut sub_par = 0_u8;
                while let Some(c) = left_iter.next() {
                    new_left += &c.to_string();
                    if c == ']' && sub_par == 0 {
                        break;
                    } else if c == ']' {
                        sub_par -= 1;
                    } else if c == '[' {
                        sub_par += 1;
                    }
                }
                if let Some(v) = is_right_order(&new_left, &new_right) {
                    return Some(v);
                }
                lc = left_iter.next();
            }
            // Close array first
            (Some(_), Some(']')) => return Some(0),
            (Some(']'), Some(_)) => return Some(1),
            // Too short
            (None, Some(_)) => return Some(1),
            (Some(_), None) => return Some(0),
            // Same len
            (None, None) => return None,
            // Against rust compiler
            (Some(_), Some(_)) => unreachable!("Not data => '{:#?}' '{:#?}'", lc, rc),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut accu = 0_u32;
    let mut idx = 1_u32;
    for chunk in &input.lines().chunks(3) {
        let mut pair_iter = chunk.into_iter();
        let left = pair_iter.next().expect("Missing left value");
        let right = pair_iter.next().expect("Missing right value");

        println!("Line {}", 3 * (idx - 1));
        if let Some(v) = is_right_order(left, right) {
            accu += idx * v;
        }
        idx += 1;
    }
    Some(accu)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 13);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 13);
        assert_eq!(part_two(&input), None);
    }
}

/*
(Debug)
5825 (elapsed: 2.73ms)
🎄 Part 2 🎄
not solved.
 */

use std::{cmp::Ordering, fmt};

use itertools::Itertools;

#[derive(Debug, PartialEq, Eq, Clone)]
enum Token {
    OpenParenthesis(u8),
    CloseParenthesis(u8),
    Number(u8),
    Coma,
}

fn token_eq_not_num(a: &Token, b: &Token) -> bool {
    matches!(
        (a, b),
        (&Token::OpenParenthesis(..), &Token::OpenParenthesis(..))
            | (&Token::CloseParenthesis(..), &Token::CloseParenthesis(..))
            | (&Token::Coma, &Token::Coma)
    )
}

#[derive(Debug, PartialEq, Clone)]
struct Packets {
    pub data: Vec<Token>,
}

impl fmt::Display for Packets {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut acc = String::new();
        self.data.iter().for_each(|token| match token {
            Token::OpenParenthesis(_) => acc += "[",
            Token::CloseParenthesis(_) => acc += "]",
            Token::Number(v) => acc += &format!("{}", v),
            Token::Coma => acc += "]",
        });
        f.write_str(acc.as_ref())
    }
}

impl From<&str> for Packets {
    fn from(value: &str) -> Self {
        let mut res: Vec<Token> = Vec::new();
        let mut acc: String = String::new();
        let mut level: u8 = 0;
        value.chars().for_each(|c| match c {
            '[' => {
                if !acc.is_empty() {
                    res.push(Token::Number(acc.parse::<u8>().unwrap()));
                    acc.clear();
                }
                res.push(Token::OpenParenthesis({
                    let tmp = level;
                    level += 1;
                    tmp
                }));
            }
            ']' => {
                if !acc.is_empty() {
                    res.push(Token::Number(acc.parse::<u8>().unwrap()));

                    acc.clear();
                }
                res.push(Token::CloseParenthesis({
                    level -= 1;
                    level
                }));
            }
            ',' => {
                if !acc.is_empty() {
                    res.push(Token::Number(acc.parse::<u8>().unwrap()));

                    acc.clear();
                }
                res.push(Token::Coma);
            }
            d if d.is_numeric() => acc += &d.to_string(),
            _ => unreachable!(),
        });
        if !acc.is_empty() {
            panic!("Invalid format")
        };

        Self { data: res }
    }
}

fn token_cmp(left: &[Token], right: &[Token]) -> Ordering {
    let mut left_iter = left.iter();
    let mut right_iter = right.iter();
    let mut left_tok = left_iter.next();
    let mut right_tok = right_iter.next();
    loop {
        match (left_tok, right_tok) {
            (Some(l), Some(r)) if token_eq_not_num(l, r) => {
                left_tok = left_iter.next();
                right_tok = right_iter.next();
            }
            (Some(Token::Number(l)), Some(Token::Number(r))) => {
                if l > r {
                    return Ordering::Greater;
                }
                if r > l {
                    return Ordering::Less;
                }
                left_tok = left_iter.next();
                right_tok = right_iter.next();
            }
            (Some(Token::OpenParenthesis(lvl)), Some(Token::Number(v))) => {
                let mut new_left: Vec<Token> = vec![Token::OpenParenthesis(*lvl)];
                for tok in left_iter.by_ref() {
                    new_left.push(tok.clone());
                    if matches!(tok, &Token::CloseParenthesis(l) if l == *lvl) {
                        break;
                    }
                }

                let new_right: Vec<Token> = vec![
                    Token::OpenParenthesis(0),
                    Token::Number(*v),
                    Token::CloseParenthesis(0),
                ];
                let res = token_cmp(&new_left, &new_right);
                match res {
                    Ordering::Less | Ordering::Greater => return res,
                    Ordering::Equal => {}
                }
                left_tok = left_iter.next();
                right_tok = right_iter.next();
            }
            (Some(Token::Number(v)), Some(Token::OpenParenthesis(lvl))) => {
                let new_left: Vec<Token> = vec![
                    Token::OpenParenthesis(0),
                    Token::Number(*v),
                    Token::CloseParenthesis(0),
                ];

                let mut new_right: Vec<Token> = vec![Token::OpenParenthesis(*lvl)];
                for tok in right_iter.by_ref() {
                    new_right.push(tok.clone());
                    if matches!(tok, &Token::CloseParenthesis(l) if l == *lvl) {
                        break;
                    }
                }

                let res = token_cmp(&new_left, &new_right);
                match res {
                    Ordering::Less | Ordering::Greater => return res,
                    Ordering::Equal => {}
                }
                left_tok = left_iter.next();
                right_tok = right_iter.next();
            }
            // Close array first
            (Some(_), Some(Token::CloseParenthesis(..))) => return Ordering::Greater,
            (Some(Token::CloseParenthesis(..)), Some(_)) => return Ordering::Less,
            // Too short
            (None, None) => return Ordering::Equal,
            (None, Some(_)) => return Ordering::Less,
            (Some(_), None) => return Ordering::Greater,
            (Some(l), Some(r)) => unreachable!("Not data => '{:#?}' '{:#?}'", l, r),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut packets_counter = 1_u32;
    let mut res = 0_u32;
    for chunk in &input.lines().chunks(3) {
        let mut iter_line = chunk;
        let line = iter_line.next().expect("Left packet is missing");
        let left = Packets::from(line);
        // println!("{} => {:#?}", line, left);

        let line = iter_line.next().expect("Right packet is missing");
        let right = Packets::from(line);
        // println!("{} => {:#?}", line, right);

        if let Ordering::Less = token_cmp(&left.data, &right.data) {
            res += packets_counter;
        }
        packets_counter += 1;
    }
    Some(res)
}

pub fn part_two(input: &str) -> Option<u32> {
    let decode_key_1 = &Packets::from("[[2]]");
    let decode_key_2 = &Packets::from("[[6]]");
    let mut stream_list: Vec<Packets> = vec![decode_key_1.clone(), decode_key_2.clone()];
    input.lines().for_each(|line| {
        if !line.is_empty() {
            stream_list.push(Packets::from(line));
        }
    });

    stream_list.sort_by(|a, b| token_cmp(&a.data, &b.data));
    // stream_list.iter().for_each(|token| println!("{}", token));

    // Some(stream_list.iter().enumerate().fold(1, |acc, (pos, tok)| {
    //     if tok == decode_key_1 || tok == decode_key_2 {
    //         acc * (pos as u32 + 1)
    //     } else {
    //         acc
    //     }
    // }))
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
        //assert_eq!(part_two(&input), Some(140));
        assert_eq!(part_two(&input), None);
    }
}

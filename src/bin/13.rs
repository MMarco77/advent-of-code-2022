/*
(Debug)
5825 (elapsed: 2.73ms)
🎄 Part 2 🎄
not solved.
 */

use std::cmp::Ordering;

use itertools::Itertools;

#[derive(Debug, PartialEq, Clone, Eq)]
enum Packet {
    Num(u8),
    List(Vec<Packet>),
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Self::List(a), Self::List(b)) => a.cmp(b),
            (Self::List(a), Self::Num(b)) => a.cmp(&vec![Self::Num(*b)]),
            (Self::Num(a), Self::List(b)) => vec![Self::Num(*a)].cmp(b),
            (Self::Num(a), Self::Num(b)) => a.cmp(b),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse(input: &str) -> Vec<[Packet; 2]> {
    input
        .split("\n\n")
        .map(|pair| {
            let mut lines = pair.lines();
            let left = lines.next().unwrap();
            let right = lines.next().unwrap();

            [parse_packet(left), parse_packet(right)]
        })
        .collect()
}

fn parse_packet(s: &str) -> Packet {
    let chars: Vec<_> = s.chars().collect();
    // parse_list returns the first parsed packet and the rest of the input
    // the rest of the input will be empty when it is done
    let (packet, _) = parse_list(&chars);
    packet
}

fn parse_num(list: &[char]) -> (Packet, &[char]) {
    // find the index where the first number ends
    let mut i = 0;
    while i < list.len() && list[i].is_ascii_digit() {
        i += 1;
    }

    // parse the number
    // uses math to concatenate numbers instead of turning them into a string first to parse that
    let mut num = 0;
    let mut offset = 1;
    for c in list[..i].iter().rev() {
        num += (*c as u8 - b'0') * offset;
        offset *= 10;
    }

    // return the number and the rest of the packet
    (Packet::Num(num), &list[i..])
}

fn parse_list(list: &[char]) -> (Packet, &[char]) {
    // start by removing the starting [ of the passed in list
    // at the end of this function, we remove the ending ] of the passed in list
    let mut list = &list[1..];
    let mut packets = Vec::new();

    loop {
        match list[0] {
            // list ended, break the loop
            ']' => break,
            // skip over ,
            ',' => list = &list[1..],
            // beginning of new list, time for recursion to parse the inner list
            '[' => {
                let (packet, rest) = parse_list(list);
                packets.push(packet);
                list = rest;
            }
            // beginning of a number
            _ => {
                let (n, rest) = parse_num(list);
                packets.push(n);
                list = rest;
            }
        }
    }

    // return the parsed list and the remaining characters minus the ] that terminates the list this just parsed
    (Packet::List(packets), &list[1..])
}

pub fn part_one(input: &str) -> Option<usize> {
    let pairs = parse(input);

    Some(
        pairs
            .iter()
            .positions(|[left, right]| left < right)
            .map(|idx| idx + 1)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let pairs = parse(input);
    let mut packets: Vec<_> = pairs.iter().flatten().collect();

    let divider_1 = parse_packet("[[2]]");
    let divider_2 = parse_packet("[[6]]");

    packets.push(&divider_1);
    packets.push(&divider_2);

    packets.sort_unstable();

    Some(
        packets
            .into_iter()
            .positions(|packet| packet == &divider_1 || packet == &divider_2)
            .map(|idx| idx + 1)
            .product(),
    )
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
        assert_eq!(part_two(&input), Some(140));
    }
}

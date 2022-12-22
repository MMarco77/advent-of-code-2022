#[derive(Debug, PartialEq, Clone, Eq)]
enum Cmd {
    Num(u8),
    Clockwise,
    CounterClockwise,
}

fn get_command_list(input: &str) -> Vec<Cmd> {
    let mut cmd: Vec<Cmd> = Vec::new();
    let mut iter_char = input.chars();
    let mut new_char = iter_char.next();
    while new_char.is_some() {
        match new_char {
            Some('R') => {
                cmd.push(Cmd::Clockwise);
                new_char = iter_char.next();
            }
            Some('L') => {
                cmd.push(Cmd::CounterClockwise);
                new_char = iter_char.next();
            }
            Some(a) if a.is_ascii_digit() => {
                let mut acc: String = String::new();
                acc += &a.to_string();
                iter_char.next();
                while new_char.is_some() && new_char.unwrap().is_ascii_digit() {
                    acc += &new_char.unwrap().to_string();
                }
                cmd.push(Cmd::Num(
                    acc.parse::<u8>()
                        .expect(&format!("Invalid value {} for tile move", acc)),
                ));
            }
            _ => unreachable!("Invalid command '{:#?}'", new_char),
        }
    }
    cmd
}

enum Facing {
    Right, // 0 | >
    Down,  // 1 | v
    Left,  // 2 | <
    Up,    // 3 | ^
}

#[derive(Debug, PartialEq, Clone, Eq)]
enum MapItem {
    OpenTile,
    SolidWall,
    Nil,
}

impl From<char> for MapItem {
    fn from(value: char) -> Self {
        match value {
            '.' => MapItem::OpenTile,
            '#' => MapItem::SolidWall,
            _ => MapItem::Nil,
        }
    }
}

struct TheGrid {
    row_max: usize,
    col_max: usize,
    map: Vec<Vec<MapItem>>,
}

impl TheGrid {
    fn new(input: &str) -> Self {
        let mut col_max = 0_usize;
        let mut row_max = 0_usize;
        let mut map: Vec<Vec<MapItem>> = Vec::new();
        for line in input.lines() {
            let new_line = line.chars().map(|c| MapItem::from(c)).collect::<Vec<_>>();
            col_max = col_max.max(new_line.len());
            map.push(new_line);
        }
        row_max = map.len();

        // Update
        map.iter_mut()
            .for_each(|row| row.resize(col_max, MapItem::Nil));

        Self {
            row_max,
            col_max,
            map,
        }
    }
}

struct Player {
    col: u32,
    row: u32,
    facing: Facing,
}

pub fn part_one(input: &str) -> Option<u32> {
    let (grid_str, commands) = input.split_once("\n\n").expect("Invalid input");

    let grid = TheGrid::new(grid_str);

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 22);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_one(&input), Some(6032));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 22);
        assert_eq!(part_two(&input), None);
    }
}

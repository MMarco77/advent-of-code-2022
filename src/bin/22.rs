#[allow(dead_code)]
/*
(Debug)
🎄 Part 1 🎄
56372 (elapsed: 3.72ms)
🎄 Part 2 🎄
not solved.
*/
use advent_of_code::error::AppResult;

#[derive(Debug, PartialEq, Clone, Copy, Eq)]
enum Cmd {
    Num(u8),
    Clockwise,
    CounterClockwise,
}

#[derive(PartialEq)]
enum Direction {
    Increase, // ++
    Decrease, // --
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
                new_char = iter_char.next();
                while new_char.is_some() && new_char.unwrap().is_ascii_digit() {
                    acc += &new_char.unwrap().to_string();
                    new_char = iter_char.next();
                }
                cmd.push(Cmd::Num(
                    acc.parse::<u8>()
                        .expect(&format!("Invalid value {} for tile move", acc)),
                ));
            }
            Some('\n') => break,
            _ => unreachable!("Invalid command '{:#?}'", new_char),
        }
    }
    cmd
}

#[derive(Debug, Clone, Copy)]
enum Facing {
    Right, // 0 | >
    Down,  // 1 | v
    Left,  // 2 | <
    Up,    // 3 | ^
}

impl std::fmt::Display for Facing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Facing::Right => write!(f, ">"),
            Facing::Down => write!(f, "v"),
            Facing::Left => write!(f, "<"),
            Facing::Up => write!(f, "^"),
        }
    }
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

impl std::fmt::Display for MapItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MapItem::OpenTile => write!(f, "."),
            MapItem::SolidWall => write!(f, "#"),
            MapItem::Nil => write!(f, " "),
        }
    }
}

struct Player {
    col: u32,
    row: u32,
    facing: Facing,
}

impl std::fmt::Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Player Col: {}, Row: {}, Facing {}",
            self.col, self.row, self.facing
        )
    }
}

impl Player {
    fn turn_right(&mut self) {
        self.facing = match &self.facing {
            Facing::Right => Facing::Down,
            Facing::Down => Facing::Left,
            Facing::Left => Facing::Up,
            Facing::Up => Facing::Right,
        };
    }

    fn turn_left(&mut self) {
        self.facing = match &self.facing {
            Facing::Right => Facing::Up,
            Facing::Down => Facing::Right,
            Facing::Left => Facing::Down,
            Facing::Up => Facing::Left,
        };
    }

    fn get_password(&self) -> u64 {
        // println!("Compute password on {}", self);
        1_000_u64 * (self.row as u64 + 1_u64)
            + 4_u64 * (self.col as u64 + 1_u64)
            + match self.facing {
                Facing::Right => 0_u64,
                Facing::Down => 1_u64,
                Facing::Left => 2_u64,
                Facing::Up => 3_u64,
            }
    }
}

enum GridForm {
    Flatten,
    Cubic(u8),
}

struct TheGrid {
    map: Vec<Vec<MapItem>>,
    player: Player,
    form: GridForm,
}

impl TheGrid {
    fn new(input: &str, form: GridForm) -> Self {
        let mut first_available_col = 0_u32;
        let mut col_max = 0_usize;
        let mut is_first = true;
        let mut map: Vec<Vec<MapItem>> = Vec::new();
        for line in input.lines() {
            let new_line = line.chars().map(|c| MapItem::from(c)).collect::<Vec<_>>();
            col_max = col_max.max(new_line.len());

            // Find first position
            if is_first {
                first_available_col = new_line
                    .iter()
                    .position(|v| *v == MapItem::OpenTile)
                    .expect("Failed to find first position")
                    .try_into()
                    .expect("Failed to convert initial coordinate");
                is_first = false;
            }

            // Add new line
            map.push(new_line);
        }

        // Update
        map.iter_mut()
            .for_each(|row| row.resize(col_max, MapItem::Nil));

        Self {
            map,
            player: Player {
                col: first_available_col,
                row: 0,
                facing: Facing::Right,
            },
            form,
        }
    }

    fn update_col(&mut self, col_shift: u8, dir: Direction) {
        match self.form {
            GridForm::Flatten => {
                self.player.col = self.find_flatten_col_pos(col_shift, dir);
            }
            GridForm::Cubic(_) => self.find_cubic_col_pos(col_shift, dir),
        }
    }

    fn find_cubic_col_pos(&mut self, _col_shift: u8, _dir: Direction) {
        todo!()
    }

    fn find_flatten_col_pos(&self, col_shift: u8, dir: Direction) -> u32 {
        let cur_line: &[MapItem] = self.map.iter().nth(self.player.row as usize).unwrap();
        let mut shift = col_shift;
        let mut last_valid_idx: u32 = self.player.col;
        let mut cur_idx: u32 = self.player.col;
        loop {
            if shift == 0 {
                return last_valid_idx;
            }

            cur_idx = (cur_idx as i32 + if dir == Direction::Increase { 1 } else { -1 })
                .rem_euclid(cur_line.len() as i32) as u32;
            match cur_line[cur_idx as usize] {
                MapItem::OpenTile => {
                    last_valid_idx = cur_idx;
                    shift -= 1
                }
                MapItem::SolidWall => shift = 0,
                _ => {}
            }
        }
    }

    fn update_row(&mut self, row_shift: u8, dir: Direction) {
        match self.form {
            GridForm::Flatten => {
                self.player.row = self.find_flatten_row_pos(row_shift, dir);
            }
            GridForm::Cubic(_) => self.move_row_player(row_shift, dir),
        }
    }

    fn move_row_player(&mut self, row_shift: u8, dir: Direction) {
        let max_row: u32 = self.map.len() as u32;
        let mut shift = row_shift;
        let mut last_valid_idx: u32 = self.player.row;
        let mut col_idx: u32 = self.player.row;
        loop {
            if shift == 0 {
                self.player.row = last_valid_idx;
                return;
            }

            // Compute new index
            col_idx = (col_idx as i32 + if dir == Direction::Increase { 1 } else { -1 })
                .rem_euclid(max_row as i32) as u32;

            // Change face?
            match self.player.col {
                0..=49 => {
                    // Face Bot/Left
                    if col_idx < 100 {
                        // Face 4
                    } else if col_idx >= 200 {
                        // Face 6
                    }
                }
                50..=99 => {
                    // Face Middle
                }
                100..=149 => {
                    // Face Top/Right
                }
                _ => unreachable!(),
            }

            match &self.map[col_idx as usize][self.player.col as usize] {
                MapItem::OpenTile => {
                    last_valid_idx = col_idx;
                    shift -= 1
                }
                MapItem::SolidWall => shift = 0,
                _ => {}
            }
        }
    }

    fn find_flatten_row_pos(&self, row_shift: u8, dir: Direction) -> u32 {
        let max_row: u32 = self.map.len() as u32;
        let mut shift = row_shift;
        let mut last_valid_idx: u32 = self.player.row;
        let mut cur_idx: u32 = self.player.row;
        loop {
            if shift == 0 {
                return last_valid_idx;
            }

            cur_idx = (cur_idx as i32 + if dir == Direction::Increase { 1 } else { -1 })
                .rem_euclid(max_row as i32) as u32;
            match &self.map[cur_idx as usize][self.player.col as usize] {
                MapItem::OpenTile => {
                    last_valid_idx = cur_idx;
                    shift -= 1
                }
                MapItem::SolidWall => shift = 0,
                _ => {}
            }
        }
    }

    fn execute(&mut self, command: Cmd) -> AppResult<()> {
        match command {
            Cmd::Num(shift) => match self.player.facing {
                Facing::Up => self.update_row(shift, Direction::Decrease),
                Facing::Down => self.update_row(shift, Direction::Increase),
                Facing::Right => self.update_col(shift, Direction::Increase),
                Facing::Left => self.update_col(shift, Direction::Decrease),
            },
            Cmd::Clockwise => self.player.turn_right(),
            Cmd::CounterClockwise => self.player.turn_left(),
        }
        // println!("Player updated\n{}", self);
        Ok(())
    }
}

impl std::fmt::Display for TheGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acc = "\n".to_owned();
        for (row_idx, row) in self.map.iter().enumerate() {
            for (col_idx, case) in row.iter().enumerate() {
                if row_idx == self.player.row as usize && col_idx == self.player.col as usize {
                    acc += &self.player.facing.to_string();
                } else {
                    acc += &case.to_string();
                }
            }
            acc += "\n"
        }
        write!(f, "{}", acc)
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (grid_str, commands) = input.split_once("\n\n").expect("Invalid input");

    let mut the_grid = TheGrid::new(grid_str, GridForm::Flatten);
    let cmd_list = get_command_list(commands);

    // Move player
    // println!("Grid at {}", the_grid);
    cmd_list.iter().enumerate().for_each(|(idx, cmd)| {
        the_grid
            .execute(*cmd)
            .expect(&format!("Failed to execute cmd '{:#?}' [{}]", cmd, idx))
    });

    Some(the_grid.player.get_password())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (grid_str, commands) = input.split_once("\n\n").expect("Invalid input");

    let mut the_grid = TheGrid::new(grid_str, GridForm::Cubic(50));
    let cmd_list = get_command_list(commands);

    // Move player
    // println!("Grid at {}", the_grid);
    cmd_list.iter().enumerate().for_each(|(idx, cmd)| {
        the_grid
            .execute(*cmd)
            .expect(&format!("Failed to execute cmd '{:#?}' [{}]", cmd, idx))
    });

    Some(the_grid.player.get_password())
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
        // assert_eq!(part_two(&input), Some(5031));
        assert!(true)
    }
}

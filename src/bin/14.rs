/*
(Debug)
🎄 Part 1 🎄
1406 (elapsed: 92.30ms)
🎄 Part 2 🎄
20870 (elapsed: 2.09s)
(Release)
🎄 Part 1 🎄
1406 (elapsed: 6.53ms)
🎄 Part 2 🎄
20870 (elapsed: 135.97ms)
*/

use std::{collections::HashSet, fmt};

use advent_of_code::error::{AppError, AppResult};

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum State {
    Origin,
    Rock,
    // Air,
    Sand,
    // Abyss,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            State::Origin => "+",
            State::Rock => "#",
            // State::Air => ".",
            State::Sand => "O",
            // State::Abyss => "~",
        })
    }
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Cell {
    x: i32,
    y: u32,
    state: State,
}

#[derive(Debug, PartialEq, Clone)]
struct Grid {
    pub cells: HashSet<Cell>,
    entry_point: Cell,
    seabed: u32,
}

impl Grid {
    pub fn new(entry_point: Cell) -> Self {
        Self {
            entry_point,
            cells: HashSet::default(),
            seabed: 0_u32,
        }
    }

    pub fn cell_available(&self, x: i32, y: u32) -> bool {
        !self.cells.contains(&Cell {
            x,
            y,
            state: State::Rock,
        }) && !self.cells.contains(&Cell {
            x,
            y,
            state: State::Sand,
        })
    }

    pub fn add_rock_line(&mut self, draw: &str) {
        let mut iter_pos = draw.split(" -> ");
        let mut prev_pos = iter_pos.next().expect("First pos is missing");
        for next_pos in iter_pos {
            let (xp, yp) = eyes::try_parse!(prev_pos, "{},{}", i32, u32)
                .unwrap_or_else(|| panic!("Invalid coord: {}", prev_pos));
            let (xn, yn) = eyes::try_parse!(next_pos, "{},{}", i32, u32)
                .unwrap_or_else(|| panic!("Invalid coord: {}", next_pos));

            match xp.cmp(&xn) {
                std::cmp::Ordering::Less => {
                    for x in xp..=xn {
                        self.cells.insert(Cell {
                            x,
                            y: yn,
                            state: State::Rock,
                        });
                    }
                }
                std::cmp::Ordering::Greater => {
                    for x in xn..=xp {
                        self.cells.insert(Cell {
                            x,
                            y: yn,
                            state: State::Rock,
                        });
                    }
                }
                std::cmp::Ordering::Equal => {
                    if yp > yn {
                        for y in yn..=yp {
                            self.cells.insert(Cell {
                                x: xn,
                                y,
                                state: State::Rock,
                            });
                        }
                    } else {
                        for y in yp..=yn {
                            self.cells.insert(Cell {
                                x: xn,
                                y,
                                state: State::Rock,
                            });
                        }
                    }
                }
            }
            self.seabed = self.seabed.max(yn.max(yp));
            prev_pos = next_pos;
        }
    }

    pub fn next_step(&mut self) -> AppResult<()> {
        let mut x_prev_sand_pos = self.entry_point.x;
        for y_pos in self.entry_point.y + 1..=self.seabed {
            if y_pos == self.seabed + 1 {
                return Err(AppError::General("No more sand".to_string()));
            }

            // Down
            if self.cell_available(x_prev_sand_pos, y_pos) {
                continue;
            } else if self.cell_available(x_prev_sand_pos - 1, y_pos) {
                // Down-left
                x_prev_sand_pos -= 1;
                continue;
            } else if self.cell_available(x_prev_sand_pos + 1, y_pos) {
                // Down-right
                x_prev_sand_pos += 1;
                continue;
            } else {
                self.cells.insert(Cell {
                    x: x_prev_sand_pos,
                    y: y_pos - 1,
                    state: State::Sand,
                });
                return Ok(());
            }
        }
        Err(AppError::General("Meet seabed".to_string()))
    }

    pub fn next_step_with_bottom(&mut self) -> AppResult<()> {
        let mut x_prev_sand_pos = self.entry_point.x;
        for y_pos in self.entry_point.y + 1..=self.seabed {
            // Down
            if self.cell_available(x_prev_sand_pos, y_pos) {
                continue;
            } else if self.cell_available(x_prev_sand_pos - 1, y_pos) {
                // Down-left
                x_prev_sand_pos -= 1;
                continue;
            } else if self.cell_available(x_prev_sand_pos + 1, y_pos) {
                // Down-right
                x_prev_sand_pos += 1;
                continue;
            } else if y_pos == self.entry_point.y + 1 {
                return Err(AppError::General("Full".to_string()));
            } else {
                self.cells.insert(Cell {
                    x: x_prev_sand_pos,
                    y: y_pos - 1,
                    state: State::Sand,
                });
                return Ok(());
            }
        }
        self.cells.insert(Cell {
            x: x_prev_sand_pos,
            y: self.seabed - 1,
            state: State::Sand,
        });
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_for_test(input, false)
}

pub fn part_one_for_test(input: &str, test: bool) -> Option<u32> {
    let mut grid = Grid::new(Cell {
        x: if test { 6 } else { 500 },
        y: 0,
        state: State::Origin,
    });
    // Produce Rock Map
    // println!("Produce grid");
    input.lines().for_each(|line| grid.add_rock_line(line));

    let mut sand_counter = 0_u32;
    loop {
        // println!("Send sand");
        if grid.next_step().is_err() {
            return Some(sand_counter);
        }
        sand_counter += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_for_test(input, false)
}

pub fn part_two_for_test(input: &str, test: bool) -> Option<u32> {
    let mut grid = Grid::new(Cell {
        x: if test { 6 } else { 500 },
        y: 0,
        state: State::Origin,
    });
    // Produce Rock Map
    // println!("Produce grid");
    input.lines().for_each(|line| grid.add_rock_line(line));
    grid.seabed += 2;

    let mut sand_counter = 1_u32;
    loop {
        // println!("Send sand");
        if grid.next_step_with_bottom().is_err() {
            return Some(sand_counter);
        }
        sand_counter += 1;
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 14);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_one_for_test(&input, true), Some(24));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 14);
        assert_eq!(part_two_for_test(&input, true), Some(93));
    }
}

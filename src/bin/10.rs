/*
(Debug)
🎄 Part 1 🎄
13820 (elapsed: 108.27µs)
🎄 Part 2 🎄
###.#..#..##..###..#..#..##..###..#..#.
...#.#.#..#..#.#..#.#.#..#..#.#..#.#.#..
..#..##...#....#..#.##...#....#..#.##...
.#...#.#..#.##.###..#.#..#.##.###..#.#..
##...#.#..#..#.#.#..#.#..#..#.#.#..#.#..
####.#..#..###.#..#.#..#..###.#..#.#..#.
.
13820 (elapsed: 124.59µs)

(Release)
🎄 Part 1 🎄
13820 (elapsed: 6.47µs)
🎄 Part 2 🎄
###.#..#..##..###..#..#..##..###..#..#.
...#.#.#..#..#.#..#.#.#..#..#.#..#.#.#..
..#..##...#....#..#.##...#....#..#.##...
.#...#.#..#.##.###..#.#..#.##.###..#.#..
##...#.#..#..#.#.#..#.#..#..#.#.#..#.#..
####.#..#..###.#..#.#..#..###.#..#.#..#.
.
13820 (elapsed: 17.66µs)
*/

use advent_of_code::error::AppResult;

#[derive(Clone, Copy)]
enum OpCode {
    Noop,
    Addx(i32),
}

struct Crt {
    pub screen: String,
}

impl Crt {
    fn new() -> Self {
        Crt {
            screen: String::new(),
        }
    }

    pub fn draw_pixel(&mut self, x_reg: i32, cycle: u32) {
        let pixel_index = cycle % 40;
        let x: u32 = x_reg.clamp(0, 39) as u32;
        if pixel_index <= x + 1 && pixel_index >= x.saturating_sub(1) {
            self.screen.push('#');
        } else {
            self.screen.push('.')
        }
        if pixel_index == 39 {
            self.screen.push('\n');
        }
    }
}

impl From<&str> for OpCode {
    fn from(value: &str) -> Self {
        if value == "noop" {
            return OpCode::Noop;
        }
        match value.trim().split_once(' ') {
            Some(("addx", n)) => OpCode::Addx(n.parse::<i32>().expect("Not a decimal.")),
            _ => panic!("Failed parsing value: '{value}'."),
        }
    }
}

struct CpuState {
    clock: u32,
    rex_x: i32,
    strengh_tic: Vec<u32>,
    crt: Option<Crt>,
}

impl CpuState {
    pub fn new(strengh_tic: &[u32], display: Option<Crt>) -> Self {
        CpuState {
            clock: 0,
            rex_x: 1,
            strengh_tic: strengh_tic.to_vec(),
            crt: display,
        }
    }

    pub fn update_clock(&mut self, cmd: OpCode) -> AppResult<i32> {
        self.clock += 1;
        match cmd {
            OpCode::Noop => {
                let value = if self.strengh_tic.contains(&self.clock) {
                    self.clock as i32 * self.rex_x
                } else {
                    0
                };
                if let Some(crt) = &mut self.crt {
                    crt.draw_pixel(self.rex_x, self.clock);
                }
                Ok(value)
            }
            OpCode::Addx(shift) => {
                // Update tic and reg
                let mut strengh_value = 0_i32;
                for tic in 0..2 {
                    self.clock += tic;

                    // Compute strenght
                    if self.strengh_tic.contains(&self.clock) {
                        strengh_value = self.clock as i32 * self.rex_x;
                    }

                    if tic == 1 {
                        self.rex_x += shift;
                    }

                    if let Some(crt) = &mut self.crt {
                        crt.draw_pixel(self.rex_x, self.clock);
                    }
                }
                Ok(strengh_value)
            }
        }
    }

    fn display_screen(&self) {
        if let Some(crt) = &self.crt {
            println!("{}", crt.screen);
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = CpuState::new(&[20, 60, 100, 140, 180, 220], None);
    Some(input.lines().map(OpCode::from).fold(0, |acc, cmd| {
        acc + cpu.update_clock(cmd).expect("Invalid command")
    }))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut cpu = CpuState::new(&[20, 60, 100, 140, 180, 220], Some(Crt::new()));
    let res = Some(input.lines().map(OpCode::from).fold(0, |acc, cmd| {
        acc + cpu.update_clock(cmd).expect("Invalid command")
    }));
    cpu.display_screen();
    res
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), Some(13140));
    }
}

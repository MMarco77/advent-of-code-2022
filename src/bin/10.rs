use advent_of_code::error::AppResult;

#[derive(Clone, Copy)]
enum OpCode {
    Noop,
    Addx(i32),
}

impl From<&str> for OpCode {
    fn from(value: &str) -> Self {
        if value == "noop" {
            return OpCode::Noop 
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
    crt: Vec<char>
}

impl CpuState {
    pub fn new(strengh_tic: &[u32]) -> Self {
        CpuState { clock: 0, rex_x: 1, strengh_tic: strengh_tic.to_vec(), crt: Vec::new() }
    }
    pub fn tic(&mut self, tic_count: u32) {
        self.clock += tic_count;
        let range = self.rex_x..self.rex_x+2;
        let pixel = if range.contains(&(self.clock as i32)) {
            '#'
        } else {
            '.'
        };
        self.crt.push(pixel);
    }

    pub fn draw_crt(&self) {
        // println!{"{:#?}", self.crt};
        self.crt.iter().for_each(|v| print!("{}", v));
    }

    pub fn update_clock(&mut self, cmd: OpCode) -> AppResult<i32> {
        self.tic(1);
        match cmd {
            OpCode::Noop => {                 
                if self.strengh_tic.contains(&self.clock) {
                    Ok(self.clock as i32 * self.rex_x)
                } else {
                    Ok(0)
                }
            },
            OpCode::Addx(shift) => {
                // Update tic and reg
                let mut strengh_value = 0_i32;
                for tic in 0..2 {
                    self.tic(tic);

                    // Compute strenght
                    if self.strengh_tic.contains(&self.clock) {
                        strengh_value = self.clock as i32 * self.rex_x;
                    }

                    if tic == 1 {
                        self.rex_x += shift;
                    }
                }
                Ok(strengh_value)
            },
        }
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut cpu = CpuState::new(&[20, 60, 100, 140, 180, 220]);
    Some(input.lines()
         .map(OpCode::from)
         .fold(0, |acc, cmd| acc + cpu.update_clock(cmd).expect("Invalid command")))
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut cpu = CpuState::new(&[20, 60, 100, 140, 180, 220]);
    let res = Some(input.lines()
         .map(OpCode::from)
         .fold(0, |acc, cmd| acc + cpu.update_clock(cmd).expect("Invalid command")));
    cpu.draw_crt();
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

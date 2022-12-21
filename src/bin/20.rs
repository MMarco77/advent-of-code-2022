struct Mixing {
    data: Vec<i32>,
    len: i32,
}

impl Mixing {
    pub fn from_input(input: &str) -> Self {
        let data = input
            .lines()
            .map(|l| l.parse::<i32>().expect(&format!("Invalid value '{}'", l)))
            .collect::<Vec<_>>();
        let len = data.len().try_into().expect("Convert len");
        Self { data, len }
    }

    pub fn get_nth_num_from_zero(&self, counter: u32) -> Option<i32> {
        let cur_zero_idx: u32 = self
            .data
            .iter()
            .position(|&v| v == 0)
            .expect("Looking for '0' idx")
            .try_into()
            .expect("Failed to convert index of '0'");

        let mod_shift: i32 = (cur_zero_idx + counter) as i32 % self.len;

        // Debug print
        self.show_chunk(mod_shift.try_into().unwrap(), 3);

        self.data.get(mod_shift as usize).cloned()
    }

    pub fn mode_item(&mut self, item: i32) {
        let watch_dog: i64 = self.data.iter().map(|v| *v as i64).sum();
        // println!("Move '{}'", shift);
        if item == 0 {
            // println!("No move");
            return;
        }

        let cur_idx: i32 = self
            .data
            .iter()
            .position(|&v| v == item)
            .expect(&format!("Looking for '{}'", item))
            .try_into()
            .expect("Failed to convert index");
        let new_idx = match cur_idx + item {
            a if a == 0 => self.len,
            a if a > 0 => (a + 1) % self.len,
            a if a < 0 => {
                let neg_moduloe = a % self.len;
                let tmp: i32 = self.len + neg_moduloe;
                tmp
            }
            _ => unreachable!(),
        };

        // println!("{} insert {}", self, new_idx);
        self.data.insert(
            new_idx.try_into().expect(&format!(
                "Failed to insert '{}' for shift to '{}', cur_idx '{}'",
                new_idx, item, cur_idx
            )),
            item,
        );

        // println!("{}", self);
        if cur_idx > new_idx {
            self.data.remove((cur_idx + 1).try_into().unwrap());
        } else {
            self.data.remove(cur_idx.try_into().unwrap());
        }

        // Check
        if watch_dog != self.data.iter().map(|v| *v as i64).sum() {
            panic!("Incorrect vector after move elements")
        }
        // println!("{}", self);
    }

    pub fn show_chunk(&self, origin: u32, delta: u32) {
        let len_eq: u32 = self.len.try_into().expect("Show chunk failed convert len");
        let lower: usize = (origin - delta)
            .clamp(0, len_eq)
            .try_into()
            .expect(&format!("Failed to display lower idx: {}", origin - delta));
        let upper: usize = (origin + delta)
            .clamp(0, len_eq)
            .try_into()
            .expect(&format!("Failed to display upper idx: {}", origin + delta));

        for idx in lower..origin as usize {
            print!("[{:#?}]", self.data.get(idx));
        }
        print!("{:#?}", self.data.get(origin as usize));
        for idx in origin as usize+1..upper {
            print!("{:#?}", self.data.get(idx));
        }
        println!();
    }
}

impl std::fmt::Display for Mixing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acc = String::new();
        acc += "[";
        self.data.iter().for_each(|v| acc += &format!("{} ", v));
        acc += "]";
        write!(f, "{}", acc)
    }
}

pub fn part_one(input: &str) -> Option<i32> {
    let mut mixing = Mixing::from_input(input);
    for line in input.lines() {
        let shift: i32 = line
            .parse::<i32>()
            .expect(&format!("Invalid value '{}'", line));
        mixing.mode_item(shift);
    }

    Some([1000, 2000, 3000].iter().fold(0_i32, |acc, v| {
        acc + mixing
            .get_nth_num_from_zero(*v)
            .expect(&format!("Failed to find {}th", v))
    }))
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
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
        assert_eq!(part_two(&input), None);
    }
}

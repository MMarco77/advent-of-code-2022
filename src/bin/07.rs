/*
(Debug)

(Release)
 */

use advent_of_code::error::AppResult;

trait Component {
    fn size(&self) -> u64;
    // HashMap => 773836; Vec => 939466
    fn stat_size(&self, statistic: &mut Vec<(String, u64)>);
    fn pretty_print(&self, shift: &str);
}

struct File {
    #[allow(dead_code)]
    pub name: String,
    pub size: u64,
}

impl File {
    pub fn new(name: &str, size: u64) -> Self {
        Self {
            name: name.to_owned(),
            size,
        }
    }
}

impl Component for File {
    fn size(&self) -> u64 {
        self.size
    }

    fn stat_size(&self, _: &mut Vec<(String, u64)>) {}

    fn pretty_print(&self, shift: &str) {
        println!("{} - {} (file, size={})", shift, self.name, self.size);
    }
}

struct Folder {
    pub name: String,
    pub components: Vec<Box<dyn Component>>,
}

impl Folder {
    pub fn new(name: &str) -> Self {
        Folder {
            name: name.to_owned(),
            components: Vec::new(),
        }
    }

    pub fn add(&mut self, component: impl Component + 'static) {
        self.components.push(Box::new(component));
    }
}

impl Component for Folder {
    fn size(&self) -> u64 {
        self.components.iter().fold(0, |acc, cmp| acc + cmp.size())
    }

    fn stat_size(&self, statistic: &mut Vec<(String, u64)>) {
        statistic.push((self.name.clone(), self.size()));
        for component in self.components.iter() {
            component.stat_size(statistic);
        }
    }

    fn pretty_print(&self, shift: &str) {
        println!("{} - {} (dir, size={})", shift, self.name, self.size());
        for component in self.components.iter() {
            component.pretty_print(&format!("{shift}    "))
        }
    }
}

fn populate<'a>(
    folder: &mut Folder,
    line_iter: &mut impl Iterator<Item = &'a str>,
) -> AppResult<()> {
    while let Some(line) = line_iter.next() {
        if line.starts_with("$ cd ..") {
            return Ok(());
        }
        if line.starts_with("$ cd ") {
            let name: String = line.chars().skip(5).collect::<Vec<_>>().iter().collect();
            let mut new_folder = Folder::new(&name);
            populate(&mut new_folder, line_iter).expect("Error on folder");
            folder.add(new_folder);
        } else if line.starts_with(|c: char| c.is_numeric()) {
            if let Some((s, n)) = eyes::try_parse!(line, "{} {}", u64, String) {
                folder.add(File::new(&n, s));
            } else {
                panic!("Invalid File + Size for {}", line);
            }
        }
    }
    Ok(())
}

pub fn part_one(input: &str) -> Option<u64> {
    // Create FS
    let mut root = Folder::new("/");
    let mut line_iter = input.lines().skip(2);
    populate(&mut root, &mut line_iter).expect("Failed to create root");

    // Compute soluce
    let mut statistic: Vec<(String, u64)> = Vec::new();
    root.stat_size(&mut statistic);
    //root.pretty_print("");
    let size_list = statistic
        .iter()
        .filter_map(|(_, s)| if *s <= 100_000 { Some(*s) } else { None })
        .collect::<Vec<u64>>();
    Some(size_list.iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
   let mut root = Folder::new("/");
    let mut line_iter = input.lines().skip(2);
    populate(&mut root, &mut line_iter).expect("Failed to create root");

    // Folder stat
    let mut statistic: Vec<(String, u64)> = Vec::new();
    root.stat_size(&mut statistic);
    let size_list = statistic
        .iter()
        .map(|(_, s)| *s)
        .collect::<Vec<u64>>();
   
    let total_space = 70_000_000;
    let used_space = size_list.iter().max().unwrap();
    let unused_space = total_space - used_space;
    let desired_space = 30_000_000;
    let amount_to_free = desired_space - unused_space;

    size_list
        .iter()
        .filter(|&size| size >= &amount_to_free)
        .min().copied()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}

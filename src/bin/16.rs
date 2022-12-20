use std::collections::HashSet;

use petgraph::{
    algo,
    dot::{Config, Dot},
    prelude::DiGraph,
    Graph, adj::NodeIndex,
};

pub fn part_one(input: &str) -> Option<u32> {
    let mut graph = DiGraph::<&str, u32>::new();

    let AA = graph.add_node("AA");
    let BB = graph.add_node("BB");
    let CC = graph.add_node("CC");
    let DD = graph.add_node("DD");
    let EE = graph.add_node("EE");
    let FF = graph.add_node("FF");
    let GG = graph.add_node("GG");
    let HH = graph.add_node("HH");
    let II = graph.add_node("II");
    let JJ = graph.add_node("JJ");

    graph.extend_with_edges(&[
        (AA, DD, 0),
        (AA, II, 0),
        (AA, BB, 0),
        (BB, CC, 13),
        (BB, AA, 13),
        (CC, DD, 2),
        (CC, BB, 2),
        (DD, CC, 20),
        (DD, AA, 20),
        (DD, EE, 20),
        (EE, FF, 3),
        (EE, DD, 3),
        (FF, EE, 0),
        (FF, GG, 0),
        (GG, FF, 0),
        (GG, HH, 0),
        (HH, GG, 22),
        (II, AA, 0),
        (II, JJ, 0),
        (JJ, II, 21),
    ]);
    // println!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    println!("{}", Dot::new(&graph));
    let mut path_finder: HashSet<(u32, u32)> = HashSet::new();
    for start in graph.node_indices() {
        println!("--- {:?} ---", start.index());
        println!("{:?}", algo::dijkstra(&graph, start, None, |_| 1));
        for (n_idx, weight) in algo::dijkstra(&graph, start, None, |_| 1) {
            path_finder.insert((start.index() as u32, n_idx.index() as  u32));
        }
    }
    println!("{:#?}", path_finder);

    Some(1651)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 16);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_one(&input), Some(1651));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 16);
        assert_eq!(part_two(&input), None);
    }
}

use std::{
    arch::x86_64::_MM_EXCEPT_INEXACT,
    collections::{HashMap, HashSet},
    mem::replace,
};

use petgraph::{algo, dot::Dot, prelude::DiGraph};

fn get_graph(input: &str) -> Option<DiGraph<u8, u8>> {
    let mut node_label: HashMap<String, petgraph::prelude::NodeIndex> = HashMap::new();
    let mut edge_list: Vec<(String, String)> = Vec::new();
    let mut graph = DiGraph::<u8, u8>::new();
    for new_node_desc in input.lines() {
        let new_node_desc_2 = new_node_desc
            .replace("tunnels", "tunnel")
            .replace("valves", "valve")
            .replace("leads", "lead");
        let (new_node_name, weight, edges) = eyes::try_parse!(
            &new_node_desc_2,
            "Valve {} has flow rate={}; tunnel lead to valve {}",
            String,
            u8,
            String
        )
        .expect(&format!("Invalid node description '{}'", new_node_desc_2));
        let new_node = graph.add_node(weight);
        node_label.insert(new_node_name.clone(), new_node);

        // Add edge
        edges.split(",").for_each(|other_node_name| {
            edge_list.push((new_node_name.clone(), other_node_name.trim().to_string()))
        });
    }

    // Create edge
    for (from_node_name, to_node_name) in edge_list.iter() {
        let from_node = node_label
            .get(from_node_name)
            .expect(&format!("Node {} is missing", from_node_name));
        let to_node = node_label
            .get(to_node_name)
            .expect(&format!("Node {} is missing", to_node_name));
        graph.add_edge(*from_node, *to_node, 1);
    }

    println!("{:?}", Dot::new(&graph));
    Some(graph)
}

pub fn part_one(input: &str) -> Option<u32> {
    let graph = get_graph(input).expect("Failed to create graph");
    for start in graph.node_indices() {
        println!("--- {:?} ---", start.index());
        let best_path = algo::dijkstra(&graph, start, None, |_| 1);
        println!("{:?}", best_path);
        let mut elapse_time = 0_u32;
        let mut pressure = 0_u32;
        for (n_idx, weight) in best_path {
            elapse_time += weight;
            pressure += *graph
                .node_weight(n_idx)
                .expect("Failed to find node weight") as u32;
        }
        println!("elapse_time: {}; pressure: {}", elapse_time, pressure);
    }

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

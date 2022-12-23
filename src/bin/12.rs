use petgraph::{algo, prelude::DiGraph, dot::Dot};

pub fn part_one(_input: &str) -> Option<u32> {
    let mut graph = DiGraph::<_, ()>::new();
    let S = graph.add_node("S");
    // let a1 = graph.add_node("a1");
    // let b1 = graph.add_node("b1");
    let q1 = graph.add_node("q1");
    let p1 = graph.add_node("p1");
    let o1 = graph.add_node("o1");
    let n1 = graph.add_node("n1");
    let m1 = graph.add_node("m1");
    let a2 = graph.add_node("a2");
    let b2 = graph.add_node("b2");
    // let c2 = graph.add_node("c2");
    let r2 = graph.add_node("r2");
    let y2 = graph.add_node("y2");
    let x2 = graph.add_node("x2");
    let x22 = graph.add_node("x2");
    let l2 = graph.add_node("l2");
    // let a3 = graph.add_node("a3");
    let c3 = graph.add_node("c3");
    let c32 = graph.add_node("c3");
    let s3 = graph.add_node("s3");
    let z3 = graph.add_node("z3");
    let E = graph.add_node("E");
    let x3 = graph.add_node("x3");
    let k3 = graph.add_node("k3");
    // let a4 = graph.add_node("a4");
    // let c4 = graph.add_node("c4");
    let c42 = graph.add_node("c4");
    let t4 = graph.add_node("t4");
    let u4 = graph.add_node("u4");
    let v4 = graph.add_node("v4");
    let w4 = graph.add_node("w4");
    let j4 = graph.add_node("j4");
    // let a5 = graph.add_node("a5");
    // let b5 = graph.add_node("b5");
    let d5 = graph.add_node("d5");
    let e5 = graph.add_node("e5");
    let f5 = graph.add_node("f5");
    let g5 = graph.add_node("g5");
    let h5 = graph.add_node("h5");
    let i5 = graph.add_node("i5");

    graph.extend_with_edges(&[
        //(S, a1),
        (S, a2),
        // (a1, S),
        // (a1, b1),
        // (a1, b2),
        // (b1, a1),
        // (b1, c2),
        // (q1, b1),
        (q1, p1),
        (q1, r2),
        (p1, q1),
        (p1, o1),
        (o1, p1),
        (o1, n1),
        (n1, o1),
        (n1, m1),
        (m1, n1),
        (m1, l2),
        (a2, S),
        (a2, b2),
        // (a2, a3),
        // (b2, a1),
        (b2, a2),
        // (b2, c2),
        (b2, c3),
        // (c2, b1),
        // (c2, b2),
        // (c2, c3),
        (r2, q1),
        //(r2, c2),
        (r2, s3),
        (y2, p1),
        (y2, r2),
        (y2, x2),
        (y2, z3),
        (x2, o1),
        (x2, y2),
        (x2, x22),
        (x2, E),
        (x22, n1),
        (x22, x2),
        (x22, x3),
        (x22, l2),
        (l2, m1),
        (l2, k3),
        // (a3, a2),
        // (a3, c3),
        // (a3, a4),
        (c3, b2),
        // (c3, a3),
        // (c3, c4),
        (c3, c32),
        //(c32, c2),
        (c32, c3),
        (c32, c42),
        (s3, r2),
        (s3, c32),
        (s3, t4),
        (z3, y2),
        (z3, s3),
        (z3, u4),
        (z3, E),
        (x3, x22),
        (x3, k3),
        (k3, l2),
        (k3, j4),
        // (a4, a3),
        // (a4, a5),
        // (c4, a4),
        // (c4, c42),
        // (c4, c3),
        // (c4, b5),
        (c42, c32),
        //(c42, c4),
        (c42, d5),
        (t4, s3),
        (t4, c42),
        (t4, e5),
        (t4, u4),
        (u4, t4),
        (u4, f5),
        (u4, v4),
        (v4, u4),
        (v4, g5),
        (v4, w4),
        (w4, x3),
        (w4, v4),
        (w4, h5),
        (w4, j4),
        (j4, k3),
        (j4, i5),
        // (a5, a4),
        // (a5, b5),
        // (b5, a5),
        // (b5, c4),
        // (d5, c4),
        // (d5, b5),
        (d5, e5),
        (e5, d5),
        (e5, f5),
        (f5, e5),
        (f5, g5),
        (g5, f5),
        (g5, h5),
        (h5, g5),
        (h5, i5),
        (i5, j4),
        (i5, h5),
    ]);
    println!("{:?}", Dot::new(&graph));

    let best_path = algo::dijkstra(&graph, S, Some(E), |_| 1);
    // println!("{:?}", best_path);
    println!("Best path length {}", best_path.iter().map(|_| 1).collect::<Vec<_>>().iter().sum::<u8>());

    Some(best_path.iter().map(|_| 1).collect::<Vec<_>>().iter().sum::<u32>())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), Some(31));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}

use std::{collections::{HashMap, HashSet}, hash::Hash, i64};
use crate::graph::{Edge, Graph, Vertex};
use super::BisectionGraphPartition;

pub struct KernighanLin;

struct KernighanLinImpl<'a, I> {
    graph: &'a Graph<I>,
    part_a: HashSet<I>,
    part_b: HashSet<I>,
}

impl<I: Copy + Hash + Eq> BisectionGraphPartition<I> for KernighanLin {
    fn partition(&self, graph: &Graph<I>) -> (HashSet<I>, HashSet<I>) {
        let mut kl_impl = KernighanLinImpl::new(graph);
        kl_impl.partition();
        (kl_impl.part_a, kl_impl.part_b)
    }
}

impl<'a, I: Copy + Hash + Eq> KernighanLinImpl<'a, I> {
    fn new(graph: &'a Graph<I>) -> Self {
        let mut part_a = HashSet::new();
        let mut part_b = HashSet::new();

        for (i, (&v_id, _)) in graph.vertices.iter().enumerate() {
            if i < (graph.num_vertices() / 2) {
                part_a.insert(v_id);
            } else {
                part_b.insert(v_id);
            }
        }

        Self {
            graph,
            part_a,
            part_b,
        }
    }

    fn partition(&mut self) {
        println!("Initial partition cost: {}", self.partition_cost());

        let mut p = 0;
        let mut total_gain = 0;

        // loop until can't imporve
        loop {
            // init group a and group b vector
            let mut group_a: Vec<&Vertex<I>> = vec![];
            let mut group_b = vec![];
            for (id, vertex) in self.graph.vertices.iter() {
                if self.part_a.contains(id) {
                    group_a.push(vertex);
                } else {
                    group_b.push(vertex);
                }
            }

            // calcuate all gain values
            let mut improvements: HashMap<I, i64> = self.graph.vertices.iter()
                .map(|(&id, vertex)| (id.clone(), self.calculate_vertex_improvement(vertex)))
                .collect();

            // find all swaps
            let mut swap_records = vec![];
            for _ in 0..self.graph.num_vertices() / 2 {
                // try to find a best swap
                let mut best = None;
                for &va in &group_a {
                    for &vb in &group_b {
                        let gain = improvements[&va.id]
                            + improvements[&vb.id]
                            - 2 * self.vertex_connection_weight(va, vb);

                        match best {
                            Some((_, _, g)) if g >= gain => {}
                            _ => best = Some((va, vb, gain)),
                        }
                    }
                }

                let (a, b, best_gain) = best.unwrap();
            
                // remove these two vertex(can't swap again)
                group_a.retain(|&x| x.id != a.id);
                group_b.retain(|&x| x.id != b.id);
                swap_records.push((a, b, best_gain));

                // update improvements
                for &x in group_a.iter() {
                    let c_xa = self.vertex_connection_weight(x, a);
                    let c_xb = self.vertex_connection_weight(x, b);
                    *improvements.get_mut(&x.id).unwrap() += 2 * c_xa - 2 * c_xb;
                }
                for &y in group_b.iter() {
                    let c_ya = self.vertex_connection_weight(y, a);
                    let c_yb = self.vertex_connection_weight(y, b);
                    *improvements.get_mut(&y.id).unwrap() += 2 * c_yb - 2 * c_ya;
                }
            }

            // find j that maximizes the sum g_max
            let mut g_max = i64::MIN;
            let mut jmax = 0;
            for j in 1..(swap_records.len() + 1) {
                let g_sum: i64 = swap_records[..j].iter().map(|(_, _, gain)| *gain).sum();
                if g_sum > g_max {
                    g_max = g_sum;
                    jmax = j;
                }
            }

            if g_max > 0 {
                // accept swap_records[..jmax]'s swaps
                for (a, b, _) in swap_records.into_iter().take(jmax) {
                    self.part_a.remove(&a.id);
                    self.part_b.insert(a.id);
            
                    self.part_b.remove(&b.id);
                    self.part_a.insert(b.id);
                }
                p += 1;
                total_gain += g_max;
                println!("Pass: {p} \t\t\tGain: {g_max}")
            } else {
                // break loop
                break;
            }
        }

        let partition_cost = self.partition_cost();
        println!("Total passes: {p} \t\tTotal gain: {total_gain} + Final partition cost: {partition_cost}")
    }

    fn calculate_vertex_improvement(&self, vertex: &Vertex<I>) -> i64 {
        vertex.neighbors.iter()
            .map(|nid| {
                if self.is_same_partition(&vertex.id, nid) { -1 } else { 1 }
            })
            .sum()
    }

    fn is_same_partition(&self, left_id: &I, right_id: &I) -> bool {
        let is_left_in_a = self.part_a.contains(left_id);
        let is_right_in_a = self.part_a.contains(right_id);
        is_left_in_a == is_right_in_a
    }

    fn is_edge_cuted(&self, edge: &Edge<I>) -> bool {
        let left = self.graph.vertices.get(&edge.left).unwrap();
        let right = self.graph.vertices.get(&&edge.right).unwrap();
        !self.is_same_partition(&left.id, &right.id)
    }

    fn is_neighbor_vertex(&self, v1: &Vertex<I>, v2: &Vertex<I>) -> bool {
        v1.neighbors.contains(&v2.id)
    }

    fn vertex_connection_weight(&self, v1: &Vertex<I>, v2: &Vertex<I>) -> i64 {
        if self.is_neighbor_vertex(v1, v2) { 1 } else { 0 }
    }

    fn partition_cost(&self) -> usize {
        self.graph.edges.iter()
            .filter(|edge| self.is_edge_cuted(edge))
            .count()
    }
}

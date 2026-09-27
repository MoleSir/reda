use std::{collections::HashSet, hash::Hash};
use crate::graph::Graph;
use super::BisectionGraphPartition;

pub struct FiducciaMattheyses;

struct FiducciaMattheysesImpl<'a, I> {
    #[allow(dead_code)] // 待 partition() 实现后使用
    graph: &'a Graph<I>,
    part_a: HashSet<I>,
    part_b: HashSet<I>,
}



impl<I: Copy + Hash + Eq> BisectionGraphPartition<I> for FiducciaMattheyses {
    fn partition(&self, graph: &Graph<I>) -> (HashSet<I>, HashSet<I>) {
        let mut kl_impl = FiducciaMattheysesImpl::new(graph);
        kl_impl.partition();
        (kl_impl.part_a, kl_impl.part_b)
    }
}

impl<'a, I: Copy + Hash + Eq> FiducciaMattheysesImpl<'a, I> {
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

    }
}
mod kernighan_lin;
mod fiduccia_mattheyses;
pub use kernighan_lin::KernighanLin;
pub use fiduccia_mattheyses::FiducciaMattheyses;

use std::collections::HashSet;
use crate::graph::Graph;


pub trait BisectionGraphPartition<I> {
    fn partition(&self, graph: &Graph<I>) -> (HashSet<I>, HashSet<I>);
}

pub trait BisectionHyperGraphPartition<I> {
}


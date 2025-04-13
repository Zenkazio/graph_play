use super::super::basics::Graph;
use super::super::edge::Edge;
use super::super::node::Node;
use std::collections::HashSet;
use std::rc::Rc;

#[derive(Debug, Default)]
pub struct AdjacencyList(HashSet<Rc<Node>>, Vec<Edge>);

impl AdjacencyList {
    pub fn new() -> Self {
        AdjacencyList(HashSet::new(), Vec::new())
    }
}

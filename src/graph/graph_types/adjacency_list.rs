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

impl Graph for AdjacencyList {
    fn add_node(&mut self, node: Rc<Node>) {
        todo!()
    }

    fn add_nodes(&mut self, nodes: HashSet<Rc<Node>>) {
        todo!()
    }

    fn add_edge(&mut self, edge: Edge) {
        todo!()
    }

    fn add_edges(&mut self, edges: HashSet<Edge>) {
        todo!()
    }

    fn to_adjacency_matrix(&self) -> super::adjacency_matrix::AdjacencyMatrix {
        todo!()
    }

    fn to_mathematical_graph(&self) -> super::mathematical_graph::MathematicalGraph {
        todo!()
    }

    fn to_edge_list(&self) -> super::edge_list::Edgelist {
        todo!()
    }

    fn to_adjacency_list(&self) -> AdjacencyList {
        todo!()
    }

    fn get_nodes(&self) -> &HashSet<Rc<Node>> {
        todo!()
    }

    fn get_edges(&self) -> &HashSet<Edge> {
        todo!()
    }
}

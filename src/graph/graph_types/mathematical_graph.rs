use super::super::edge::Edge;
use super::super::node::Node;
use super::adjacency_list::AdjacencyList;
use super::edge_list::Edgelist;
use super::{super::basics::Graph, adjacency_matrix::AdjacencyMatrix};
use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Clone)]
pub struct MathematicalGraph(HashSet<Rc<Node>>, HashSet<Edge>);

impl MathematicalGraph {
    pub fn new() -> Self {
        MathematicalGraph(HashSet::new(), HashSet::new())
    }
}

impl Graph for MathematicalGraph {
    fn add_node(&mut self, node: Rc<Node>) {
        self.0.insert(node);
    }
    fn add_nodes(&mut self, nodes: HashSet<Rc<Node>>) {
        self.0.extend(nodes);
    }
    fn add_edge(&mut self, edge: Edge) {
        self.1.insert(edge);
    }
    fn add_edges(&mut self, edges: HashSet<Edge>) {
        self.1.extend(edges);
    }
    fn to_adjacency_matrix(&self) -> AdjacencyMatrix {
        unimplemented!()
    }
    fn to_mathematical_graph(&self) -> MathematicalGraph {
        self.clone()
    }
    fn to_edge_list(&self) -> Edgelist {
        unimplemented!()
    }
    fn to_adjacency_list(&self) -> AdjacencyList {
        unimplemented!()
    }
    fn get_nodes(&self) -> &HashSet<Rc<Node>> {
        &self.0
    }
    fn get_edges(&self) -> &HashSet<Edge> {
        &self.1
    }
}

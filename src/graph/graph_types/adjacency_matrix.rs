use super::super::basics::Graph;
use super::super::node::Node;
use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Default)]
pub struct AdjacencyMatrix(HashSet<Rc<Node>>, Vec<Vec<u64>>);

impl Graph for AdjacencyMatrix {
    fn add_node(&mut self, node: Rc<Node>) {
        unimplemented!()
    }

    fn add_nodes(&mut self, nodes: HashSet<Rc<Node>>) {
        unimplemented!()
    }

    fn add_edge(&mut self, edge: crate::graph::edge::Edge) {
        unimplemented!()
    }

    fn add_edges(&mut self, edges: HashSet<crate::graph::edge::Edge>) {
        unimplemented!()
    }

    fn to_adjacency_matrix(&self) -> AdjacencyMatrix {
        unimplemented!()
    }

    fn to_mathematical_graph(&self) -> super::mathematical_graph::MathematicalGraph {
        unimplemented!()
    }

    fn to_edge_list(&self) -> super::edge_list::Edgelist {
        unimplemented!()
    }

    fn to_adjacency_list(&self) -> super::adjacency_list::AdjacencyList {
        unimplemented!()
    }

    fn get_nodes(&self) -> &HashSet<Rc<Node>> {
        unimplemented!()
    }

    fn get_edges(&self) -> &HashSet<crate::graph::edge::Edge> {
        unimplemented!()
    }
}

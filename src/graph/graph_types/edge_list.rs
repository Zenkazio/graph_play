use super::super::basics::Graph;
use super::super::edge::Edge;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Edgelist(HashSet<Edge>);

impl Graph for Edgelist {
    fn add_node(&mut self, node: std::rc::Rc<crate::graph::node::Node>) {
        unimplemented!()
    }

    fn add_nodes(&mut self, nodes: HashSet<std::rc::Rc<crate::graph::node::Node>>) {
        unimplemented!()
    }

    fn add_edge(&mut self, edge: Edge) {
        unimplemented!()
    }

    fn add_edges(&mut self, edges: HashSet<Edge>) {
        unimplemented!()
    }

    fn to_adjacency_matrix(&self) -> super::adjacency_matrix::AdjacencyMatrix {
        unimplemented!()
    }

    fn to_mathematical_graph(&self) -> super::mathematical_graph::MathematicalGraph {
        unimplemented!()
    }

    fn to_edge_list(&self) -> Edgelist {
        unimplemented!()
    }

    fn to_adjacency_list(&self) -> super::adjacency_list::AdjacencyList {
        unimplemented!()
    }

    fn get_nodes(&self) -> &HashSet<std::rc::Rc<crate::graph::node::Node>> {
        unimplemented!()
    }

    fn get_edges(&self) -> &HashSet<Edge> {
        unimplemented!()
    }
}

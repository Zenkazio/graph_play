use crate::graph::edge::EdgeType;

use super::super::edge::Edge;
use super::super::node::Node;
use super::adjacency_list::AdjacencyList;
use super::edge_list::Edgelist;
use super::{super::basics::Graph, adjacency_matrix::AdjacencyMatrix};
use std::collections::HashMap;
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

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, rc::Rc};

    use crate::graph::{
        basics::Graph,
        edge::{Edge, EdgeType},
        graph_types::mathematical_graph::MathematicalGraph,
        node::Node,
    };

    #[test]
    fn test_to_adjacency_matrix() {
        let mut graph = MathematicalGraph::new();
        let mut nodes = Vec::new();
        let mut edges = Vec::new();
        for _ in 0..5 {
            nodes.push(Rc::new(Node::new()));
        }
        edges.push(Edge::new(
            nodes.get(0).unwrap().clone(),
            nodes.get(1).unwrap().clone(),
            Some(4),
            Some(EdgeType::Directed),
        ));
        edges.push(Edge::new(
            nodes.get(0).unwrap().clone(),
            nodes.get(2).unwrap().clone(),
            Some(6),
            None,
        ));
        edges.push(Edge::new(
            nodes.get(3).unwrap().clone(),
            nodes.get(4).unwrap().clone(),
            Some(8),
            None,
        ));

        graph.add_nodes(HashSet::from_iter(nodes.iter().map(|x| x.clone())));
        graph.add_edges(HashSet::from_iter(edges.iter().map(|x| x.clone())));
        dbg!(graph.to_adjacency_matrix().get_matrix());
        panic!()
    }
}

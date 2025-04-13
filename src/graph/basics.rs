use std::{collections::HashSet, rc::Rc};

use super::{
    edge::Edge,
    graph_types::{
        adjacency_list::AdjacencyList, adjacency_matrix::AdjacencyMatrix, edge_list::Edgelist,
        mathematical_graph::MathematicalGraph,
    },
    node::Node,
};

pub trait Graph {
    fn add_node(&mut self, node: Rc<Node>);
    fn add_nodes(&mut self, nodes: Vec<Rc<Node>>);
    fn add_edge(&mut self, edge: Edge);
    fn add_edges(&mut self, edges: Vec<Edge>);
    fn to_adjacency_matrix(&self) -> AdjacencyMatrix;
    fn to_mathematical_graph(&self) -> MathematicalGraph;
    fn to_edge_list(&self) -> Edgelist;
    fn to_adjacency_list(&self) -> AdjacencyList;
    fn get_nodes(&self) -> &HashSet<Rc<Node>>;
    fn get_edges(&self) -> &HashSet<Edge>;
}

use std::{
    hash::{Hash, Hasher},
    rc::Rc,
};

use super::node::Node;

#[derive(PartialEq, Hash, Eq, Debug, Clone)]
pub enum EdgeType {
    Undirected,
    Directed,
}

#[derive(Debug, Clone)]
pub struct Edge {
    start_node: Rc<Node>,
    end_node: Rc<Node>,
    weight: Option<u64>,
    edge_type: EdgeType,
}

impl Edge {
    pub fn new(
        start_node: Rc<Node>,
        end_node: Rc<Node>,
        weight: Option<u64>,
        edge_type: Option<EdgeType>,
    ) -> Self {
        Edge {
            start_node,
            end_node,
            weight,
            edge_type: edge_type.unwrap_or(EdgeType::Undirected),
        }
    }
    pub fn get_start_node(&self) -> Rc<Node> {
        self.start_node.clone()
    }
    pub fn get_end_node(&self) -> Rc<Node> {
        self.end_node.clone()
    }
    pub fn get_weight(&self) -> Option<u64> {
        self.weight
    }
    pub fn get_edge_type(&self) -> &EdgeType {
        &self.edge_type
    }
}
impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        ((self.start_node == other.start_node && self.end_node == other.end_node)
            || (self.start_node == other.end_node && self.end_node == other.start_node))
            && self.weight == other.weight
            && self.edge_type == other.edge_type
    }
}

impl Eq for Edge {}

impl Hash for Edge {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the fields in a consistent order
        let (start, end) = if self.start_node < self.end_node {
            (&self.start_node, &self.end_node)
        } else {
            (&self.end_node, &self.start_node)
        };
        start.hash(state);
        end.hash(state);
        self.weight.hash(state);
        self.edge_type.hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::node::Node;

    #[test]
    fn test_edge() {
        let node1 = Rc::new(Node::new());
        let node2 = Rc::new(Node::new());
        let edge = Edge::new(node1.clone(), node2.clone(), None, None);
        assert_eq!(edge.get_start_node(), node1);
        assert_eq!(edge.get_end_node(), node2);
        assert_eq!(edge.get_weight(), None);
        assert_eq!(edge.get_edge_type(), &EdgeType::Undirected);
    }
    #[test]
    fn test_edge_equality() {
        let node1 = Rc::new(Node::new());
        let node2 = Rc::new(Node::new());
        let edge1 = Edge::new(node1.clone(), node2.clone(), None, None);
        let edge2 = Edge::new(node1.clone(), node2.clone(), None, None);
        assert_eq!(edge1, edge2);

        let edge3 = Edge::new(node1.clone(), node2.clone(), None, Some(EdgeType::Directed));
        assert_ne!(edge1, edge3);

        let edge4 = Edge::new(node2.clone(), node1.clone(), None, None);
        assert_eq!(edge1, edge4);

        let edge5 = Edge::new(node2.clone(), node1.clone(), None, Some(EdgeType::Directed));
        assert_ne!(edge1, edge5);
    }
    #[test]
    fn test_edge_hashset() {
        let node1 = Rc::new(Node::new());
        let node2 = Rc::new(Node::new());
        let edge1 = Edge::new(node1.clone(), node2.clone(), None, None);
        let edge2 = Edge::new(node1.clone(), node2.clone(), None, None);
        let edge3 = Edge::new(node2.clone(), node1.clone(), None, None);

        let mut hashset = std::collections::HashSet::new();
        hashset.insert(edge1);
        hashset.insert(edge2);
        assert_eq!(hashset.len(), 1);

        hashset.insert(edge3);
        assert_eq!(hashset.len(), 1);
    }
}

use std::rc::Rc;
mod graph;
use graph::basics::Graph;
use graph::edge::{Edge, EdgeType};
use graph::graph_types::mathematical_graph::MathematicalGraph;
use graph::node::Node;

fn main() {
    let node1 = Rc::new(Node::new().set_name("Test".into()));
    let node2 = Rc::new(Node::new());
    let node3 = Rc::new(Node::new());
    let node4 = Rc::new(Node::new());
    let edge1 = Edge::new(Rc::clone(&node1), node2, None, None);
    let mut graph = MathematicalGraph::new();
    for _ in 0..10 {
        graph.add_node(Rc::new(Node::new()));
    }
    graph.add_edge(edge1);
    graph.add_edge(Edge::new(node3.clone(), node4.clone(), None, None));
    graph.add_edge(Edge::new(
        node1.clone(),
        node3.clone(),
        None,
        Some(EdgeType::Directed),
    ));

    println!("{:?}", graph);
}

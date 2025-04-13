use super::super::basics::Graph;
use super::super::node::Node;
use std::{collections::HashSet, rc::Rc};

#[derive(Debug, Default)]
pub struct AdjacencyMatrix(HashSet<Rc<Node>>, Vec<Vec<u64>>);

use super::super::basics::Graph;
use super::super::edge::Edge;
use std::collections::HashSet;

#[derive(Debug, Default)]
pub struct Edgelist(HashSet<Edge>);

use std::{hash::Hash, sync::atomic::AtomicUsize};
// use uuid::Uuid;

static GLOBAL_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Eq, Debug)]
pub struct Node {
    id: usize,
    name: Option<String>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: GLOBAL_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            name: None,
        }
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Hash for Node {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.id < other.id {
            Some(std::cmp::Ordering::Less)
        } else if self.id > other.id {
            Some(std::cmp::Ordering::Greater)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

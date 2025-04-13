use uuid::Uuid;

#[derive(Eq, Hash, PartialEq, Debug)]
pub struct Node {
    id: Uuid,
    name: Option<String>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            id: Uuid::new_v4(),
            name: None,
        }
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub description: Box<str>,
    pub completed: bool
}
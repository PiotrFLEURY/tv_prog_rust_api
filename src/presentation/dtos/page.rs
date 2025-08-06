use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Page<T> {
    pub content: Vec<T>,
}
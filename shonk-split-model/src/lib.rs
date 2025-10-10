use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    pub name: String,
    pub amount: u32,
    pub price: f32,
}

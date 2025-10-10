use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub name: String,
    pub amount: u32,
    pub price: f32,
}

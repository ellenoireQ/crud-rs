use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub stock: u32,
}

impl Product {
    pub fn new(id: u32, name: String, description: String, price: f64, stock: u32) -> Self {
        Self {
            id,
            name,
            description,
            price,
            stock,
        }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Product {
    name: String,
    description: String,
    
} 

impl Product {
    pub fn new() -> Self {
        Product {
            name: String::new(),
            description: String::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, x: String) {
        self.name = x;
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, x: String) {
        self.description = x;
    }
}

use mongodb::bson::{doc, Document};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Product {
    barcode: String,
    name: String,
    description: String,
    quantity: i64,
    category: String,
    image_url: String,
}

impl Product {
    pub fn get_doc(&self) -> Document {
        doc! {
             "$set": {
                 "barcode": &self.barcode,
                 "name": &self.name,
                 "description": &self.description,
                 "quantity": self.quantity,
                 "category": &self.category,
                 "image_url": &self.image_url,
             }
        }
    }
}

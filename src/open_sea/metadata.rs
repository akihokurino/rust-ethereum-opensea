use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub image: String,
    pub description: String,
}

impl Metadata {
    pub fn new(name: &str, image: &str, description: &str) -> Self {
        Self {
            name: name.to_string(),
            image: image.to_string(),
            description: description.to_string(),
        }
    }
}

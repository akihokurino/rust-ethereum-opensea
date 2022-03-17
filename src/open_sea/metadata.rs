use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub attributes: Vec<Attribute>,
    pub image: String,
    pub external_url: String,
}

impl Metadata {
    pub fn new(
        name: String,
        description: String,
        image_url: String,
        attrs: Vec<(String, String)>,
    ) -> Self {
        let attributes = attrs
            .iter()
            .map(|attr| Attribute {
                trait_type: attr.to_owned().0,
                display_type: "string".to_string(),
                value: attr.to_owned().1,
            })
            .collect();

        Self {
            name,
            description,
            attributes,
            image: image_url,
            external_url: "https://github.com/akihokurino/rust-opensea".to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
    pub trait_type: String,
    pub display_type: String,
    pub value: String,
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FridgeItem {
    pub number: u32,
    pub image_url: String,
    pub title: String,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm, Debug)]
pub struct ProductName {
    pub name: String,
}

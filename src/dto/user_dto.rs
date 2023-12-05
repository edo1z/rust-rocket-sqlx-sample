use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, FromForm, Debug)]
pub struct UserName {
    pub name: String,
}

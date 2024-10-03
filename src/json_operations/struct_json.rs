use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct StructJson {
    pub key_name_command: String,
    pub key_description_command: String,
}

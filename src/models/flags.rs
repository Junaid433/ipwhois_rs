use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Flag {
    #[serde(default)]
    pub img: String,
    #[serde(default)]
    pub emoji: String,
    #[serde(default)]
    pub emoji_unicode: String,
}

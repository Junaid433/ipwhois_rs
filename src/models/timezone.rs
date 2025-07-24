use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TimeZone {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub abbr: String,
    #[serde(default)]
    pub is_dst: bool,
    #[serde(default)]
    pub offset: i32,
    #[serde(default)]
    pub utc: String,
    #[serde(default)]
    pub current_time: String,
}

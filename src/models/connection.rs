use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Connection {
    pub asn: i32,
    #[serde(default)]
    pub org: String,
    #[serde(default)]
    pub isp: String,
    #[serde(default)]
    pub domain: String,
}

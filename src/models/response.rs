use serde::{Deserialize, Serialize};

use crate::models::{Flag, Connection, TimeZone};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct IpWhoIsResponse {
    pub ip: String,
    pub success: bool,
    #[serde(rename = "type", default)]
    pub ip_type: String,
    #[serde(default)]
    pub continent: String,
    #[serde(default)]
    pub continent_code: String,
    #[serde(default)]
    pub country: String,
    #[serde(default)]
    pub country_code: String,
    #[serde(default)]
    pub region: String,
    #[serde(default)]
    pub region_code: String,
    #[serde(default)]
    pub city: String,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(default)]
    pub is_eu: bool,
    #[serde(default)]
    pub postal: String,
    #[serde(default)]
    pub calling_code: String,
    #[serde(default)]
    pub capital: String,
    #[serde(default)]
    pub borders: String,
    pub flag: Flag,
    pub connection: Connection,
    pub timezone: TimeZone,
}

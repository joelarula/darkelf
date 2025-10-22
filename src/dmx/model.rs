// Add at the top of the file
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Fixture {
    pub name: String,
    pub short_name: String,
    pub categories: Vec<String>,
    pub meta: Meta,
    pub links: Links,
    pub manufacturer: String,
    pub channels: Vec<Channel>,
    pub modes: Vec<Mode>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    pub authors: Vec<String>,
    pub create_date: String,
    pub last_modify_date: String,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Links {
    pub manual: Option<String>,
    pub product_page: Option<String>,
    pub video: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Channel {
    pub name: String,
    pub default_value: u8,
    pub capabilities: Vec<Capability>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub dmx_range: [u8; 2],
    pub type_: String,
    pub menu_name: Option<String>,
    pub menu_value: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Mode {
    pub name: String,
    pub channels: Vec<String>,
}

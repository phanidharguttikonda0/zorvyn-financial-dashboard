use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Party {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub party_type: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}
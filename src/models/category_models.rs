use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Category {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub category_type: Option<String>,
    pub description: Option<String>,
}
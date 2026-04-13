use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeciesDto {
    pub id: u32,
    pub common_name: String,
    pub scientific_name: String,
    pub coefficient: f64,
    pub version: String,
}

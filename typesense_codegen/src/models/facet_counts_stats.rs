/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.23.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct FacetCountsStats {
    #[serde(rename = "max", skip_serializing_if = "Option::is_none")]
    pub max: Option<i32>,
    #[serde(rename = "min", skip_serializing_if = "Option::is_none")]
    pub min: Option<i32>,
    #[serde(rename = "sum", skip_serializing_if = "Option::is_none")]
    pub sum: Option<i32>,
    #[serde(rename = "total_values", skip_serializing_if = "Option::is_none")]
    pub total_values: Option<i32>,
    #[serde(rename = "avg", skip_serializing_if = "Option::is_none")]
    pub avg: Option<f32>,
}

impl FacetCountsStats {
    pub fn new() -> FacetCountsStats {
        FacetCountsStats {
            max: None,
            min: None,
            sum: None,
            total_values: None,
            avg: None,
        }
    }
}

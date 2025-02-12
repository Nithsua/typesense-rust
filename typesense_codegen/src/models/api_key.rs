/*
 * Typesense API
 *
 * An open source search engine for building delightful search experiences.
 *
 * The version of the OpenAPI document: 0.23.0
 *
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ApiKey {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actions")]
    pub actions: Vec<String>,
    #[serde(rename = "collections")]
    pub collections: Vec<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
    #[serde(rename = "id")]
    pub id: i64,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "value_prefix")]
    pub value_prefix: String,
}

impl ApiKey {
    pub fn new(
        actions: Vec<String>,
        collections: Vec<String>,
        id: i64,
        value: String,
        value_prefix: String,
    ) -> ApiKey {
        ApiKey {
            description: None,
            actions,
            collections,
            expires_at: None,
            id,
            value,
            value_prefix,
        }
    }
}

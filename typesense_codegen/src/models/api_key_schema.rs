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
pub struct ApiKeySchema {
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "actions")]
    pub actions: Vec<String>,
    #[serde(rename = "collections")]
    pub collections: Vec<String>,
    #[serde(rename = "expires_at", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,
}

impl ApiKeySchema {
    pub fn new(actions: Vec<String>, collections: Vec<String>) -> ApiKeySchema {
        ApiKeySchema {
            description: None,
            actions,
            collections,
            expires_at: None,
        }
    }
}

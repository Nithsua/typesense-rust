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
pub struct SearchHighlight {
    #[serde(rename = "field", skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// Present only for (non-array) string fields
    #[serde(rename = "snippet", skip_serializing_if = "Option::is_none")]
    pub snippet: Option<String>,
    /// Present only for (array) string[] fields
    #[serde(rename = "snippets", skip_serializing_if = "Option::is_none")]
    pub snippets: Option<Vec<String>>,
    /// The indices property will be present only for string[] fields and will contain the corresponding indices of the snippets in the search field
    #[serde(rename = "indices", skip_serializing_if = "Option::is_none")]
    pub indices: Option<Vec<i32>>,
    #[serde(rename = "matched_tokens", skip_serializing_if = "Option::is_none")]
    pub matched_tokens: Option<Vec<serde_json::Value>>,
}

impl SearchHighlight {
    pub fn new() -> SearchHighlight {
        SearchHighlight {
            field: None,
            snippet: None,
            snippets: None,
            indices: None,
            matched_tokens: None,
        }
    }
}



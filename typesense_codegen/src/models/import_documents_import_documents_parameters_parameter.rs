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
pub struct ImportDocumentsImportDocumentsParametersParameter {
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "batch_size", skip_serializing_if = "Option::is_none")]
    pub batch_size: Option<i32>,
    #[serde(rename = "dirty_values", skip_serializing_if = "Option::is_none")]
    pub dirty_values: Option<DirtyValues>,
}

impl ImportDocumentsImportDocumentsParametersParameter {
    pub fn new() -> ImportDocumentsImportDocumentsParametersParameter {
        ImportDocumentsImportDocumentsParametersParameter {
            action: None,
            batch_size: None,
            dirty_values: None,
        }
    }
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DirtyValues {
    #[serde(rename = "coerce_or_reject")]
    CoerceOrReject,
    #[serde(rename = "coerce_or_drop")]
    CoerceOrDrop,
    #[serde(rename = "drop")]
    Drop,
    #[serde(rename = "reject")]
    Reject,
}

impl Default for DirtyValues {
    fn default() -> DirtyValues {
        Self::CoerceOrReject
    }
}

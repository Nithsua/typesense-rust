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
pub struct CollectionSchema {
    /// Name of the collection
    #[serde(rename = "name")]
    pub name: String,
    /// A list of fields for querying, filtering and faceting
    #[serde(rename = "fields")]
    pub fields: Vec<crate::models::Field>,
    /// The name of an int32 / float field that determines the order in which the search results are ranked when a sort_by clause is not provided during searching. This field must indicate some kind of popularity.
    #[serde(
        rename = "default_sorting_field",
        skip_serializing_if = "Option::is_none"
    )]
    pub default_sorting_field: Option<String>,
    /// List of symbols or special characters to be used for  splitting the text into individual words in addition to space and new-line characters.
    #[serde(rename = "token_separators", skip_serializing_if = "Option::is_none")]
    pub token_separators: Option<Vec<String>>,
    /// List of symbols or special characters to be indexed.
    #[serde(rename = "symbols_to_index", skip_serializing_if = "Option::is_none")]
    pub symbols_to_index: Option<Vec<String>>,
}

impl CollectionSchema {
    pub fn new(name: String, fields: Vec<crate::models::Field>) -> CollectionSchema {
        CollectionSchema {
            name,
            fields,
            default_sorting_field: None,
            token_separators: None,
            symbols_to_index: None,
        }
    }
}

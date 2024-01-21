/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCorporationsCorporationIdIconsNotFound : No image server for this datasource

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCorporationsCorporationIdIconsNotFound {
    /// error message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetCorporationsCorporationIdIconsNotFound {
    /// No image server for this datasource
    pub fn new() -> GetCorporationsCorporationIdIconsNotFound {
        GetCorporationsCorporationIdIconsNotFound { error: None }
    }
}

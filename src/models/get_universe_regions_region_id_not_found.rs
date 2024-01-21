/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetUniverseRegionsRegionIdNotFound : Not found

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetUniverseRegionsRegionIdNotFound {
    /// Not found message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetUniverseRegionsRegionIdNotFound {
    /// Not found
    pub fn new() -> GetUniverseRegionsRegionIdNotFound {
        GetUniverseRegionsRegionIdNotFound { error: None }
    }
}

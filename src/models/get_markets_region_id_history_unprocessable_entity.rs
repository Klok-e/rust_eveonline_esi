/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetMarketsRegionIdHistoryUnprocessableEntity : Unprocessable entity

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMarketsRegionIdHistoryUnprocessableEntity {
    /// Unprocessable entity message
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl GetMarketsRegionIdHistoryUnprocessableEntity {
    /// Unprocessable entity
    pub fn new() -> GetMarketsRegionIdHistoryUnprocessableEntity {
        GetMarketsRegionIdHistoryUnprocessableEntity { error: None }
    }
}

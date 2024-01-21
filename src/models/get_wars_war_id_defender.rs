/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetWarsWarIdDefender : The defending corporation or alliance that declared this war, only contains either corporation_id or alliance_id

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetWarsWarIdDefender {
    /// Alliance ID if and only if the defender is an alliance
    #[serde(rename = "alliance_id", skip_serializing_if = "Option::is_none")]
    pub alliance_id: Option<i32>,
    /// Corporation ID if and only if the defender is a corporation
    #[serde(rename = "corporation_id", skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i32>,
    /// ISK value of ships the defender has killed
    #[serde(rename = "isk_destroyed")]
    pub isk_destroyed: f32,
    /// The number of ships the defender has killed
    #[serde(rename = "ships_killed")]
    pub ships_killed: i32,
}

impl GetWarsWarIdDefender {
    /// The defending corporation or alliance that declared this war, only contains either corporation_id or alliance_id
    pub fn new(isk_destroyed: f32, ships_killed: i32) -> GetWarsWarIdDefender {
        GetWarsWarIdDefender {
            alliance_id: None,
            corporation_id: None,
            isk_destroyed,
            ships_killed,
        }
    }
}

/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFwLeaderboardsOk {
    #[serde(rename = "kills")]
    pub kills: Box<crate::models::GetFwLeaderboardsKills>,
    #[serde(rename = "victory_points")]
    pub victory_points: Box<crate::models::GetFwLeaderboardsVictoryPoints>,
}

impl GetFwLeaderboardsOk {
    /// 200 ok object
    pub fn new(
        kills: crate::models::GetFwLeaderboardsKills,
        victory_points: crate::models::GetFwLeaderboardsVictoryPoints,
    ) -> GetFwLeaderboardsOk {
        GetFwLeaderboardsOk {
            kills: Box::new(kills),
            victory_points: Box::new(victory_points),
        }
    }
}

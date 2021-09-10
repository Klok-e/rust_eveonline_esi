/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCharactersOk : 200 ok object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCharactersOk {
    #[serde(rename = "kills")]
    pub kills: Box<crate::models::GetFwLeaderboardsCharactersKills>,
    #[serde(rename = "victory_points")]
    pub victory_points: Box<crate::models::GetFwLeaderboardsCharactersVictoryPoints>,
}

impl GetFwLeaderboardsCharactersOk {
    /// 200 ok object
    pub fn new(kills: crate::models::GetFwLeaderboardsCharactersKills, victory_points: crate::models::GetFwLeaderboardsCharactersVictoryPoints) -> GetFwLeaderboardsCharactersOk {
        GetFwLeaderboardsCharactersOk {
            kills: Box::new(kills),
            victory_points: Box::new(victory_points),
        }
    }
}


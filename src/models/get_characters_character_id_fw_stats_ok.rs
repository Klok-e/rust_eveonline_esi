/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdFwStatsOk : 200 ok object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdFwStatsOk {
    /// The given character's current faction rank
    #[serde(rename = "current_rank", skip_serializing_if = "Option::is_none")]
    pub current_rank: Option<i32>,
    /// The enlistment date of the given character into faction warfare. Will not be included if character is not enlisted in faction warfare
    #[serde(rename = "enlisted_on", skip_serializing_if = "Option::is_none")]
    pub enlisted_on: Option<String>,
    /// The faction the given character is enlisted to fight for. Will not be included if character is not enlisted in faction warfare
    #[serde(rename = "faction_id", skip_serializing_if = "Option::is_none")]
    pub faction_id: Option<i32>,
    /// The given character's highest faction rank achieved
    #[serde(rename = "highest_rank", skip_serializing_if = "Option::is_none")]
    pub highest_rank: Option<i32>,
    #[serde(rename = "kills")]
    pub kills: Box<crate::models::GetCharactersCharacterIdFwStatsKills>,
    #[serde(rename = "victory_points")]
    pub victory_points: Box<crate::models::GetCharactersCharacterIdFwStatsVictoryPoints>,
}

impl GetCharactersCharacterIdFwStatsOk {
    /// 200 ok object
    pub fn new(
        kills: crate::models::GetCharactersCharacterIdFwStatsKills,
        victory_points: crate::models::GetCharactersCharacterIdFwStatsVictoryPoints,
    ) -> GetCharactersCharacterIdFwStatsOk {
        GetCharactersCharacterIdFwStatsOk {
            current_rank: None,
            enlisted_on: None,
            faction_id: None,
            highest_rank: None,
            kills: Box::new(kills),
            victory_points: Box::new(victory_points),
        }
    }
}

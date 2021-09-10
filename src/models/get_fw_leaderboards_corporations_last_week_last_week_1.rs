/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetFwLeaderboardsCorporationsLastWeekLastWeek1 : last_week object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetFwLeaderboardsCorporationsLastWeekLastWeek1 {
    /// Amount of victory points
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<i32>,
    /// corporation_id integer
    #[serde(rename = "corporation_id", skip_serializing_if = "Option::is_none")]
    pub corporation_id: Option<i32>,
}

impl GetFwLeaderboardsCorporationsLastWeekLastWeek1 {
    /// last_week object
    pub fn new() -> GetFwLeaderboardsCorporationsLastWeekLastWeek1 {
        GetFwLeaderboardsCorporationsLastWeekLastWeek1 {
            amount: None,
            corporation_id: None,
        }
    }
}


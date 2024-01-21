/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdMining200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMining200Ok {
    /// date string
    #[serde(rename = "date")]
    pub date: String,
    /// quantity integer
    #[serde(rename = "quantity")]
    pub quantity: i64,
    /// solar_system_id integer
    #[serde(rename = "solar_system_id")]
    pub solar_system_id: i32,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetCharactersCharacterIdMining200Ok {
    /// 200 ok object
    pub fn new(
        date: String,
        quantity: i64,
        solar_system_id: i32,
        type_id: i32,
    ) -> GetCharactersCharacterIdMining200Ok {
        GetCharactersCharacterIdMining200Ok {
            date,
            quantity,
            solar_system_id,
            type_id,
        }
    }
}

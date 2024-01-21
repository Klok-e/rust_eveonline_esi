/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdPlanetsPlanetIdPin : pin object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdPin {
    /// contents array
    #[serde(rename = "contents", skip_serializing_if = "Option::is_none")]
    pub contents: Option<Vec<crate::models::GetCharactersCharacterIdPlanetsPlanetIdContent>>,
    /// expiry_time string
    #[serde(rename = "expiry_time", skip_serializing_if = "Option::is_none")]
    pub expiry_time: Option<String>,
    #[serde(rename = "extractor_details", skip_serializing_if = "Option::is_none")]
    pub extractor_details:
        Option<Box<crate::models::GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails>>,
    #[serde(rename = "factory_details", skip_serializing_if = "Option::is_none")]
    pub factory_details:
        Option<Box<crate::models::GetCharactersCharacterIdPlanetsPlanetIdFactoryDetails>>,
    /// install_time string
    #[serde(rename = "install_time", skip_serializing_if = "Option::is_none")]
    pub install_time: Option<String>,
    /// last_cycle_start string
    #[serde(rename = "last_cycle_start", skip_serializing_if = "Option::is_none")]
    pub last_cycle_start: Option<String>,
    /// latitude number
    #[serde(rename = "latitude")]
    pub latitude: f32,
    /// longitude number
    #[serde(rename = "longitude")]
    pub longitude: f32,
    /// pin_id integer
    #[serde(rename = "pin_id")]
    pub pin_id: i64,
    /// schematic_id integer
    #[serde(rename = "schematic_id", skip_serializing_if = "Option::is_none")]
    pub schematic_id: Option<i32>,
    /// type_id integer
    #[serde(rename = "type_id")]
    pub type_id: i32,
}

impl GetCharactersCharacterIdPlanetsPlanetIdPin {
    /// pin object
    pub fn new(
        latitude: f32,
        longitude: f32,
        pin_id: i64,
        type_id: i32,
    ) -> GetCharactersCharacterIdPlanetsPlanetIdPin {
        GetCharactersCharacterIdPlanetsPlanetIdPin {
            contents: None,
            expiry_time: None,
            extractor_details: None,
            factory_details: None,
            install_time: None,
            last_cycle_start: None,
            latitude,
            longitude,
            pin_id,
            schematic_id: None,
            type_id,
        }
    }
}

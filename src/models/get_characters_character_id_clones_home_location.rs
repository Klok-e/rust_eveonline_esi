/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdClonesHomeLocation : home_location object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdClonesHomeLocation {
    /// location_id integer
    #[serde(rename = "location_id", skip_serializing_if = "Option::is_none")]
    pub location_id: Option<i64>,
    /// location_type string
    #[serde(rename = "location_type", skip_serializing_if = "Option::is_none")]
    pub location_type: Option<LocationType>,
}

impl GetCharactersCharacterIdClonesHomeLocation {
    /// home_location object
    pub fn new() -> GetCharactersCharacterIdClonesHomeLocation {
        GetCharactersCharacterIdClonesHomeLocation {
            location_id: None,
            location_type: None,
        }
    }
}

/// location_type string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum LocationType {
    #[serde(rename = "station")]
    Station,
    #[serde(rename = "structure")]
    Structure,
}

impl Default for LocationType {
    fn default() -> LocationType {
        Self::Station
    }
}

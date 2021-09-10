/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails : extractor_details object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails {
    /// in seconds
    #[serde(rename = "cycle_time", skip_serializing_if = "Option::is_none")]
    pub cycle_time: Option<i32>,
    /// head_radius number
    #[serde(rename = "head_radius", skip_serializing_if = "Option::is_none")]
    pub head_radius: Option<f32>,
    /// heads array
    #[serde(rename = "heads")]
    pub heads: Vec<crate::models::GetCharactersCharacterIdPlanetsPlanetIdHead>,
    /// product_type_id integer
    #[serde(rename = "product_type_id", skip_serializing_if = "Option::is_none")]
    pub product_type_id: Option<i32>,
    /// qty_per_cycle integer
    #[serde(rename = "qty_per_cycle", skip_serializing_if = "Option::is_none")]
    pub qty_per_cycle: Option<i32>,
}

impl GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails {
    /// extractor_details object
    pub fn new(heads: Vec<crate::models::GetCharactersCharacterIdPlanetsPlanetIdHead>) -> GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails {
        GetCharactersCharacterIdPlanetsPlanetIdExtractorDetails {
            cycle_time: None,
            head_radius: None,
            heads,
            product_type_id: None,
            qty_per_cycle: None,
        }
    }
}


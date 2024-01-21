/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostCharactersCharacterIdFittingsFitting : fitting object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostCharactersCharacterIdFittingsFitting {
    /// description string
    #[serde(rename = "description")]
    pub description: String,
    /// items array
    #[serde(rename = "items")]
    pub items: Vec<crate::models::PostCharactersCharacterIdFittingsItem>,
    /// name string
    #[serde(rename = "name")]
    pub name: String,
    /// ship_type_id integer
    #[serde(rename = "ship_type_id")]
    pub ship_type_id: i32,
}

impl PostCharactersCharacterIdFittingsFitting {
    /// fitting object
    pub fn new(
        description: String,
        items: Vec<crate::models::PostCharactersCharacterIdFittingsItem>,
        name: String,
        ship_type_id: i32,
    ) -> PostCharactersCharacterIdFittingsFitting {
        PostCharactersCharacterIdFittingsFitting {
            description,
            items,
            name,
            ship_type_id,
        }
    }
}

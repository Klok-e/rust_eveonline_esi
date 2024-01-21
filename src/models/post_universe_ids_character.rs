/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// PostUniverseIdsCharacter : character object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PostUniverseIdsCharacter {
    /// id integer
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl PostUniverseIdsCharacter {
    /// character object
    pub fn new() -> PostUniverseIdsCharacter {
        PostUniverseIdsCharacter {
            id: None,
            name: None,
        }
    }
}

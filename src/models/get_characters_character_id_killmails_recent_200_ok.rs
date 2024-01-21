/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.19
 *
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdKillmailsRecent200Ok : 200 ok object

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdKillmailsRecent200Ok {
    /// A hash of this killmail
    #[serde(rename = "killmail_hash")]
    pub killmail_hash: String,
    /// ID of this killmail
    #[serde(rename = "killmail_id")]
    pub killmail_id: i32,
}

impl GetCharactersCharacterIdKillmailsRecent200Ok {
    /// 200 ok object
    pub fn new(
        killmail_hash: String,
        killmail_id: i32,
    ) -> GetCharactersCharacterIdKillmailsRecent200Ok {
        GetCharactersCharacterIdKillmailsRecent200Ok {
            killmail_hash,
            killmail_id,
        }
    }
}

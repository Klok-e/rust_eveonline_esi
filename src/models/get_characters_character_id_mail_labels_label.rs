/*
 * EVE Swagger Interface
 *
 * An OpenAPI for EVE Online
 *
 * The version of the OpenAPI document: 1.8.2
 * 
 * Generated by: https://openapi-generator.tech
 */

/// GetCharactersCharacterIdMailLabelsLabel : label object



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetCharactersCharacterIdMailLabelsLabel {
    /// color string
    #[serde(rename = "color", skip_serializing_if = "Option::is_none")]
    pub color: Option<Color>,
    /// label_id integer
    #[serde(rename = "label_id", skip_serializing_if = "Option::is_none")]
    pub label_id: Option<i32>,
    /// name string
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// unread_count integer
    #[serde(rename = "unread_count", skip_serializing_if = "Option::is_none")]
    pub unread_count: Option<i32>,
}

impl GetCharactersCharacterIdMailLabelsLabel {
    /// label object
    pub fn new() -> GetCharactersCharacterIdMailLabelsLabel {
        GetCharactersCharacterIdMailLabelsLabel {
            color: None,
            label_id: None,
            name: None,
            unread_count: None,
        }
    }
}

/// color string
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Color {
    #[serde(rename = "#0000fe")]
    _0000fe,
    #[serde(rename = "#006634")]
    _006634,
    #[serde(rename = "#0099ff")]
    _0099ff,
    #[serde(rename = "#00ff33")]
    _00ff33,
    #[serde(rename = "#01ffff")]
    _01ffff,
    #[serde(rename = "#349800")]
    _349800,
    #[serde(rename = "#660066")]
    _660066,
    #[serde(rename = "#666666")]
    _666666,
    #[serde(rename = "#999999")]
    _999999,
    #[serde(rename = "#99ffff")]
    _99ffff,
    #[serde(rename = "#9a0000")]
    _9a0000,
    #[serde(rename = "#ccff9a")]
    Ccff9a,
    #[serde(rename = "#e6e6e6")]
    E6e6e6,
    #[serde(rename = "#fe0000")]
    Fe0000,
    #[serde(rename = "#ff6600")]
    Ff6600,
    #[serde(rename = "#ffff01")]
    Ffff01,
    #[serde(rename = "#ffffcd")]
    Ffffcd,
    #[serde(rename = "#ffffff")]
    Ffffff,
}
